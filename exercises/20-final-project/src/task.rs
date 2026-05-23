#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Task {
    pub title: String,
    pub done: bool,
}

impl Task {
    pub fn new(title: impl Into<String>, done: bool) -> Self {
        Self {
            title: title.into(),
            done,
        }
    }
}
