#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]

use rts as _; // global logger + panicking-behavior + memory layout

/* teporary */
mod auxiliary;
mod parameters;
mod production_workload;
mod activation_manager;
mod request_buffer;
mod activation_log;

// TODO(7) Configure the `rtic::app` macro
#[rtic::app(
    // TODO: Replace `nrf52840_hal::pac` with the path to the PAC
    device = nrf52840_hal::pac,
    // TODO: Replace the `SWI0_EGU0` with free interrupt vectors if software tasks are used
    // You can usually find the names of the interrupt vectors in the nrf52840_hal::pac::interrupt enum.
    dispatchers = [SWI0_EGU0]
)]
mod app {
    // Shared resources go here
    #[shared]
    struct Shared {
        // TODO: Add resources
    }

    // Local resources go here
    #[local]
    struct Local {
        // TODO: Add resources
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        defmt::info!("init");

        // TODO setup monotonic if used
        // let sysclk = { /* clock setup + returning sysclk as an u32 */ };
        // let token = rtic_monotonics::create_systick_token!();
        // rtic_monotonics::systick::Systick::new(cx.core.SYST, sysclk, token);


        task1::spawn().ok();

        (
            Shared {
                // Initialization of shared resources go here
            },
            Local {
                // Initialization of local resources go here
            },
        )
    }

    // Optional idle, can be removed if not needed.
    #[idle]
    fn idle(_: idle::Context) -> ! {
        defmt::info!("idle");

        loop {
            continue;
        }
    }

    // TODO: Add tasks
    #[task(priority = 1)]
    async fn task1(_cx: task1::Context) {
        defmt::info!("Hello from task1!");
    }
}
