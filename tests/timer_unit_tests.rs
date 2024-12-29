#[cfg(all(test, not(feature = "mips64_timer_tests")))]
mod timer_unit_tests {
    use martos::timer::Timer;
    use std::time::Duration;

    #[test]
    /// Tests setup timer function and getting counter value (bad unit test).
    fn test_setup_timer() {
        Timer::setup_timer();
        let timer = Timer::get_timer(0)
            .expect("The timer is already active or a timer with this index does not exist.");
        assert_eq!(timer.get_time().as_micros(), 0);
        timer.release_timer();
    }

    #[test]
    /// Tests loop timer function.
    fn test_loop_timer() {
        Timer::setup_timer();
        let mut timer = Timer::get_timer(0)
            .expect("The timer is already active or a timer with this index does not exist.");
        timer.loop_timer();
        assert_eq!(timer.get_time().as_micros(), 0);
        timer.release_timer();
    }

    #[test]
    /// Tests stop condition timer function.
    fn test_stop_condition_timer() {
        let timer = Timer::get_timer(0)
            .expect("The timer is already active or a timer with this index does not exist.");
        timer.change_period_timer(Duration::new(10, 0));
        timer.start_timer();
        assert!(!timer.stop_condition_timer());
        timer.release_timer();
    }
}
