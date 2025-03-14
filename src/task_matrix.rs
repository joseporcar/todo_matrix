use chrono::{NaiveDate, Utc};
pub struct DayMatrix {
    date: NaiveDate,
    tasks: Vec<Task>
}
impl DayMatrix {
    pub fn default() -> DayMatrix {
        DayMatrix { date:Utc::now().date_naive(), tasks: Vec::new() }
    }
    pub fn date(&self) -> &NaiveDate {
        &self.date
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
    id: Uploaded,
    content: String,
    complete: Completeness,
    dates: Vec<NaiveDate>,
    importance: Importance,
    urgency: Urgency,
}

impl Task {
    pub fn new(content: String, dates: Vec<NaiveDate>, importance: Importance, urgency: Urgency) -> Task {
        Task {
            id: Uploaded::NotUploaded,
            content,
            complete: Completeness::None,
            dates,
            importance,
            urgency
        }
    }
    // pub fn from_sql(id: u32, dates: &str, content: String, importance: u8, urgency: u8) -> Task {
    //     let dates = dates.split("_").map(|date| NaiveDate::from_str(date).unwrap()).collect();

    //     Task {
    //         id: Uploaded::Uploaded(id),
    //         content,
    //         dates,
    //         complete: Completeness::None,
    //         importance,
    //         urgency
    //     }
    // }
    pub fn set_id(&mut self, id: u32) {
        self.id = Uploaded::Uploaded(id)
    }
    pub fn id(&self) -> Option<u32>{
        match self.id {
            Uploaded::Uploaded(id) => Some(id),
            Uploaded::NotUploaded => None
        }
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
    pub fn dates(&self) -> &Vec<NaiveDate> {
        &self.dates
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

#[derive(Debug, Clone)]
enum Uploaded {
    Uploaded(u32),
    NotUploaded,
}