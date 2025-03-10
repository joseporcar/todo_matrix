pub struct Matrix {
    tasks: Vec<Task>
}
impl Matrix {
    pub fn empty() -> Matrix {
        Matrix { tasks: Vec::new() }
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

impl From<Vec<Task>> for Matrix {
    fn from(value: Vec<Task>) -> Self {
        Matrix { tasks: value}
    }
}
#[derive(Debug)]
pub struct Task {
    content: String,
    completed: bool,
    importance: Importance,
    urgency: Urgency,
}

impl<'a> Task {
    fn cmp_importance(&self, other: &Task) -> std::cmp::Ordering {
        self.importance.cmp(&other.importance)
    }

    fn cmp_urgency(&self, other: &Task) -> std::cmp::Ordering {
        self.urgency.cmp(&other.urgency)
    }
    fn content(&self) -> &str {
        &self.content
    }

}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum Importance {
    Low,
    MidLow,
    Mid,
    MidHigh,
    High
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum Urgency {
    Low,
    MidLow,
    Mid,
    MidHigh,
    High
}
