use chrono::{NaiveDate, Utc};
pub struct Matrix {
    date: NaiveDate,
    tasks: Vec<Task>
}
impl Matrix {
    pub fn empty() -> Matrix {
        Matrix { date:Utc::now().date_naive(), tasks: Vec::new() }
    }
    pub fn date(&self) -> String {
        self.date.to_string()
    }
    pub fn tasks(&self) -> &Vec<Task> {
        &self.tasks
    }
    pub fn task(&self, index:usize) -> &Task {
        &self.tasks[index]
    }
    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }
    pub fn sort_by_importance(&mut self) {
        self.tasks.sort_by(|a, b| a.cmp_importance(b));
    }
    pub fn sort_by_urgency(&mut self) {
        self.tasks.sort_by(|a, b| a.cmp_urgency(b));
    }

}

impl From<&[Task]> for Matrix {
    fn from(value: &[Task]) -> Self {
        Matrix { date:Utc::now().date_naive(), tasks: value.to_vec()}
    }
}
#[derive(Debug, Clone)]
pub struct Task {
    content: String,
    complete: Completeness,
    importance: Importance,
    urgency: Urgency,
}

impl Task {
    pub fn new(content: String, importance: Importance, urgency: Urgency) -> Task {
        Task {
            content,
            complete: Completeness::None,
            importance,
            urgency
        }
    }
    fn cmp_importance(&self, other: &Task) -> std::cmp::Ordering {
        self.importance.cmp(&other.importance)
    }
    fn cmp_urgency(&self, other: &Task) -> std::cmp::Ordering {
        self.urgency.cmp(&other.urgency)
    }
    pub fn content(&self) -> &str {
        &self.content
    }
    pub fn completeness(&self) -> &Completeness {
        &self.complete
    }
    pub fn set_completeness(&mut self, completeness: Completeness) {
        self.complete = completeness
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone)]
pub enum Importance {
    Low,
    MidLow,
    Mid,
    MidHigh,
    High
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone)]
pub enum Urgency {
    Low,
    MidLow,
    Mid,
    MidHigh,
    High
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone)]
pub enum Completeness {
    None,
    Started,
    Almost, 
    Complete
}
