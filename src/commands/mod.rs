mod parser;

use crate::task::Priority;

pub use parser::parse;

/// Команда пользователя
#[derive(Debug)]
pub enum Command {
    Empty,
    Help,
    Add { text: String },
    List { filter: Option<ListFilter> },
    Done { id: u32 },
    Priority { id: u32, level: Priority },
    Tag { id: u32, tag: String },
    Stats,
    Remove { id: u32 },
    Quit,
}

/// Фильтр для списка задач
#[derive(Debug)]
pub enum ListFilter {
    Done,
    Pending,
    Priority(Priority),
    Tag(String),
    Sorted,
}
