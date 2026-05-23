mod task;

pub use task::Task;

use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum TaskError {
    MissingComma,
    EmptyTitle,
    InvalidStatus(String),
}

impl fmt::Display for TaskError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaskError::MissingComma => write!(f, "each task line must contain a comma"),
            TaskError::EmptyTitle => write!(f, "task titles cannot be empty"),
            TaskError::InvalidStatus(status) => {
                write!(f, "task status must be `true` or `false`, got `{status}`")
            }
        }
    }
}

impl Error for TaskError {}

pub fn parse_task(line: &str) -> Result<Task, TaskError> {
    let (title, done_text) = line.split_once(',').ok_or(TaskError::MissingComma)?;
    let title = title.trim();

    if title.is_empty() {
        return Err(TaskError::EmptyTitle);
    }

    let done = match done_text.trim() {
        "true" => true,
        "false" => false,
        other => return Err(TaskError::InvalidStatus(other.to_string())),
    };

    Ok(Task::new(title, done))
}

pub fn parse_tasks(input: &str) -> Result<Vec<Task>, TaskError> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(parse_task)
        .collect()
}

pub fn build_report(input: &str) -> Result<String, TaskError> {
    let tasks = parse_tasks(input)?;
    let total = tasks.len();
    let completed = tasks.iter().filter(|task| task.done).count();
    let remaining: Vec<&str> = tasks
        .iter()
        .filter(|task| !task.done)
        .map(|task| task.title.as_str())
        .collect();

    Ok(format!(
        "Scope: track a small project checklist.\nCompleted: {completed}/{total}\nRemaining: {}",
        remaining.join(", ")
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_task_reads_title_and_status() {
        assert_eq!(
            parse_task("Write tests,true"),
            Ok(Task::new("Write tests", true))
        );
    }

    #[test]
    fn parse_task_rejects_invalid_status() {
        assert_eq!(
            parse_task("Write docs,maybe"),
            Err(TaskError::InvalidStatus(String::from("maybe")))
        );
    }

    #[test]
    fn build_report_summarizes_remaining_work() {
        let report = build_report("Write tests,true\nShip project,false").unwrap();
        assert!(report.contains("Completed: 1/2"));
        assert!(report.contains("Remaining: Ship project"));
    }
}
