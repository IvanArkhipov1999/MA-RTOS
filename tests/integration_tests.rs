#[cfg(test)]
mod unit_tests {
    use martos::task_manager::TaskManager;
    use martos::timer::Timer;
    use sequential_test::sequential;
    use std::sync::atomic::{AtomicU32, Ordering};

    // TODO: refactor unit tests. They should check less. Separate tests for setup, loop and stop functions.
    // TODO: refactor unit tests. Task manager and timer tests should be in different files in one directory.

    #[test]
    #[sequential]
    /// Tests if task manager without tasks works during some time.
    fn test_empty_task_manager() {
        TaskManager::test_start_task_manager();
    }

    /// Counter for task for test_one_finite_task_task_manager.
    static TEST_ONE_FINITE_TASK_TASK_MANAGER_COUNTER: AtomicU32 = AtomicU32::new(1);
    /// Setup function for task for test_one_finite_task_task_manager.
    fn test_one_finite_task_task_manager_setup_fn() {}
    /// Loop function for task for test_one_finite_task_task_manager.
    fn test_one_finite_task_task_manager_loop_fn() {
        TEST_ONE_FINITE_TASK_TASK_MANAGER_COUNTER.fetch_add(1, Ordering::Relaxed);
    }
    /// Stop function for task for test_one_finite_task_task_manager.
    fn test_one_finite_task_task_manager_stop_condition_fn() -> bool {
        let value = unsafe { TEST_ONE_FINITE_TASK_TASK_MANAGER_COUNTER.as_ptr().read() };
        if value % 50 == 0 {
            return true;
        }
        false
    }
    #[test]
    #[sequential]
    /// Tests if task manager with one finite task works correctly during some time without panic.
    fn test_one_finite_task_task_manager() {
        TaskManager::add_task(
            test_one_finite_task_task_manager_setup_fn,
            test_one_finite_task_task_manager_loop_fn,
            test_one_finite_task_task_manager_stop_condition_fn,
        );
        TaskManager::test_start_task_manager();

        assert_eq!(
            unsafe { TEST_ONE_FINITE_TASK_TASK_MANAGER_COUNTER.as_ptr().read() },
            50
        );
    }

    /// Counter for task for test_one_infinite_task_task_manager.
    static TEST_ONE_INFINITE_TASK_TASK_MANAGER_COUNTER: AtomicU32 = AtomicU32::new(1);
    /// Setup function for task for test_one_infinite_task_task_manager.
    fn test_one_infinite_task_task_manager_setup_fn() {}
    /// Loop function for task for test_one_infinite_task_task_manager.
    fn test_one_infinite_task_task_manager_loop_fn() {
        TEST_ONE_INFINITE_TASK_TASK_MANAGER_COUNTER.fetch_add(1, Ordering::Relaxed);
    }
    /// Stop function for task for test_one_infinite_task_task_manager.
    fn test_one_infinite_task_task_manager_stop_condition_fn() -> bool {
        false
    }
    #[test]
    #[sequential]
    /// Tests if task manager with one infinite task works correctly during some time without panic.
    fn test_one_infinite_task_task_manager() {
        TaskManager::add_task(
            test_one_infinite_task_task_manager_setup_fn,
            test_one_infinite_task_task_manager_loop_fn,
            test_one_infinite_task_task_manager_stop_condition_fn,
        );
        TaskManager::test_start_task_manager();
    }

    /// Counter for task for test_two_finite_tasks_task_manager.
    static TEST_TWO_FINITE_TASK_TASK_MANAGER_COUNTER1: AtomicU32 = AtomicU32::new(1);
    /// Setup function for task for test_two_finite_tasks_task_manager.
    fn test_two_finite_tasks_task_manager_setup_fn1() {}
    /// Loop function for task for test_two_finite_tasks_task_manager.
    fn test_two_finite_tasks_task_manager_loop_fn1() {
        TEST_TWO_FINITE_TASK_TASK_MANAGER_COUNTER1.fetch_add(1, Ordering::Relaxed);
    }
    /// Stop function for task for test_two_finite_tasks_task_manager.
    fn test_two_finite_tasks_task_manager_stop_condition_fn1() -> bool {
        let value = unsafe { TEST_TWO_FINITE_TASK_TASK_MANAGER_COUNTER1.as_ptr().read() };
        if value % 50 == 0 {
            return true;
        }
        false
    }
    /// Counter for task for test_two_finite_tasks_task_manager.
    static TEST_TWO_FINITE_TASK_TASK_MANAGER_COUNTER2: AtomicU32 = AtomicU32::new(1);
    /// Setup function for task for test_two_finite_tasks_task_manager.
    fn test_two_finite_tasks_task_manager_setup_fn2() {}
    /// Loop function for task for test_two_finite_tasks_task_manager.
    fn test_two_finite_tasks_task_manager_loop_fn2() {
        TEST_TWO_FINITE_TASK_TASK_MANAGER_COUNTER2.fetch_add(1, Ordering::Relaxed);
    }
    /// Stop function for task for test_two_finite_tasks_task_manager.
    fn test_two_finite_tasks_task_manager_stop_condition_fn2() -> bool {
        let value = unsafe { TEST_TWO_FINITE_TASK_TASK_MANAGER_COUNTER2.as_ptr().read() };
        if value % 25 == 0 {
            return true;
        }
        false
    }
    #[test]
    #[sequential]
    /// Tests if task manager with two finite tasks works correctly during some time without panic.
    fn test_two_finite_tasks_task_manager() {
        TaskManager::add_task(
            test_two_finite_tasks_task_manager_setup_fn1,
            test_two_finite_tasks_task_manager_loop_fn1,
            test_two_finite_tasks_task_manager_stop_condition_fn1,
        );
        TaskManager::add_task(
            test_two_finite_tasks_task_manager_setup_fn2,
            test_two_finite_tasks_task_manager_loop_fn2,
            test_two_finite_tasks_task_manager_stop_condition_fn2,
        );
        TaskManager::test_start_task_manager();

        assert_eq!(
            unsafe { TEST_TWO_FINITE_TASK_TASK_MANAGER_COUNTER1.as_ptr().read() },
            50
        );
        assert_eq!(
            unsafe { TEST_TWO_FINITE_TASK_TASK_MANAGER_COUNTER2.as_ptr().read() },
            25
        );
    }

    /// Counter for task for test_two_different_tasks_task_manager.
    static TEST_TWO_DIFFERENT_TASK_TASK_MANAGER_COUNTER1: AtomicU32 = AtomicU32::new(1);
    /// Setup function for task for test_two_different_tasks_task_manager.
    fn test_two_different_tasks_task_manager_setup_fn1() {}
    /// Loop function for task for test_two_different_tasks_task_manager.
    fn test_two_different_tasks_task_manager_loop_fn1() {
        TEST_TWO_DIFFERENT_TASK_TASK_MANAGER_COUNTER1.fetch_add(1, Ordering::Relaxed);
    }
    /// Stop function for task for test_two_different_tasks_task_manager.
    fn test_two_different_tasks_task_manager_stop_condition_fn1() -> bool {
        let value = unsafe {
            TEST_TWO_DIFFERENT_TASK_TASK_MANAGER_COUNTER1
                .as_ptr()
                .read()
        };
        if value % 50 == 0 {
            return true;
        }
        false
    }
    /// Counter for task for test_two_different_tasks_task_manager.
    static TEST_TWO_DIFFERENT_TASK_TASK_MANAGER_COUNTER2: AtomicU32 = AtomicU32::new(1);
    /// Setup function for task for test_two_different_tasks_task_manager.
    fn test_two_different_tasks_task_manager_setup_fn2() {}
    /// Loop function for task for test_two_different_tasks_task_manager.
    fn test_two_different_tasks_task_manager_loop_fn2() {
        TEST_TWO_DIFFERENT_TASK_TASK_MANAGER_COUNTER2.fetch_add(1, Ordering::Relaxed);
    }
    /// Stop function for task for test_two_different_tasks_task_manager.
    fn test_two_different_tasks_task_manager_stop_condition_fn2() -> bool {
        false
    }
    #[test]
    #[sequential]
    /// Tests if task manager with two different (finite and infinite) tasks works correctly during some time without panic.
    fn test_two_different_tasks_task_manager() {
        TaskManager::add_task(
            test_two_different_tasks_task_manager_setup_fn1,
            test_two_different_tasks_task_manager_loop_fn1,
            test_two_different_tasks_task_manager_stop_condition_fn1,
        );
        TaskManager::add_task(
            test_two_different_tasks_task_manager_setup_fn2,
            test_two_different_tasks_task_manager_loop_fn2,
            test_two_different_tasks_task_manager_stop_condition_fn2,
        );
        TaskManager::test_start_task_manager();

        assert_eq!(
            unsafe {
                TEST_TWO_DIFFERENT_TASK_TASK_MANAGER_COUNTER1
                    .as_ptr()
                    .read()
            },
            50
        );
    }

    /// Counter for task for test_two_infinite_tasks_task_manager.
    static TEST_TWO_INFINITE_TASK_TASK_MANAGER_COUNTER1: AtomicU32 = AtomicU32::new(1);
    /// Setup function for task for test_two_infinite_tasks_task_manager.
    fn test_two_infinite_tasks_task_manager_setup_fn1() {}
    /// Loop function for task for test_two_infinite_tasks_task_manager.
    fn test_two_infinite_tasks_task_manager_loop_fn1() {
        TEST_TWO_INFINITE_TASK_TASK_MANAGER_COUNTER1.fetch_add(1, Ordering::Relaxed);
    }
    /// Stop function for task for test_two_infinite_tasks_task_manager.
    fn test_two_infinite_tasks_task_manager_stop_condition_fn1() -> bool {
        false
    }
    /// Counter for task for test_two_infinite_tasks_task_manager.
    static TEST_TWO_INFINITE_TASK_TASK_MANAGER_COUNTER2: AtomicU32 = AtomicU32::new(1);
    /// Setup function for task for test_two_infinite_tasks_task_manager.
    fn test_two_infinite_tasks_task_manager_setup_fn2() {}
    /// Loop function for task for test_two_infinite_tasks_task_manager.
    fn test_two_infinite_tasks_task_manager_loop_fn2() {
        TEST_TWO_INFINITE_TASK_TASK_MANAGER_COUNTER2.fetch_add(1, Ordering::Relaxed);
    }
    /// Stop function for task for test_two_infinite_tasks_task_manager.
    fn test_two_infinite_tasks_task_manager_stop_condition_fn2() -> bool {
        false
    }
    #[test]
    #[sequential]
    /// Tests if task manager with two infinite tasks works correctly during some without panic.
    fn test_two_infinite_tasks_task_manager() {
        TaskManager::add_task(
            test_two_infinite_tasks_task_manager_setup_fn1,
            test_two_infinite_tasks_task_manager_loop_fn1,
            test_two_infinite_tasks_task_manager_stop_condition_fn1,
        );
        TaskManager::add_task(
            test_two_infinite_tasks_task_manager_setup_fn2,
            test_two_infinite_tasks_task_manager_loop_fn2,
            test_two_infinite_tasks_task_manager_stop_condition_fn2,
        );
        TaskManager::test_start_task_manager();
    }

    /// Counter for task for test_setup_task_manager.
    static TEST_SETUP_TASK_MANAGER_COUNTER: AtomicU32 = AtomicU32::new(1);
    /// Setup function for task for test_setup_task_manager.
    fn test_setup_task_manager_setup_fn() {
        TEST_SETUP_TASK_MANAGER_COUNTER.store(42, Ordering::Relaxed);
    }
    /// Loop function for task for test_setup_task_manager.
    fn test_setup_task_manager_loop_fn() {}
    /// Stop function for task for test_setup_task_manager.
    fn test_setup_task_manager_stop_condition_fn() -> bool {
        false
    }
    #[test]
    #[sequential]
    /// Tests if task manager works correctly with setup function during some time without panic.
    fn test_setup_task_manager() {
        TaskManager::add_task(
            test_setup_task_manager_setup_fn,
            test_setup_task_manager_loop_fn,
            test_setup_task_manager_stop_condition_fn,
        );
        TaskManager::test_start_task_manager();

        assert_eq!(
            unsafe { TEST_SETUP_TASK_MANAGER_COUNTER.as_ptr().read() },
            42
        );
    }

    #[test]
    /// Tests setup timer function and getting tick counter (bad unit test).
    fn test_setup_timer() {
        Timer::setup_timer();
        assert_eq!(Timer::get_tick_counter(), 0);
    }

    #[test]
    /// Tests loop timer function.
    fn test_loop_timer() {
        Timer::setup_timer();
        Timer::loop_timer();
        assert_eq!(Timer::get_tick_counter(), 0);
    }

    #[test]
    /// Tests stop condition timer function.
    fn test_stop_condition_timer() {
        assert!(!Timer::stop_condition_timer());
    }
}
