#![no_main]
#![no_std]

use rts as _; // global logger + panicking-behavior + memory layout

mod auxiliary;
mod parameters;
mod production_workload;
mod activation_manager;
mod request_buffer;
mod activation_log; 
mod event_queue;

#[rtic::app(
    device = stm32f4xx_hal::pac,
    peripherals = true,
    dispatchers = [USART1, USART2, USART3, USART6, UART5]
)]
mod app {
    use crate::parameters::parameters::*;
    use crate::parameters::parameters::act_log_reader::LOAD;
    use crate::activation_manager::activation_manager::*;
    use crate::production_workload::production_workload::WorkloadProd;
    use crate::auxiliary::auxiliary::Aux;
    use crate::request_buffer::request_buffer::RequestBuffer;
    use crate::activation_log::activation_log::ActivationLog;
    use crate::activation_log::reader::act_log_reader::ActLogReader;
    use crate::event_queue::event_queue::EventQueue;

    use rtic_monotonics::Monotonic;
    use rtic_sync::{channel::*, make_channel};

    use stm32f4xx_hal::{
        gpio::{Input, self, GpioExt, Edge, ExtiPin},
        prelude::*,
        pac::{Peripherals,EXTI},
    };

    // Shared resources go here
    #[shared]
    struct Shared {
        activation_manager : ActivationManager,
        request_buffer : RequestBuffer,
        activation_log : ActivationLog,
        act_log_reader : ActLogReader,
        event_queue : EventQueue,
    }

    // Local resources go here
    #[local]
    struct Local {
        regular_prod_work : WorkloadProd,
        on_call_prod_work : WorkloadProd,
        reader_prod_work : WorkloadProd,
        reg_aux : Aux,
        button : gpio::PA0<Input>,
        exti : EXTI,
        deposit_signal: Sender<'static, bool, 1>,
        reader_signal: Sender<'static, bool, 1>,
        extract_wait: Receiver<'static, bool, 1>,
        reader_wait: Receiver<'static, bool, 1>,
    }

    #[derive(Debug)]
    enum DeadlineConstant {
        START,
        END,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        defmt::info!("init");

        let dp: Peripherals = cx.device;

        let gpioa = dp.GPIOA.split();

        let mut _button = gpioa.pa0.into_pull_down_input();

        // Configure Button Pin for Interrupts
        // 1) Promote SYSCFG structure to HAL to be able to configure interrupts
        let mut syscfg = dp.SYSCFG.constrain();
        let mut _exti = dp.EXTI;
        // 2) Make button an interrupt source
        _button.make_interrupt_source(&mut syscfg);
        // 3) Make button an interrupt source
        _button.trigger_on_edge(&mut _exti, Edge::Rising);
        // 4) Enable gpio interrupt for button
        _button.enable_interrupt(&mut _exti);

        Mono::start(cx.core.SYST, 16_000_000);

        /* Following names are acronyms, eg: rps stands for regular producer sender */
        let (deposit_signal, extract_wait) = make_channel!(bool, 1);
        let (reader_signal, reader_wait) = make_channel!(bool, 1);

        regular_producer::spawn().unwrap();
        on_call_producer::spawn().unwrap();
        activation_log_reader::spawn().unwrap();
        external_event_server::spawn().unwrap();
        force_interrupt::spawn().unwrap();
                    
        (
            Shared {
                activation_manager: ActivationManager::new(),
                request_buffer: RequestBuffer::new(),
                activation_log: ActivationLog::new(),
                act_log_reader: ActLogReader::new(),
                event_queue: EventQueue::new(),
                //offset: _offset,
            },
            Local {
                regular_prod_work: WorkloadProd::new(),
                on_call_prod_work: WorkloadProd::new(),
                reader_prod_work: WorkloadProd::new(),
                reg_aux: Aux::new(),
                button: _button,
                exti: _exti,
                deposit_signal: deposit_signal,
                extract_wait: extract_wait,
                reader_signal: reader_signal,
                reader_wait: reader_wait,
            },
        )
    }

    async fn deadline_handler_job(start: Time, deadline: MyDuration, mut recv: Receiver<'static, DeadlineConstant, 1>, who: &str) {
        let finish: Time = start.checked_add_duration(deadline).unwrap();
        Mono::delay_until(finish).await;
        match recv.try_recv() { 
            Ok(_) => {}
            Err(_) => defmt::info!("{:?} Deadline missed!", who),
        }
    }

    #[task(priority = 16)]
    async fn deadline_handler_rp(
        _cx: deadline_handler_rp::Context, 
        start: Time, 
        deadline: MyDuration, 
        recv: Receiver<'static, DeadlineConstant, 1>
        ) {
        deadline_handler_job(start, deadline, recv, "RP").await;
    }
    
    #[task(priority = 16)]
    async fn deadline_handler_ocp(
        _cx: deadline_handler_ocp::Context, 
        start: Time, 
        deadline: MyDuration, 
        recv: Receiver<'static, DeadlineConstant, 1>
        ) {
        deadline_handler_job(start, deadline, recv, "OCP").await;
    }
    
    #[task(priority = 16)]
    async fn deadline_handler_alr(
        _cx: deadline_handler_alr::Context, 
        start: Time, 
        deadline: MyDuration, 
        recv: Receiver<'static, DeadlineConstant, 1>
        ) {
        deadline_handler_job(start, deadline, recv, "ALR").await;
    }

    #[task(priority = 16)]
    async fn deadline_handler_ees(
        _cx: deadline_handler_ees::Context, 
        start: Time, 
        deadline: MyDuration, 
        recv: Receiver<'static, DeadlineConstant, 1>
        ) {
        deadline_handler_job(start, deadline, recv, "EES").await;
    }

    #[task(priority = 7, shared = [&activation_manager, request_buffer, &act_log_reader], local = [regular_prod_work, reg_aux, deposit_signal, reader_signal])]
    async fn regular_producer(mut cx: regular_producer::Context) {
        let mut next_time : Time = cx.shared.activation_manager.activation_cyclic().await;
        defmt::info!("Activation cyclic");

        loop { 
            let (mut s, r) = make_channel!(DeadlineConstant, 1);
            deadline_handler_rp::spawn(Mono::now(), regular::get_deadline(), r).unwrap();

            next_time = next_time.checked_add_duration(regular::get_period()).unwrap();
            
            cx.local.regular_prod_work.small_whetstone(regular::REGULAR_PRODUCER_WORKLOAD);

            if cx.local.reg_aux.due_activation(regular::ACTIVATION_CONDITION) {
                let res : bool = cx.shared.request_buffer.lock(|shared| {
                    shared.deposit(regular::ON_CALL_PRODUCER_WORKLOAD)
                });
                if !res {
                    defmt::info!("Failed sporadic activation");
                } 
            }

            if cx.local.reg_aux.check_due() {
                let _ = cx.local.reader_signal.try_send(true);
            }
            
            s.send(DeadlineConstant::END).await.unwrap();
            
            Mono::delay_until(next_time).await;
         }
    }

    #[task(priority = 5, shared = [&activation_manager, request_buffer], local = [on_call_prod_work, extract_wait])]
    async fn on_call_producer(mut cx: on_call_producer::Context) {
        cx.shared.activation_manager.activation_sporadic().await;

        loop {
            let mut curr_workload : i32;
            let mut ok : bool;
            let (mut s, r) = make_channel!(DeadlineConstant, 1);

            loop {
                (curr_workload, ok) = cx.shared.request_buffer.lock(|shared| {
                    shared.extract()
                    }
                );
                if ok {
                    break;
                } else {
                    let delay : Time = delay_time();
                    Mono::delay_until(delay).await;
                }
            }

            deadline_handler_ocp::spawn(Mono::now(), on_call_prod::get_deadline(), r).unwrap();
            cx.local.on_call_prod_work.small_whetstone(curr_workload);
            s.send(DeadlineConstant::END).await.unwrap(); 
        } 
    }

    #[task(priority = 3, shared = [&activation_manager, &act_log_reader, activation_log], local = [reader_prod_work, reader_wait])]
    async fn activation_log_reader(mut cx: activation_log_reader::Context) {
        cx.shared.activation_manager.activation_sporadic().await;

        loop {
            let (mut s, r) = make_channel!(DeadlineConstant, 1);

            let _ = cx.local.reader_wait.recv().await;

            deadline_handler_alr::spawn(Mono::now(), on_call_prod::get_deadline(), r).unwrap();
            cx.local.reader_prod_work.small_whetstone(LOAD);
            let _ = cx.shared.activation_log.lock(|shared| {shared.read();});            
            s.send(DeadlineConstant::END).await.unwrap();
        }
    }

    #[task(priority = 11, shared = [&activation_manager, event_queue, activation_log])]
    async fn external_event_server(mut cx : external_event_server::Context) {
        cx.shared.activation_manager.activation_sporadic().await;
            
        loop {
            let (mut s, r) = make_channel!(DeadlineConstant, 1);
            
            loop {
                let ok : bool = cx.shared.event_queue.lock(|shared| {shared.wait()});  
                if ok {
                    defmt::info!("Checked succeeded!");
                    break;
                } else {
                    let delay : Time = delay_time();
                    Mono::delay_until(delay).await;
                }
            } 

            deadline_handler_ees::spawn(Mono::now(), on_call_prod::get_deadline(), r).unwrap();
            cx.shared.activation_log.lock(
                |shared| {
                    shared.write();
                }
            );
            s.send(DeadlineConstant::END).await.unwrap();
        }
        
    }

    #[task(binds = EXTI0, priority = 16, shared = [event_queue], local = [button])]
    fn interrupt(mut cx : interrupt::Context) {
        defmt::info!("Interrupt!!!!!!!!!!");
        cx.shared.event_queue.lock(|shared| {shared.signal()});
        // clear interrupt
        cx.local.button.clear_interrupt_pending_bit();
    }

    #[task(priority = 16, shared = [&activation_manager], local = [exti])]
    async fn force_interrupt(cx : force_interrupt::Context) {
        let mut next_time : Time = cx.shared.activation_manager.activation_cyclic().await;

        loop {
            next_time = next_time.checked_add_duration(force_inter::get_period()).unwrap();
            
            cx.local.exti.swier.write(|w| w.swier0().set_bit());

            Mono::delay_until(next_time).await;
         }
    }


}
