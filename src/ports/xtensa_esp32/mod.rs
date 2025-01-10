pub mod hardware_timer;
pub mod memory_manager;
#[cfg(feature = "network")]
pub mod network;
#[cfg(feature = "preemptive")]
mod preempt;

use crate::ports::PortTrait;
use esp_hal::gpio::Io;
use esp_hal::peripherals::UART2;
#[cfg(feature = "network")]
use esp_wifi::esp_now::EspNow;

// TODO: make it port just for esp32, not only for XtensaEsp32
/// PortTrait implementation for XtensaEsp32 platform
pub struct XtensaEsp32;
impl PortTrait for XtensaEsp32 {
    fn setup_hardware_timer() {
        hardware_timer::setup_hardware_timer();
    }

    fn valid_timer_index(_timer_index: u8) -> bool {
        true
    }

    fn try_acquire_timer(_timer_index: u8) -> bool {
        hardware_timer::try_acquire_timer()
    }

    fn start_hardware_timer(_timer_index: u8) {
        hardware_timer::start_hardware_timer();
    }

    fn set_reload_mode(_timer_index: u8, auto_reload: bool) {
        hardware_timer::set_reload_mode(auto_reload);
    }

    fn change_period_timer(_timer_index: u8, period: core::time::Duration) {
        hardware_timer::change_period_timer(period);
    }

    fn get_time(_timer_index: u8) -> core::time::Duration {
        hardware_timer::get_time()
    }

    fn stop_hardware_timer(_timer_index: u8) -> bool {
        false
    }

    fn release_hardware_timer(_timer_index: u8) {
        hardware_timer::release_hardware_timer()
    }

    fn init_heap() {
        memory_manager::init_heap();
    }

    #[cfg(target_arch = "xtensa")]
    fn get_uart2() -> UART2 {
        hardware_timer::get_uart2()
    }

    #[cfg(target_arch = "xtensa")]
    fn get_io() -> Io {
        hardware_timer::get_io()
    }

    #[cfg(feature = "network")]
    fn init_network() {
        network::init_network();
    }

    #[cfg(feature = "network")]
    fn get_esp_now() -> EspNow<'static> {
        network::get_esp_now()
    }

    #[cfg(feature = "preemptive")]
    fn setup_interrupt() {
        preempt::setup_interrupt();
    }
    #[cfg(feature = "preemptive")]
    fn setup_stack(thread: &mut crate::task_manager::preemptive::Thread) {
        preempt::setup_stack(thread);
    }
    #[cfg(feature = "preemptive")]
    fn save_ctx(thread_ctx: &mut TrapFrame, isr_ctx: &TrapFrame) {
        preempt::save_ctx(thread_ctx, isr_ctx)
    }
    #[cfg(feature = "preemptive")]
    fn load_ctx(thread_ctx: &TrapFrame, isr_ctx: &mut TrapFrame) {
        preempt::load_ctx(thread_ctx, isr_ctx)
    }
}

#[cfg(feature = "preemptive")]
pub type TrapFrame = esp_hal::trapframe::TrapFrame;
