extern crate alloc;

use crate::task_manager::task::{
    TaskLoopFunctionType, TaskSetupFunctionType, TaskStopConditionFunctionType,
};

mod task;

cfg_if::cfg_if! {
    if #[cfg(feature = "preemptive")] {
        pub(crate) mod preemptive;
        pub type TaskManager = preemptive::PreemptiveTaskManager;
    } else {
        mod cooperative;
        pub type TaskManager = cooperative::CooperativeTaskManager;
    }
}

/// Operating system task manager.
/// By default, [cooperative::CooperativeTaskManager] is used
static mut TASK_MANAGER: TaskManager = TaskManager::new();

pub trait TaskManagerTrait {
    /// Add task to task manager. You should pass setup, loop and condition functions.
    fn add_task(
        setup_fn: TaskSetupFunctionType,
        loop_fn: TaskLoopFunctionType,
        stop_condition_fn: TaskStopConditionFunctionType,
    );

    /// Starts task manager work.
    fn start_task_manager() -> !;
}
