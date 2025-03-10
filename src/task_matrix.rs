pub struct Matrix {
    tasks: Vec<Task>
}
impl <'a> Matrix {
    pub fn empty() -> Matrix {
        Matrix { tasks: Vec::new() }
    }
    pub fn get_tasks(&'a self) -> &'a Vec<Task> {
        &self.tasks
    }
    pub fn sort_by_importance(&mut self) {
        self.tasks.sort_by(|a, b| a.cmp_importance(b));
    }
    pub fn sort_by_urgency(&mut self) {
        self.tasks.sort_by(|a, b| a.cmp_urgency(b));
    }
}
#[derive(Debug)]
pub struct Task {
    content: String,
    completed: bool,
    importance: Importance,
    urgency: Urgency,
}

impl Task {
    pub fn cmp_importance(&self, other: &Task) -> std::cmp::Ordering {
        self.importance.cmp(&other.importance)
    }

    pub fn cmp_urgency(&self, other: &Task) -> std::cmp::Ordering {
        self.urgency.cmp(&other.urgency)
    }

    pub fn _test() {
        let task1 = Task { content: "h".to_owned(), completed: true, importance: Importance::High, urgency: Urgency::High};
        let task2 = Task { content: "h".to_owned(), completed: true, importance: Importance::MidLow, urgency: Urgency::High};
        println!("{:?}", task1.cmp_importance(&task2))
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
