use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    // Ошибки задач
    #[error("задача #{0} не найдена")]
    TaskNotFound(u32),

    #[error("текст задачи не может быть пустым")]
    EmptyTaskText,

    #[error("неверный приоритет: {0} (допустимо 1-4)")]
    InvalidPriority(u8),

    // Ошибки команд
    #[error("неизвестная команда: '{0}'")]
    UnknownCommand(String),

    #[error("не указан аргумент: {0}")]
    MissingArgument(&'static str),

    #[error("неверный ID задачи")]
    InvalidId,

    // Ошибки хранилища
    #[error("ошибка чтения файла: {0}")]
    FileRead(#[source] std::io::Error),

    #[error("ошибка записи файла: {0}")]
    FileWrite(#[source] std::io::Error),

    #[error("ошибка формата данных: {0}")]
    DataFormat(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, AppError>;
