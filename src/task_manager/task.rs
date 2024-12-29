// TODO: rewrite with cfg!
#[cfg(not(feature = "c-library"))]
/// Type of setup function, that is called once at the beginning of task.
pub type TaskSetupFunctionType = fn() -> ();
#[cfg(feature = "c-library")]
/// Type of setup function, that is called once at the beginning of task.
pub type TaskSetupFunctionType = extern "C" fn() -> ();
#[cfg(not(feature = "c-library"))]
/// Type of loop function, that is called in loop.
pub type TaskLoopFunctionType = fn() -> ();
#[cfg(feature = "c-library")]
/// Type of loop function, that is called in loop.
pub type TaskLoopFunctionType = extern "C" fn() -> ();
#[cfg(not(feature = "c-library"))]
/// Type of condition function for stopping loop function execution.
pub type TaskStopConditionFunctionType = fn() -> bool;
#[cfg(feature = "c-library")]
/// Type of condition function for stopping loop function execution.
pub type TaskStopConditionFunctionType = extern "C" fn() -> bool;

#[repr(C)]
/// Task representation for task manager.
#[derive(Clone)]
pub struct Task {
    /// Setup function, that is called once at the beginning of task.
    pub(crate) setup_fn: TaskSetupFunctionType,
    /// Loop function, that is called in loop.
    pub(crate) loop_fn: TaskLoopFunctionType,
    /// Condition function for stopping loop function execution.
    pub(crate) stop_condition_fn: TaskStopConditionFunctionType,
}
