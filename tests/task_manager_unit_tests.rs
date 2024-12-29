mod task_manager_unit_tests {
    use martos::task_manager::{TaskManager, TaskManagerTrait};
    use sequential_test::sequential;
    use std::sync::atomic::{AtomicU32, Ordering};

    fn dummy_setup_fn() {}
    fn dummy_loop_fn() {}
    fn dummy_condition_true() -> bool {
        true
    }
    fn dummy_condition_false() -> bool {
        false
    }

    #[test]
    #[sequential]
    fn test_reset_task_manager() {
        TaskManager::add_priority_task(dummy_setup_fn, dummy_loop_fn, dummy_condition_true, 3);
        TaskManager::add_priority_task(dummy_setup_fn, dummy_loop_fn, dummy_condition_true, 5);

        TaskManager::reset_task_manager();
        assert_eq!(TaskManager::count_all_tasks(), 0);
    }

    #[test]
    #[sequential]
    #[should_panic(
        expected = "Error: add_priority_task: Task's priority 100 is invalid. It must be between 0 and 11."
    )]
    fn test_add_task_invalid_priority() {
        TaskManager::reset_task_manager();
        TaskManager::add_priority_task(dummy_setup_fn, dummy_loop_fn, dummy_condition_true, 100);

        TaskManager::test_start_task_manager();
    }

    #[test]
    #[sequential]
    fn test_add_two_priority_tasks_and_check_vectors() {
        TaskManager::reset_task_manager();
        TaskManager::add_priority_task(dummy_setup_fn, dummy_loop_fn, dummy_condition_true, 0);
        TaskManager::add_priority_task(dummy_setup_fn, dummy_loop_fn, dummy_condition_true, 1);
        TaskManager::add_priority_task(dummy_setup_fn, dummy_loop_fn, dummy_condition_true, 0);

        assert_eq!(TaskManager::count_tasks_with_priority(0), 2);
        assert_eq!(TaskManager::count_tasks_with_priority(1), 1);
        assert_eq!(TaskManager::count_all_tasks(), 3);
    }

    #[test]
    #[sequential]
    fn test_add_task_and_check_if_priority_zero() {
        TaskManager::reset_task_manager();
        TaskManager::add_task(dummy_setup_fn, dummy_loop_fn, dummy_condition_true);
        assert_eq!(TaskManager::count_tasks_with_priority(0), 1);
        assert_eq!(TaskManager::count_all_tasks(), 1);
    }

    #[test]
    #[sequential]
    fn test_get_task_by_id() {
        TaskManager::reset_task_manager();
        TaskManager::add_priority_task(dummy_setup_fn, dummy_loop_fn, dummy_condition_true, 10);
        let id = TaskManager::get_id_by_position(10, 0);
        let found_task = TaskManager::get_task_by_id(id)
            .unwrap_or_else(|| panic!("Task not found for id {}", id));

        assert_eq!(id, TaskManager::get_id_from_task(found_task));
        TaskManager::reset_task_manager();
    }

    #[test]
    #[sequential]
    #[should_panic(expected = "Error: get_task_by_id: Task with id 2 not found.")]
    fn test_get_task_by_invalid_id() {
        TaskManager::reset_task_manager();
        TaskManager::add_priority_task(dummy_setup_fn, dummy_loop_fn, dummy_condition_true, 10);
        let found_task = TaskManager::get_task_by_id(2)
            .unwrap_or_else(|| panic!("Error: get_task_by_id: Task with id 2 not found."));
    }

    #[test]
    #[sequential]
    fn test_get_id_from_position() {
        TaskManager::reset_task_manager();
        // ID of a first added task is 1.
        TaskManager::add_priority_task(dummy_setup_fn, dummy_loop_fn, dummy_condition_true, 10);

        let task_id = TaskManager::get_id_by_position(10, 0);
        assert_eq!(task_id, 1);
        TaskManager::reset_task_manager();
    }

    #[test]
    #[sequential]
    #[should_panic(expected = "Error: get_id_by_position: No tasks found for task on position 1.")]
    fn test_get_id_from_invalid_position() {
        TaskManager::reset_task_manager();
        // id of first added task is 1.
        TaskManager::add_priority_task(dummy_setup_fn, dummy_loop_fn, dummy_condition_true, 10);
        let task_id = TaskManager::get_id_by_position(10, 1);
        TaskManager::reset_task_manager();
    }

    #[test]
    #[sequential]
    fn test_get_id_from_task() {
        TaskManager::reset_task_manager();
        TaskManager::add_task(dummy_setup_fn, dummy_loop_fn, dummy_condition_true);

        let task =
            TaskManager::get_task_by_id(1).unwrap_or_else(|| panic!("Task not found for id 1"));

        assert_eq!(TaskManager::get_id_from_task(task), 1);
        TaskManager::reset_task_manager();
    }

    /// Loop function for task for test_put_to_sleep_running_task_loop_fn.
    fn test_put_to_sleep_running_task_loop_fn() {
        TaskManager::put_to_sleep(1);
    }
    #[test]
    #[sequential]
    #[should_panic(expected = "Error: put_to_sleep: Task with id 1 is currently running.")]
    fn test_put_to_sleep_running_task() {
        TaskManager::reset_task_manager();
        TaskManager::add_task(
            dummy_setup_fn,
            test_put_to_sleep_running_task_loop_fn,
            dummy_condition_true,
        );

        assert_eq!(TaskManager::get_id_by_position(0, 0), 1);
        TaskManager::test_start_task_manager();
        TaskManager::reset_task_manager();
    }

    /// Loop function for task for test_put_to_sleep_sleeping_task_loop_fn.
    fn test_put_to_sleep_sleeping_task_loop_fn() {
        TaskManager::put_to_sleep(2);
    }
    #[test]
    #[sequential]
    #[should_panic(expected = "Error: put_to_sleep: Task with id 2 is currently sleeping.")]
    fn test_put_to_sleep_sleeping_task() {
        TaskManager::reset_task_manager();

        // Change a task state with id 2 to sleeping.
        TaskManager::add_task(
            dummy_setup_fn,
            test_put_to_sleep_sleeping_task_loop_fn,
            dummy_condition_true,
        );
        TaskManager::add_task(dummy_setup_fn, dummy_loop_fn, dummy_condition_true);

        let task_2 =
            TaskManager::get_task_by_id(2).unwrap_or_else(|| panic!("Task not found for id 2"));
        assert_eq!(TaskManager::get_status(task_2), TaskManager::ready_status());
        TaskManager::schedule();
        TaskManager::schedule();

        assert_eq!(
            TaskManager::get_status(task_2),
            TaskManager::sleeping_status()
        );

        TaskManager::add_task(
            dummy_setup_fn,
            test_put_to_sleep_sleeping_task_loop_fn,
            dummy_condition_true,
        );

        // Add a task that will put to a sleep sleeping task.
        TaskManager::test_start_task_manager();

        assert_eq!(TaskManager::count_all_tasks(), 1);
    }

    /// Loop function for task for test_put_to_sleep_task_from_task_loop_fn.
    fn test_put_to_sleep_task_from_task_loop_fn() {
        TaskManager::put_to_sleep(3);
    }
    #[test]
    #[sequential]
    fn test_put_to_sleep_task_from_task() {
        TaskManager::reset_task_manager();

        TaskManager::add_task(
            dummy_setup_fn,
            test_put_to_sleep_task_from_task_loop_fn,
            dummy_condition_true,
        );

        TaskManager::add_task(dummy_setup_fn, dummy_loop_fn, dummy_condition_true);
        let task_2 =
            TaskManager::get_task_by_id(2).unwrap_or_else(|| panic!("Task not found for id 2"));

        TaskManager::add_task(dummy_setup_fn, dummy_loop_fn, dummy_condition_true);
        let task_3 =
            TaskManager::get_task_by_id(3).unwrap_or_else(|| panic!("Task not found for id 3"));

        assert_eq!(TaskManager::get_status(task_3), TaskManager::ready_status());

        TaskManager::test_start_task_manager();

        assert_eq!(
            TaskManager::get_status(task_3),
            TaskManager::sleeping_status()
        );
        assert_eq!(TaskManager::count_all_tasks(), 1);
    }

    /// Loop functions for task for test_wake_up_sleeping_task.
    fn test_wake_up_sleeping_task_loop_fn() {
        TaskManager::wake_up_task(3);
    }
    fn test_put_to_sleep_task_loop_fn() {
        TaskManager::put_to_sleep(3);
    }
    #[test]
    #[sequential]
    fn test_wake_up_sleeping_task() {
        TaskManager::reset_task_manager();
        TaskManager::add_task(
            dummy_setup_fn,
            test_put_to_sleep_task_loop_fn,
            dummy_condition_true,
        );
        TaskManager::add_task(
            dummy_setup_fn,
            test_wake_up_sleeping_task_loop_fn,
            dummy_condition_true,
        );
        TaskManager::add_task(dummy_setup_fn, dummy_loop_fn, dummy_condition_true);

        let mut task_3 =
            TaskManager::get_task_by_id(3).unwrap_or_else(|| panic!("Task not found for id 3"));

        assert_eq!(TaskManager::get_status(task_3), TaskManager::ready_status());

        TaskManager::schedule();
        TaskManager::schedule();

        task_3 =
            TaskManager::get_task_by_id(3).unwrap_or_else(|| panic!("Task not found for id 3"));

        assert_eq!(
            TaskManager::get_status(task_3),
            TaskManager::sleeping_status()
        );

        TaskManager::schedule();
        TaskManager::schedule();
        TaskManager::schedule();
        task_3 =
            TaskManager::get_task_by_id(3).unwrap_or_else(|| panic!("Task not found for id 3"));

        assert_eq!(TaskManager::get_status(task_3), TaskManager::ready_status());

        TaskManager::test_start_task_manager();
        assert_eq!(TaskManager::count_all_tasks(), 0);
    }

    /// Loop functions for task for test_wake_up_non_sleeping_task_loop_fn.
    fn test_wake_up_non_sleeping_task_loop_fn() {
        TaskManager::wake_up_task(2);
    }
    #[test]
    #[sequential]
    #[should_panic(expected = "Error: wake_up_task: Task with id 2 is currently not sleeping.")]
    fn test_wake_up_non_sleeping_task() {
        TaskManager::reset_task_manager();
        TaskManager::add_task(
            dummy_setup_fn,
            test_wake_up_non_sleeping_task_loop_fn,
            dummy_condition_true,
        );
        TaskManager::add_task(dummy_setup_fn, dummy_loop_fn, dummy_condition_true);
        let task_2 =
            TaskManager::get_task_by_id(2).unwrap_or_else(|| panic!("Task not found for id 2"));

        assert_eq!(TaskManager::get_status(task_2), TaskManager::ready_status());
        TaskManager::test_start_task_manager();
    }

    /// Loop functions for task for test_terminate_non_terminated_task.
    fn test_terminate_non_terminated_task_loop_fn() {
        TaskManager::terminate_task(2);
    }
    fn infinite_loop_fn() {
        loop {}
    }
    #[test]
    #[sequential]
    fn test_terminate_non_terminated_task() {
        TaskManager::reset_task_manager();
        TaskManager::add_task(
            dummy_setup_fn,
            test_terminate_non_terminated_task_loop_fn,
            dummy_condition_true,
        );
        TaskManager::add_task(dummy_setup_fn, infinite_loop_fn, dummy_condition_true);
        let task_2 =
            TaskManager::get_task_by_id(2).unwrap_or_else(|| panic!("Task not found for id 2"));

        assert_eq!(TaskManager::get_status(task_2), TaskManager::ready_status());
        TaskManager::test_start_task_manager();
        assert_eq!(TaskManager::count_all_tasks(), 0);
    }

    #[test]
    #[sequential]
    /// Tests if task manager without tasks works during some time.
    fn test_empty_task_manager() {
        TaskManager::reset_task_manager();
        TaskManager::test_start_task_manager();
        assert!(TaskManager::is_empty());
    }

    /// Counter for a task for test_one_finite_task_task_manager.
    static TEST_ONE_FINITE_TASK_TASK_MANAGER_COUNTER: AtomicU32 = AtomicU32::new(1);
    /// Loop function for task for test_one_finite_task_task_manager.
    fn test_one_finite_task_task_manager_loop_fn() {
        TEST_ONE_FINITE_TASK_TASK_MANAGER_COUNTER.fetch_add(1, Ordering::Relaxed);
        // То есть состояния помогают только контролировать flow????
        // А по факту вся суть в состояниях... или наоборот, состояния формируют работу, а эти все функции для проверки чисто рудименты...
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
        TaskManager::reset_task_manager();
        TaskManager::add_task(
            dummy_setup_fn,
            test_one_finite_task_task_manager_loop_fn,
            test_one_finite_task_task_manager_stop_condition_fn,
        );
        TaskManager::test_start_task_manager();

        assert_eq!(
            unsafe { TEST_ONE_FINITE_TASK_TASK_MANAGER_COUNTER.as_ptr().read() },
            50
        );
    }

    /// Counter for a task for test_one_infinite_task_task_manager.
    static TEST_ONE_INFINITE_TASK_TASK_MANAGER_COUNTER: AtomicU32 = AtomicU32::new(1);
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
        TaskManager::reset_task_manager();
        TaskManager::add_task(
            dummy_setup_fn,
            test_one_infinite_task_task_manager_loop_fn,
            test_one_infinite_task_task_manager_stop_condition_fn,
        );
        TaskManager::test_start_task_manager();
    }

    /// Counter for a task for test_two_finite_tasks_task_manager.
    static TEST_TWO_FINITE_TASK_TASK_MANAGER_COUNTER1: AtomicU32 = AtomicU32::new(1);
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
    /// Counter for a task for test_two_finite_tasks_task_manager.
    static TEST_TWO_FINITE_TASK_TASK_MANAGER_COUNTER2: AtomicU32 = AtomicU32::new(1);
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
        TaskManager::reset_task_manager();
        TaskManager::add_task(
            dummy_setup_fn,
            test_two_finite_tasks_task_manager_loop_fn1,
            test_two_finite_tasks_task_manager_stop_condition_fn1,
        );
        TaskManager::add_task(
            dummy_setup_fn,
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

    /// Counter for a task for test_two_different_tasks_task_manager.
    static TEST_TWO_DIFFERENT_TASK_TASK_MANAGER_COUNTER1: AtomicU32 = AtomicU32::new(1);
    /// Loop function for task for test_two_different_tasks_task_manager.
    fn test_two_different_tasks_task_manager_loop_fn1() {
        TEST_TWO_DIFFERENT_TASK_TASK_MANAGER_COUNTER1.fetch_add(1, Ordering::Relaxed);
    }
    /// Stop function for a task for test_two_different_tasks_task_manager.
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
    /// Counter for a task for test_two_different_tasks_task_manager.
    static TEST_TWO_DIFFERENT_TASK_TASK_MANAGER_COUNTER2: AtomicU32 = AtomicU32::new(1);
    /// Loop function for task for test_two_different_tasks_task_manager.
    fn test_two_different_tasks_task_manager_loop_fn2() {
        TEST_TWO_DIFFERENT_TASK_TASK_MANAGER_COUNTER2.fetch_add(1, Ordering::Relaxed);
    }
    #[test]
    #[sequential]
    /// Tests if task manager with two different (finite and infinite) tasks works correctly during some time without panic.
    fn test_two_different_tasks_task_manager() {
        TaskManager::reset_task_manager();
        TaskManager::add_task(
            dummy_setup_fn,
            test_two_different_tasks_task_manager_loop_fn1,
            test_two_different_tasks_task_manager_stop_condition_fn1,
        );
        TaskManager::add_task(
            dummy_setup_fn,
            test_two_different_tasks_task_manager_loop_fn2,
            dummy_condition_false,
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

    /// Counter for a task for test_two_infinite_tasks_task_manager.
    static TEST_TWO_INFINITE_TASK_TASK_MANAGER_COUNTER1: AtomicU32 = AtomicU32::new(1);
    /// Loop function for task for test_two_infinite_tasks_task_manager.
    fn test_two_infinite_tasks_task_manager_loop_fn1() {
        TEST_TWO_INFINITE_TASK_TASK_MANAGER_COUNTER1.fetch_add(1, Ordering::Relaxed);
    }
    /// Counter for a task for test_two_infinite_tasks_task_manager.
    static TEST_TWO_INFINITE_TASK_TASK_MANAGER_COUNTER2: AtomicU32 = AtomicU32::new(1);
    /// Loop function for task for test_two_infinite_tasks_task_manager.
    fn test_two_infinite_tasks_task_manager_loop_fn2() {
        TEST_TWO_INFINITE_TASK_TASK_MANAGER_COUNTER2.fetch_add(1, Ordering::Relaxed);
    }
    #[test]
    #[sequential]
    /// Tests if task manager with two infinite tasks works correctly during some without panic.
    fn test_two_infinite_tasks_task_manager() {
        TaskManager::reset_task_manager();
        TaskManager::add_task(
            dummy_setup_fn,
            test_two_infinite_tasks_task_manager_loop_fn1,
            dummy_condition_false,
        );
        TaskManager::add_task(
            dummy_setup_fn,
            test_two_infinite_tasks_task_manager_loop_fn2,
            dummy_condition_false,
        );
        TaskManager::test_start_task_manager();

        assert_eq!(TaskManager::count_all_tasks(), 2);
    }

    /// Counter for a task for test_setup_task_manager.
    static TEST_SETUP_TASK_MANAGER_COUNTER: AtomicU32 = AtomicU32::new(1);
    /// Setup function for task for test_setup_task_manager.
    fn test_setup_task_manager_setup_fn() {
        TEST_SETUP_TASK_MANAGER_COUNTER.store(42, Ordering::Relaxed);
    }
    #[test]
    #[sequential]
    /// Tests if task manager works correctly with setup function during some time without panic.
    fn test_setup_task_manager() {
        TaskManager::reset_task_manager();
        TaskManager::add_task(
            test_setup_task_manager_setup_fn,
            dummy_loop_fn,
            dummy_condition_false,
        );
        TaskManager::test_start_task_manager();

        assert_eq!(
            unsafe { TEST_SETUP_TASK_MANAGER_COUNTER.as_ptr().read() },
            42
        );
    }
}
