use chrono::{NaiveDate, Utc};



pub struct DayMatrix {
    date: NaiveDate,
    tasks: Vec<Task>,
}
impl DayMatrix {
    pub fn default() -> DayMatrix {
        DayMatrix {
            date: Utc::now().date_naive(),
            tasks: Vec::new(),
        }
    }
    pub fn date(&self) -> &NaiveDate {
        &self.date
    }
    pub fn tasks(&self) -> &Vec<Task> {
        &self.tasks
    }
    pub fn task(&self, index: usize) -> &Task {
        &self.tasks[index]
    }
    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }
    pub fn sort_by_importance(&mut self) {
        self.tasks.sort_by(|a, b| a.compare_importance(b));
    }
    pub fn sort_by_urgency(&mut self) {
        self.tasks.sort_by(|a, b| a.compare_urgency(b));
    }
}
