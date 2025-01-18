struct TaskDispatcher {
    tasks: Vec<String>,
    is_running: bool
}

impl TaskDispatcher {
    fn new() -> Self {
        TaskDispatcher { tasks: Vec::new(), is_running: true,
         }
    }

    fn add_task(&mut self, task: String) {
        self.tasks.push(task);
    }

    fn run_tasks(&self) {
        for task in &self.tasks {
            println!("Running task: {}", task);
        }
    }

    fn is_running(&self) -> bool{
        self.is_running
    }

    fn start(&mut self) {
        self.is_running = true;
    }

    fn stop(&mut self) {
        self.is_running = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_dispatcher() {
        let mut dispatcher = TaskDispatcher::new();
        dispatcher.add_task(String::from("Task 1"));
        dispatcher.add_task(String::from("Task 2"));
        dispatcher.add_task(String::from("Task 3"));
        dispatcher.run_tasks();
        assert_eq!(dispatcher.tasks.len(), 3);
    }

    #[test]
    fn test_timer_start_stop() {
        let mut taskdispatcher = TaskDispatcher::new();
        taskdispatcher.start();
        assert_eq!(taskdispatcher.is_running(), true);
        taskdispatcher.stop();
        assert_eq!(taskdispatcher.is_running(), false);
    }
}
