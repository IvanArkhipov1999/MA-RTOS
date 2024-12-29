#![no_std]
#![no_main]

use core::sync::atomic::{AtomicU32, Ordering};
use esp_hal::entry;
use esp_println::println;
use martos::{
    init_system,
    task_manager::{TaskManager, TaskManagerTrait},
};

/// Counter to work with in loop.
static COUNTER: AtomicU32 = AtomicU32::new(1);

/// Setup function for the main task.
fn main_task_setup_fn() {
    println!("Main task setup.\n");
}

/// Loop function for the main task.
fn main_task_loop_fn() {
    let count = COUNTER.fetch_add(1, Ordering::Relaxed);
    println!("Main task loop: Counter = {}.\n", unsafe {
        COUNTER.as_ptr().read()
    });
    if count == 25 {
        TaskManager::add_task(
            inner_task_setup_fn,
            inner_task_loop_fn,
            inner_task_stop_condition_fn,
        );
    }
}

/// Stop condition for the main task.
fn main_task_stop_condition_fn() -> bool {
    false
}

/// Setup function for the inner task.
fn inner_task_setup_fn() {
    println!("Inner task setup.\n");
}

/// Loop function for the inner task.
fn inner_task_loop_fn() {
    COUNTER.fetch_add(1, Ordering::Relaxed);
    println!("Inner task loop: Counter = {}.\n", unsafe {
        COUNTER.as_ptr().read()
    });
}

/// Stop condition for the inner task.
fn inner_task_stop_condition_fn() -> bool {
    let value = unsafe { COUNTER.as_ptr().read() };
    value % 10 == 0
}

#[entry]
fn main() -> ! {
    // Initialize Martos.
    init_system();
    // Add task to execute.
    TaskManager::add_task(
        main_task_setup_fn,
        main_task_loop_fn,
        main_task_stop_condition_fn,
    );
    // Start task manager.
    TaskManager::start_task_manager();
}
