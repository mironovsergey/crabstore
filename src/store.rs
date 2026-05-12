use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::error::{AppError, Result};
use crate::task::{Priority, Task};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
struct StoreData {
    next_id: u32,
    tasks: Vec<Task>,
}

/// Хранилище задач
pub struct TaskStore {
    tasks: Vec<Task>,
    next_id: u32,
    file_path: Option<String>,
}

impl TaskStore {
    /// Создание нового хранилища задач
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
            next_id: 1,
            file_path: None,
        }
    }

    /// Загрузить из файла или создать новое хранилище
    pub fn load_or_create(path: &str) -> Result<Self> {
        if Path::new(path).exists() {
            Self::load(path)
        } else {
            let mut store = Self::new();
            store.file_path = Some(path.to_string());
            Ok(store)
        }
    }

    /// Загрузить из файла
    pub fn load(path: &str) -> Result<Self> {
        let content = fs::read_to_string(path).map_err(AppError::FileRead)?;

        let data: StoreData = serde_json::from_str(&content)?;

        Ok(Self {
            tasks: data.tasks,
            next_id: data.next_id,
            file_path: Some(path.to_string()),
        })
    }

    /// Сохранить в файл
    pub fn save(&self) -> Result<()> {
        let Some(ref path) = self.file_path else {
            return Ok(()); // нет файла — не сохраняем
        };

        let data = StoreData {
            next_id: self.next_id,
            tasks: self.tasks.clone(),
        };

        let content = serde_json::to_string_pretty(&data)?;

        fs::write(path, content).map_err(AppError::FileWrite)?;

        Ok(())
    }

    /// Добавление новой задачи
    pub fn add(&mut self, text: String) -> Result<&Task> {
        if text.trim().is_empty() {
            return Err(AppError::EmptyTaskText);
        }

        let task = Task::new(self.next_id, text);
        self.next_id += 1;
        self.tasks.push(task);

        self.save()?;

        Ok(self.tasks.last().unwrap())
    }

    /// Поиск задачи по ID для изменения
    pub fn find_mut(&mut self, id: u32) -> Option<&mut Task> {
        self.tasks.iter_mut().find(|t| t.id == id)
    }

    /// Обновление задачи по ID с помощью функции-замыкания
    pub fn update<F>(&mut self, id: u32, f: F) -> Result<()>
    where
        F: FnOnce(&mut Task),
    {
        let task = self.find_mut(id).ok_or(AppError::TaskNotFound(id))?;

        f(task);
        self.save()?;

        Ok(())
    }

    /// Удаление задачи по ID
    pub fn remove(&mut self, id: u32) -> Result<Task> {
        let pos = self
            .tasks
            .iter()
            .position(|t| t.id == id)
            .ok_or(AppError::TaskNotFound(id))?;

        let task = self.tasks.remove(pos);
        self.save()?;

        Ok(task)
    }

    /// Получение всех задач
    pub fn all(&self) -> &[Task] {
        &self.tasks
    }

    /// Проверка на пустоту
    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }

    /// Фильтрация по статусу выполнения
    pub fn filter_by_done(&self, done: bool) -> Vec<&Task> {
        self.tasks.iter().filter(|t| t.done == done).collect()
    }

    /// Фильтрация по тегу
    pub fn filter_by_tag(&self, tag: &str) -> Vec<&Task> {
        self.tasks
            .iter()
            .filter(|t| t.tags.iter().any(|t| t == tag))
            .collect()
    }

    /// Фильтрация по приоритету
    pub fn filter_by_priority(&self, p: Priority) -> Vec<&Task> {
        self.tasks.iter().filter(|t| t.priority == p).collect()
    }

    /// Сортировка по приоритету
    pub fn sorted_by_priority(&self) -> Vec<&Task> {
        let mut sorted: Vec<_> = self.tasks.iter().collect();
        sorted.sort_by(|a, b| b.priority.cmp(&a.priority));
        sorted
    }

    /// Статистика по выполненным задачам
    pub fn stats(&self) -> (usize, usize) {
        let total = self.tasks.len();
        let done = self.tasks.iter().filter(|t| t.done).count();
        (total, done)
    }

    /// Статистика по тегам
    pub fn tag_stats(&self) -> HashMap<&str, usize> {
        let mut stats = HashMap::new();
        for task in &self.tasks {
            for tag in &task.tags {
                *stats.entry(tag.as_str()).or_insert(0) += 1;
            }
        }
        stats
    }

    /// Статистика по приоритетам
    pub fn priority_stats(&self) -> HashMap<Priority, usize> {
        let mut stats = HashMap::new();
        for task in &self.tasks {
            *stats.entry(task.priority).or_insert(0) += 1;
        }
        stats
    }
}

/// Реализация Default trait
impl Default for TaskStore {
    fn default() -> Self {
        Self::new()
    }
}
