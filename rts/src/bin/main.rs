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
mod timing_event;

#[rtic::app(
    device = stm32f4xx_hal::pac,
    peripherals = true,
    dispatchers = [USART1, USART2, USART3, USART6, UART5, UART4]
)]
mod app {
    use crate::parameters::parameters::*;
    use crate::parameters::parameters::act_log_reader::LOAD;
    use crate::activation_manager::activation_manager::*;
    use crate::production_workload::production_workload::WorkloadProd;
    use crate::auxiliary::auxiliary::Aux;
    use crate::request_buffer::request_buffer::RequestBuffer;
    use crate::activation_log::activation_log::ActivationLog;
    use crate::event_queue::event_queue::EventQueue;
    use crate::timing_event::timing_event::TimingEvent;
    
    use rtic_monotonics::Monotonic;
    use embassy_sync::channel::*;
    use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;

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
        event_queue : EventQueue,
        rp_deadline_event: TimingEvent,
        ocp_deadline_event: TimingEvent,
        alr_deadline_event: TimingEvent,
        ees_deadline_event: TimingEvent,
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
        deposit_signal: Sender<'static, CriticalSectionRawMutex, u32, 1>,
        reader_signal: Sender<'static, CriticalSectionRawMutex, u32, 1>,
        queue_signal: Sender<'static, CriticalSectionRawMutex, u32, 1>,
        extract_wait: Receiver<'static, CriticalSectionRawMutex, u32, 1>,
        reader_wait: Receiver<'static, CriticalSectionRawMutex, u32, 1>,
        queue_wait: Receiver<'static, CriticalSectionRawMutex, u32, 1>,
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

        static BUFFER_CHANNEL: Channel<CriticalSectionRawMutex, u32, 1> = Channel::new();
        static READER_CHANNEL: Channel<CriticalSectionRawMutex, u32, 1> = Channel::new();
        static EVENT_CHANNEL: Channel<CriticalSectionRawMutex, u32, 1> = Channel::new();

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
                event_queue: EventQueue::new(),
                rp_deadline_event: TimingEvent::new(),
                ocp_deadline_event: TimingEvent::new(),
                alr_deadline_event: TimingEvent::new(),
                ees_deadline_event: TimingEvent::new(),
            },
            Local {
                regular_prod_work: WorkloadProd::new(),
                on_call_prod_work: WorkloadProd::new(),
                reader_prod_work: WorkloadProd::new(),
                reg_aux: Aux::new(),
                button: _button,
                exti: _exti,
                deposit_signal: BUFFER_CHANNEL.sender(),
                extract_wait: BUFFER_CHANNEL.receiver(),
                reader_signal: READER_CHANNEL.sender(),
                reader_wait: READER_CHANNEL.receiver(),
                queue_signal: EVENT_CHANNEL.sender(),
                queue_wait: EVENT_CHANNEL.receiver(),
            },
        )
    }

    async fn deadline_handler_job(
        start: Time, 
        deadline: MyDuration, 
        event: &TimingEvent, 
        who: &str
        ) {
        let finish: Time = 
            start.checked_add_duration(deadline).unwrap();
        event.set().await;
        Mono::delay_until(finish).await;
        match event.check().await { 
            true => {},
            false => defmt::info!("{:?} Deadline missed!", who),
        }
    }

    #[task(priority = 16, shared = [&rp_deadline_event])]
    async fn deadline_handler_rp(
        cx: deadline_handler_rp::Context, 
    ) {
        /*loop {
            let _ = cx.local.rp_deadline_recv.receive().await;
            let start: Time = Mono::now();
            defmt::info!("here1");
            let finish: Time = 
                start.checked_add_duration(regular::get_deadline()).unwrap();
            defmt::info!("here2");
            cx.shared.rp_deadline_event.set().await;
            defmt::info!("here3");
            Mono::delay_until(finish).await;
            defmt::info!("here4");
            match cx.shared.rp_deadline_event.check().await { 
                true => defmt::info!("Received RP"),
                false => defmt::info!("RP Deadline missed!"),
            }
            defmt::info!("here5");

        }*/
        deadline_handler_job(
            Mono::now(), 
            regular::get_deadline(), 
            cx.shared.rp_deadline_event, 
            "RP").await;
    }
    
    #[task(priority = 16, shared = [&ocp_deadline_event])]
    async fn deadline_handler_ocp(
        cx: deadline_handler_ocp::Context, 
        start: Time, 
    ) {
        deadline_handler_job(
            start, 
            on_call_prod::get_deadline(), 
            cx.shared.ocp_deadline_event, 
            "OCP").await;
    }
    
    #[task(priority = 16, shared = [&alr_deadline_event])]
    async fn deadline_handler_alr(
        cx: deadline_handler_alr::Context, 
        start: Time, 
    ) {
        deadline_handler_job(
            start, 
            act_log_reader::get_deadline(), 
            cx.shared.alr_deadline_event, 
            "ALR").await;
    }

    #[task(priority = 16, shared = [&ees_deadline_event])]
    async fn deadline_handler_ees(
        cx: deadline_handler_ees::Context, 
        start: Time, 
    ) {
        deadline_handler_job(
            start, 
            ext_event_serv::get_deadline(), 
            cx.shared.ees_deadline_event, 
            "EES").await;
    }

    #[task(priority = 7, 
        shared = [&activation_manager, request_buffer, &rp_deadline_event], 
        local = [regular_prod_work, reg_aux, deposit_signal, reader_signal])]
    async fn regular_producer(mut cx: regular_producer::Context) {
        let mut next_time : Time = cx.shared.activation_manager.activation_cyclic().await;
        defmt::info!("Activation cyclic");

        loop { 
            deadline_handler_rp::spawn().unwrap();

            next_time = next_time.checked_add_duration(regular::get_period()).unwrap();
            
            cx.local.regular_prod_work.small_whetstone(regular::REGULAR_PRODUCER_WORKLOAD);

            if cx.local.reg_aux.due_activation(regular::ACTIVATION_CONDITION) {
                let res : bool = cx.shared.request_buffer.lock(|shared| {
                    shared.deposit(regular::ON_CALL_PRODUCER_WORKLOAD, cx.local.deposit_signal.clone())
                });
                if !res {
                    defmt::info!("Failed sporadic activation");
                } 
            }

            if cx.local.reg_aux.check_due() {
                let _ = cx.local.reader_signal.try_send(1);
            }

            cx.shared.rp_deadline_event.cancel().await; 
            Mono::delay_until(next_time).await;
         }
    }

    #[task(priority = 5, shared = [&activation_manager, request_buffer, &ocp_deadline_event], local = [on_call_prod_work, extract_wait])]
    async fn on_call_producer(mut cx: on_call_producer::Context) {
        cx.shared.activation_manager.activation_sporadic().await;

        loop {
            let _ = cx.local.extract_wait.receive().await;
            let curr_workload : i32 = cx.shared.request_buffer.lock(|shared| {
                shared.extract()
                }
            );

            deadline_handler_ocp::spawn(Mono::now()).unwrap();
            cx.local.on_call_prod_work.small_whetstone(curr_workload);
             
            cx.shared.ocp_deadline_event.cancel().await; 
        } 
    }

    #[task(priority = 3, shared = [&activation_manager, activation_log, &alr_deadline_event], local = [reader_prod_work, reader_wait])]
    async fn activation_log_reader(mut cx: activation_log_reader::Context) {
        cx.shared.activation_manager.activation_sporadic().await;

        loop {
            let _ = cx.local.reader_wait.receive().await;

            deadline_handler_alr::spawn(Mono::now()).unwrap();
            cx.local.reader_prod_work.small_whetstone(LOAD);
            let _ = cx.shared.activation_log.lock(|shared| {shared.read();});            

            cx.shared.alr_deadline_event.cancel().await; 
        }
    }

    #[task(priority = 11, shared = [&activation_manager, event_queue, activation_log, &ees_deadline_event], local = [queue_wait])]
    async fn external_event_server(mut cx : external_event_server::Context) {
        cx.shared.activation_manager.activation_sporadic().await;
            
        loop {
            let _ = cx.local.queue_wait.receive().await;
            cx.shared.event_queue.lock(|shared| {shared.wait()});   

            deadline_handler_ees::spawn(Mono::now()).unwrap();
            cx.shared.activation_log.lock(
                |shared| {
                    shared.write();
                }
            );
            
            cx.shared.ees_deadline_event.cancel().await; 
        }
        
    }

    #[task(binds = EXTI0, priority = 16, shared = [event_queue], local = [button, queue_signal])]
    fn interrupt(mut cx : interrupt::Context) {
        defmt::info!("Interrupt!!!!!!!!!!");
        cx.shared.event_queue.lock(|shared| {shared.signal(cx.local.queue_signal.clone())});
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
