pub struct Timer {
    start_time: std::time::Instant,
    is_running: bool,
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            start_time: std::time::Instant::now(),
            is_running: false,
        }
    }

    pub fn start(&mut self) {
        self.start_time = std::time::Instant::now();
        self.is_running = true;
    }

    pub fn stop(&mut self) {
        self.is_running = false;
    }

    pub fn is_running(&self) -> bool {
        self.is_running
    }
}

#[cfg(test)]
mod timer_tests {
    use super::*;

    #[test]
    fn test_timer_start_stop() {
        let mut timer = Timer::new();
        timer.start();
        assert_eq!(timer.is_running(), true);
        timer.stop();
        assert_eq!(timer.is_running(), false);
    }
}