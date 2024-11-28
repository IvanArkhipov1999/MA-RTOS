use crate::ports::xtensa_esp32::hardware_timer::*;
use crate::task_manager::preemptive::Thread;
use esp_hal::timer::timg::{Timer, Timer0, TimerGroup};
use esp_hal::trapframe::TrapFrame;
use esp_hal::xtensa_lx_rt;
use esp_hal::{
    interrupt::{self, InterruptHandler, Priority},
    prelude::*,
};
use esp_hal::{peripherals::*, prelude::*, Cpu};

const TIME_SLICE_MILLIS: u64 = 1000;

pub fn setup_interrupt() {
    let timer0 = unsafe { TIMER00.take().expect("Timer error") };
    timer0.set_interrupt_handler(InterruptHandler::new(
        unsafe { core::mem::transmute::<*const (), extern "C" fn()>(handler as *const ()) },
        Priority::Priority1,
    ));
    timer0.enable_interrupt(true);
    interrupt::enable(Interrupt::TG0_T0_LEVEL, Priority::Priority1).unwrap();

    timer0.load_value(TIME_SLICE_MILLIS.millis()).unwrap();
    timer0.start();
    timer0.listen();

    unsafe {
        TIMER00 = Some(timer0);
    };
}

extern "C" fn handler(ctx: &mut TrapFrame) {
    crate::task_manager::preemptive::PreemptiveTaskManager::schedule(ctx);

    let mut timer00 = unsafe { TIMER00.take().expect("Timer error") };
    timer00.clear_interrupt();
    timer00.load_value(TIME_SLICE_MILLIS.millis()).unwrap();
    timer00.start();
    unsafe {
        TIMER00 = Some(timer00);
    };
}

pub fn setup_stack(thread: &mut crate::task_manager::preemptive::Thread) {
    // manual 8.1
    thread.context.PC = Thread::run_task as u32;
    thread.context.A0 = 0; // return address

    thread.context.A6 = thread.task.setup_fn as u32; // A2 after `entry` instruction
    thread.context.A7 = thread.task.loop_fn as u32; // A3
    thread.context.A8 = thread.task.stop_condition_fn as u32; // A4

    let stack_ptr = thread.stack as usize + crate::task_manager::preemptive::THREAD_STACK_SIZE;
    thread.context.A1 = stack_ptr as u32;

    thread.context.PS = 0x00040000 | (1 & 3) << 16;
    unsafe {
        *((stack_ptr - 4) as *mut u32) = 0;
        *((stack_ptr - 8) as *mut u32) = 0;
        *((stack_ptr - 12) as *mut u32) = stack_ptr as u32;
        *((stack_ptr - 16) as *mut u32) = 0;
    }
}

pub fn save_ctx(thread_ctx: &mut TrapFrame, isr_ctx: &TrapFrame) {
    thread_ctx.clone_from(isr_ctx)
}

pub fn load_ctx(thread_ctx: &TrapFrame, isr_ctx: &mut TrapFrame) {
    isr_ctx.clone_from(thread_ctx)
}
