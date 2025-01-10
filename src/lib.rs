#![no_std]
extern crate alloc;

mod ports;
use ports::PortTrait;
#[cfg(feature = "c-library")]
pub mod c_api;
pub mod task_manager;
pub mod timer;
#[cfg(any(target_arch = "riscv32", target_arch = "xtensa"))]
#[cfg(feature = "network")]
use esp_wifi::esp_now::EspNow;
#[cfg(target_arch = "xtensa")]
use esp_hal::{gpio::*, peripherals::*};

/// Martos initialization. Should be called before using Martos functions.
pub fn init_system() {
    // Memory initialization.
    ports::Port::init_heap();
    // Hardware timer setup.
    ports::Port::setup_hardware_timer();
    #[cfg(feature = "network")]
    // Network setup.
    ports::Port::init_network();
}

#[cfg(any(target_arch = "riscv32", target_arch = "xtensa"))]
#[cfg(feature = "network")]
pub fn get_esp_now() -> EspNow<'static> {
    return ports::Port::get_esp_now();
}

#[cfg(target_arch = "xtensa")]
pub fn get_uart2() -> UART2 {
    return ports::Port::get_uart2();
}

#[cfg(target_arch = "xtensa")]
pub fn get_io() -> Io {
    return ports::Port::get_io();
}
