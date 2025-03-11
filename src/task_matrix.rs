
use chrono::{NaiveDate, Utc};
pub struct Matrix {
    dates: Vec<NaiveDate>,
    tasks: Vec<Task>
}
impl Matrix {
    pub fn default() -> Matrix {
        Matrix { dates:vec![Utc::now().date_naive()], tasks: Vec::new() }
    }
    pub fn dates(&self) -> &Vec<NaiveDate> {
        &self.dates
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
        self.tasks.sort_by(|a, b| a.compare_importance(b));
    }
    pub fn sort_by_urgency(&mut self) {
        self.tasks.sort_by(|a, b| a.compare_urgency(b));
    }

}

// impl FromIterator<Task> for Matrix {
//     fn from_iter<T: IntoIterator<Item = Task>>(iter: T) -> Self {
//         Matrix { dates:Utc::now().date_naive(), tasks: iter.into_iter().collect() }
//     }
// }

// impl From<&[Task]> for Matrix {
//     fn from(value: &[Task]) -> Self {
//         Matrix { date: Utc::now().date_naive(), tasks: value.into() }
//     }
// }

#[derive(Debug, Clone)]
pub struct Task {
    id: u16,
    content: String,
    complete: Completeness,
    importance: Importance,
    urgency: Urgency,
}

impl Task {
    pub fn new(id: u16, content: String, importance: Importance, urgency: Urgency) -> Task {
        Task {
            id,
            content,
            complete: Completeness::None,
            importance,
            urgency
        }
    }
    pub fn id(&self) -> u16{
        self.id
    }
    pub fn compare_importance(&self, other: &Task) -> std::cmp::Ordering {
        self.importance.cmp(&other.importance)
    }
    pub fn compare_urgency(&self, other: &Task) -> std::cmp::Ordering {
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
