pub struct Matrix {
    tasks: Vec<Vec<Option<Task>>>
}
impl Matrix {
    pub fn empty() -> Matrix {
        Matrix { tasks: vec![vec![None; 5]; 5] }
    }
}
#[derive(Clone, Debug)]
pub struct Task {
    content: String,
    completed: bool,
}

impl Task {

}
