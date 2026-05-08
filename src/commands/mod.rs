mod parser;

use crate::task::Priority;

pub use parser::parse; // реэкспорт функции parse

/// Команда пользователя
#[derive(Debug)]
pub enum Command {
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

/// Ошибка парсинга команды
#[derive(Debug)]
pub enum ParseError {
    EmptyInput,
    UnknownCommand(String),
    MissingArgument(&'static str),
    InvalidId,
    InvalidPriority,
}

impl ParseError {
    pub fn message(&self) -> String {
        match self {
            ParseError::EmptyInput => "Введите команду".to_string(),
            ParseError::UnknownCommand(cmd) => {
                format!("Неизвестная команда: '{}'", cmd)
            }
            ParseError::MissingArgument(arg) => {
                format!("Не указан аргумент: {}", arg)
            }
            ParseError::InvalidId => "Неверный ID".to_string(),
            ParseError::InvalidPriority => "Приоритет: 1-4".to_string(),
        }
    }
}

/// Результат выполнения команды
pub enum CommandResult {
    Continue,
    Quit,
}
