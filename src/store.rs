use crate::task::{Priority, Task};
use std::collections::HashMap;

/// Хранилище задач
pub struct TaskStore {
    tasks: Vec<Task>,
    next_id: u32,
}

impl TaskStore {
    // Создание нового хранилища задач
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    // Добавление новой задачи
    pub fn add(&mut self, text: String) -> &Task {
        let task = Task::new(self.next_id, text);
        self.next_id += 1;
        self.tasks.push(task);
        self.tasks.last().unwrap()
    }

    // Поиск задачи по ID
    pub fn find_mut(&mut self, id: u32) -> Option<&mut Task> {
        self.tasks.iter_mut().find(|t| t.id == id)
    }

    // Удаление задачи по ID
    pub fn remove(&mut self, id: u32) -> Option<Task> {
        let pos = self.tasks.iter().position(|t| t.id == id)?;
        Some(self.tasks.remove(pos))
    }

    // Получение всех задач
    pub fn all(&self) -> &[Task] {
        &self.tasks
    }

    // Проверка на пустоту
    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }

    // Фильтрация по статусу выполнения
    pub fn filter_by_done(&self, done: bool) -> Vec<&Task> {
        self.tasks.iter().filter(|t| t.done == done).collect()
    }

    // Фильтрация по тегу
    pub fn filter_by_tag(&self, tag: &str) -> Vec<&Task> {
        self.tasks.iter().filter(|t| t.has_tag(tag)).collect()
    }

    // Фильтрация по приоритету
    pub fn filter_by_priority(&self, priority: Priority) -> Vec<&Task> {
        self.tasks
            .iter()
            .filter(|t| t.priority == priority)
            .collect()
    }

    // Сортировка по приоритету
    pub fn sorted_by_priority(&self) -> Vec<&Task> {
        let mut sorted: Vec<&Task> = self.tasks.iter().collect();
        sorted.sort_by(|a, b| b.priority.cmp(&a.priority));
        sorted
    }

    // Статистика по выполненным задачам
    pub fn stats(&self) -> (usize, usize) {
        let total = self.tasks.len();
        let done = self.tasks.iter().filter(|t| t.done).count();
        (total, done)
    }

    // Статистика по тегам
    pub fn tag_stats(&self) -> HashMap<&str, usize> {
        let mut stats: HashMap<&str, usize> = HashMap::new();
        for task in &self.tasks {
            for tag in &task.tags {
                *stats.entry(tag.as_str()).or_insert(0) += 1;
            }
        }
        stats
    }

    // Статистика по приоритетам
    pub fn priority_stats(&self) -> HashMap<Priority, usize> {
        let mut stats: HashMap<Priority, usize> = HashMap::new();
        for task in &self.tasks {
            *stats.entry(task.priority).or_insert(0) += 1;
        }
        stats
    }
}

// Реализация Default trait
impl Default for TaskStore {
    fn default() -> Self {
        Self::new()
    }
}
