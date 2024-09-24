#![no_main]
#![no_std]
//#![feature(type_alias_impl_trait)]

use rts as _; // global logger + panicking-behavior + memory layout

/* teporary */
mod auxiliary;
mod parameters;
mod production_workload;
mod activation_manager;
mod request_buffer;
mod activation_log; 
mod event_queue;

#[rtic::app(
    // TODO: Replace `nrf52840_hal::pac` with the path to the PAC
    device = stm32f4xx_hal::pac,
    peripherals = true,
    // TODO: Replace the `SWI0_EGU0` with free interrupt vectors if software tasks are used
    // You can usually find the names of the interrupt vectors in the nrf52840_hal::pac::interrupt enum.
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
        //offset : Time,
    }

    // Local resources go here
    #[local]
    struct Local {
        regular_prod_work : WorkloadProd,
        on_call_prod_work : WorkloadProd,
        reader_prod_work : WorkloadProd,
        reg_aux : Aux,
        button : gpio::PA0<Input>,
        exti : EXTI 
    }

    

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        defmt::info!("init");

        let dp: Peripherals = cx.device;

        //let gpioc = dp.GPIOC.split();
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
            },
        )
    }

    // Optional idle, can be removed if not needed.
    /*#[idle]
    fn idle(_: idle::Context) -> ! {
        defmt::info!("idle");

        loop {
            //defmt::info!("keep idle");
            continue;
        }
    }*/

    #[task(priority = 7, shared = [&activation_manager, request_buffer, act_log_reader], local = [regular_prod_work, reg_aux])]
    async fn regular_producer(mut cx: regular_producer::Context) {
        let mut next_time : Time = cx.shared.activation_manager.activation_cyclic().await;
        defmt::info!("Activation cyclic");

        loop {
            next_time = next_time.checked_add_duration(regular::get_period()).unwrap();

            let deadline : Time = get_deadline(regular::get_deadline());

            cx.local.regular_prod_work.small_whetstone(regular::REGULAR_PRODUCER_WORKLOAD);

            if cx.local.reg_aux.due_activation(regular::ACTIVATION_CONDITION) {
                let res : bool = cx.shared.request_buffer.lock(|shared| {
                    shared.deposit(regular::ON_CALL_PRODUCER_WORKLOAD)
                });
                if !res {
                    defmt::info!("Faile sporadic activation");
                } 
            }

            if cx.local.reg_aux.check_due() {
                cx.shared.act_log_reader.lock(|shared|{shared.signal()});
            }

            check_deadline(deadline, "RP");
            
            Mono::delay_until(next_time).await;
         }
    }

    #[task(priority = 5, shared = [&activation_manager, request_buffer], local = [on_call_prod_work])]
    async fn on_call_producer(mut cx: on_call_producer::Context) {
        let mut next_time : Time = cx.shared.activation_manager.activation_sporadic().await;

        loop {
            next_time = next_time.checked_add_duration(on_call_prod::get_inter_arrival_time()).unwrap();

            let mut curr_workload : i32;
            let mut ok : bool;

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
            let deadline : Time = get_deadline(on_call_prod::get_deadline());
            cx.local.on_call_prod_work.small_whetstone(curr_workload);
            check_deadline(deadline, "OCP");
            Mono::delay_until(next_time).await;
        } 
    }

    #[task(priority = 3, shared = [&activation_manager, act_log_reader, activation_log], local = [reader_prod_work])]
    async fn activation_log_reader(mut cx: activation_log_reader::Context) { 
        let mut next_time : Time = cx.shared.activation_manager.activation_sporadic().await;

        loop {
            next_time = next_time.checked_add_duration(act_log_reader::get_inter_arrival_time()).unwrap();

            loop {
                let ok : bool = cx.shared.act_log_reader.lock(|shared| {shared.wait()});
                if ok {
                    //defmt::info!("Check succeeded in act log read");
                    break;
                }
            }
            let deadline : Time = get_deadline(act_log_reader::get_deadline());
            cx.local.reader_prod_work.small_whetstone(LOAD);
            let _ = cx.shared.activation_log.lock(|shared| {shared.read();});
            check_deadline(deadline, "ALR");
            Mono::delay_until(next_time).await;
        }
    }

    #[task(priority = 11, shared = [&activation_manager, event_queue, activation_log])]
    async fn external_event_server(mut cx : external_event_server::Context) {
        let mut next_time : Time = cx.shared.activation_manager.activation_sporadic().await;
            
        loop {
            next_time = next_time.checked_add_duration(ext_event_serv::get_inter_arrival_time()).unwrap();

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
            let deadline : Time = get_deadline(ext_event_serv::get_deadline());
            cx.shared.activation_log.lock(
                |shared| {
                    shared.write();
                }
            );
            check_deadline(deadline, "EES"); 
            Mono::delay_until(next_time).await;
        }
        
    }

    #[task(binds = EXTI0, shared = [event_queue], local = [button])]
    fn interrupt(mut cx : interrupt::Context) {
        defmt::info!("Button pressed!!!!!!!!!!");
        cx.shared.event_queue.lock(|shared| {shared.signal()});
        // clear interrupt
        cx.local.button.clear_interrupt_pending_bit();
    }

    #[task(priority = 13, shared = [&activation_manager], local = [exti])]
    async fn force_interrupt(cx : force_interrupt::Context) {
        let mut next_time : Time = cx.shared.activation_manager.activation_cyclic().await;

        loop {
            //defmt::info!("Force interrupt");
            next_time = next_time.checked_add_duration(force_inter::get_period()).unwrap();
            //cx.local.exti.swier.reset();
            //let deadline : Time = get_deadline(regular::get_deadline());
            
            cx.local.exti.swier.write(|w| w.swier0().set_bit());

            //check_deadline(deadline);
            
            Mono::delay_until(next_time).await;
         }
    }


}
