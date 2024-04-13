#![no_std]
#![no_main]

extern crate alloc;

use alloc::vec::Vec;
use core::sync::atomic::{AtomicU32, Ordering};
use esp32_hal::entry;
use esp_backtrace as _;
use esp_println::println;
use martos::task_manager::TaskManager;
use martos::init_system;

/// Counter to work with in loop.
static COUNTER: AtomicU32 = AtomicU32::new(1);
/// Vector to work with in loop.
static mut VEC: Vec<u32> = Vec::new();

/// Setup function for task to execute.
fn setup_fn() {
    println!("Setup hello world!")
}

/// Loop function for task to execute.
fn loop_fn() {
    COUNTER.fetch_add(1, Ordering::Relaxed);
    unsafe { VEC.push(COUNTER.as_ptr().read()); }
    println!("Loop hello world!");
    println!("Vector last value = {}", unsafe { VEC.last().unwrap() });
}

/// Stop condition function for task to execute.
fn stop_condition_fn() -> bool {
    let value = unsafe { COUNTER.as_ptr().read() };
    if value % 50 == 0 {
        return true;
    }
    return false;
}

#[entry]
fn main() -> ! {
    // Initialize Martos.
    init_system();
    // Add task to execute.
    TaskManager::add_task(setup_fn, loop_fn, stop_condition_fn);
    // Start task manager.
    TaskManager::start_task_manager();
}
