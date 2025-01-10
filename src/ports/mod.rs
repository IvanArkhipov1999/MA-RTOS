use core::time::Duration;
#[cfg(target_arch = "xtensa")]
use esp_hal::{gpio::*, peripherals::*};
#[cfg(any(target_arch = "riscv32", target_arch = "xtensa"))]
#[cfg(feature = "network")]
use esp_wifi::esp_now::EspNow;

/// PortTrait contains all the platform specific functions.
pub trait PortTrait {
    /// Function is called when timer is created. Can be used to set configuration.
    fn setup_hardware_timer();
    /// Function is used to check the correctness of index.
    fn valid_timer_index(timer_index: u8) -> bool;
    /// Function is called to attempt to acquire the timer.
    fn try_acquire_timer(timer_index: u8) -> bool;
    /// Function is called to start the timer.
    fn start_hardware_timer(timer_index: u8);
    /// Function is called to change the timer operating mode.
    fn set_reload_mode(timer_index: u8, auto_reload: bool);
    /// Function is called to change the period of the timer.
    fn change_period_timer(timer_index: u8, period: Duration);
    /// Function is called to get amount of time from the start of the timer.
    fn get_time(timer_index: u8) -> Duration;
    /// Function is called to stop the timer.
    fn stop_hardware_timer(timer_index: u8) -> bool;
    /// Function is called to release the timer.
    fn release_hardware_timer(timer_index: u8);

    /// Function is called when heap is created. Can be used to set configuration.
    fn init_heap();
    #[cfg(feature = "network")]
    /// Function for initializing network settings.
    fn init_network();
    #[cfg(target_arch = "xtensa")]
    /// Esp32 uart2.
    fn get_uart2() -> UART2;
    #[cfg(target_arch = "xtensa")]
    /// Esp32 io.
    fn get_io() -> Io;
    #[cfg(any(target_arch = "riscv32", target_arch = "xtensa"))]
    #[cfg(feature = "network")]
    /// Function for getting esp-now object for network.
    fn get_esp_now() -> EspNow<'static>;

    // TODO: split to separate trait?
    #[cfg(feature = "preemptive")]
    fn setup_interrupt();
    #[cfg(feature = "preemptive")]
    fn setup_stack(thread: &mut crate::task_manager::preemptive::Thread);
    #[cfg(feature = "preemptive")]
    fn save_ctx(thread_ctx: &mut TrapFrame, isr_ctx: &TrapFrame);
    #[cfg(feature = "preemptive")]
    fn load_ctx(thread_ctx: &TrapFrame, isr_ctx: &mut TrapFrame);
}

/// Port is an alias of PortTrait implementation for a current platform

#[cfg(any(target_arch = "riscv32", target_arch = "xtensa"))]
pub mod xtensa_esp32;
#[cfg(any(target_arch = "riscv32", target_arch = "xtensa"))]
mod arch {
    use super::xtensa_esp32;
    pub type Port = xtensa_esp32::XtensaEsp32;
    #[cfg(feature = "preemptive")]
    pub type TrapFrame = xtensa_esp32::TrapFrame;
    #[cfg(feature = "preemptive")]
    pub const STACK_ALIGN: usize = 16;
}

#[cfg(all(
    not(any(target_arch = "riscv32", target_arch = "xtensa")),
    not(target_arch = "mips64")
))]
pub mod mok;
#[cfg(all(
    not(any(target_arch = "riscv32", target_arch = "xtensa")),
    not(target_arch = "mips64")
))]
mod arch {
    use super::mok;
    pub type Port = mok::Mok;
    #[cfg(feature = "preemptive")]
    pub type TrapFrame = mok::TrapFrame;
    #[cfg(feature = "preemptive")]
    pub const STACK_ALIGN: usize = 0;
}

#[cfg(any(target_arch = "mips64", feature = "mips64_timer_tests"))]
pub mod mips64;
#[cfg(target_arch = "mips64")]
mod arch {
    use super::mips64;
    pub type Port = mips64::Mips64;
    #[cfg(feature = "preemptive")]
    pub type TrapFrame = ();
    #[cfg(feature = "preemptive")]
    pub const STACK_ALIGN: usize = 0;
}

pub use arch::*;
