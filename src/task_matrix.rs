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

}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum Importance {
    Low,
    MidLo,
    Mid,
    MidHigh,
    High
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum Urgency {
    Low,
    MidLo,
    Mid,
    MidHigh,
    High
}
