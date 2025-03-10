pub struct Matrix {
    tasks: Vec<Vec<Option<Task>>>
}
impl Matrix {
    // pub fn empty() -> Matrix {
    //     Matrix { tasks: vec![vec![None; 5]; 5] }
    // }
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
