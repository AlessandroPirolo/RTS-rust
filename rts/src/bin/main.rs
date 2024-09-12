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
    dispatchers = [USART1, USART2, USART3, USART6]
)]
mod app {
    use crate::parameters::parameters::*;
    use crate::parameters::parameters::activation_log_reader::LOAD;
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
        pac::Peripherals,
    };
    //use core::fmt::Write;

    /*systick_monotonic!(Mono, 1000);
    type Time = <Mono as rtic_monotonics::Monotonic>::Instant;
    type Duration = <Mono as rtic_monotonics::Monotonic>::Duration;*/

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
        button : gpio::PA0<Input>
    }

    

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        defmt::info!("init");

        // TODO setup monotonic if used
        // let sysclk = { /* clock setup + returning sysclk as an u32 */ };
        // let token = rtic_monotonics::create_systick_token!();
        // rtic_monotonics::systick::Systick::new(cx.core.SYST, sysclk, token);

        let mut dp: Peripherals = cx.device;

        //let gpioc = dp.GPIOC.split();
        let gpioa = dp.GPIOA.split();

        let mut _button = gpioa.pa0;

        // Configure Button Pin for Interrupts
        // 1) Promote SYSCFG structure to HAL to be able to configure interrupts
        let mut syscfg = dp.SYSCFG.constrain();

        // 2) Make button an interrupt source
        _button.make_interrupt_source(&mut syscfg);
        // 3) Make button an interrupt source
        _button.trigger_on_edge(&mut dp.EXTI, Edge::Rising);
        // 4) Enable gpio interrupt for button
        _button.enable_interrupt(&mut dp.EXTI);

        Mono::start(cx.core.SYST, 16_000_000);

        /*let _reloff : Duration = 1000.millis();
        let _offset : Time = Mono::now().checked_add_duration(_reloff).unwrap();*/

        regular_producer::spawn().unwrap();
        on_call_producer::spawn().unwrap();
        activation_log_reader::spawn().unwrap();
        external_event_server::spawn().unwrap();
            
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
                button: _button
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
            let per : MyDuration = regular::PERIOD.millis();
            next_time = next_time.checked_add_duration(per).unwrap();

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

            Mono::delay_until(next_time).await;
         }
    }

    #[task(priority = 5, shared = [&activation_manager, request_buffer], local = [on_call_prod_work])]
    async fn on_call_producer(mut cx: on_call_producer::Context) {
        cx.shared.activation_manager.activation_sporadic().await;

        loop {
            let (curr_workload, ok) : (i32, bool) = cx.shared.request_buffer.lock(|shared| {
                shared.extract()
            }
            );

            if ok {
                // defmt::info!("sporadic workload extracted");
                cx.local.on_call_prod_work.small_whetstone(curr_workload);
            } else {
                Mono::delay(100.millis()).await;
            }
        } 
    }

    #[task(priority = 3, shared = [&activation_manager, act_log_reader, activation_log], local = [reader_prod_work])]
    async fn activation_log_reader(mut cx: activation_log_reader::Context) { 
        cx.shared.activation_manager.activation_sporadic().await;

        loop {
            let ok : bool = cx.shared.act_log_reader.lock(|shared| {shared.wait()});
            if ok {
                // defmt::info!("Check succeeded in act log read");
                // defmt::info!("activation log reader");
                cx.local.reader_prod_work.small_whetstone(LOAD);
                let _ = cx.shared.activation_log.lock(|shared| {shared.read();});
            } else {
                Mono::delay(100.millis()).await;
            }
        }
    }

    #[task(priority = 11, shared = [&activation_manager, event_queue, activation_log])]
    async fn external_event_server(mut cx : external_event_server::Context) {
        cx.shared.activation_manager.activation_sporadic().await;
        defmt::info!("ext even serv");
            
        loop {
            let ok : bool = cx.shared.event_queue.lock(|shared| {shared.wait()});  
            if ok {
                // defmt::info!("Checked succeeded!");
                // defmt::info!("external event server");
                cx.shared.activation_log.lock(
                    |shared| {
                        shared.write();
                    }
                );
            } else {
                Mono::delay(100.millis()).await;
            }
        }
        
    }

    #[task(binds = EXTI15_10, local = [button], shared = [event_queue])]
    fn interrupt(mut cx : interrupt::Context) {
        defmt::info!("Button pressed");
        cx.shared.event_queue.lock(|shared| {shared.signal()});
        // clear interrupt
        cx.local.button.clear_interrupt_pending_bit();
    }


}
