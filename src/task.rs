use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Priority {
    Low = 1,
    Normal = 2,
    High = 3,
    Critical = 4,
}

impl Priority {
    pub fn from_u8(value: u8) -> Option<Priority> {
        match value {
            1 => Some(Priority::Low),
            2 => Some(Priority::Normal),
            3 => Some(Priority::High),
            4 => Some(Priority::Critical),
            _ => None,
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            Priority::Low => "⬇",
            Priority::Normal => "➡",
            Priority::High => "⬆",
            Priority::Critical => "🔥",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: u32,
    pub text: String,
    pub done: bool,
    pub priority: Priority,
    #[serde(default)]
    pub tags: Vec<String>,
}

impl Task {
    pub fn new(id: u32, text: String) -> Self {
        Self {
            id,
            text,
            done: false,
            priority: Priority::Normal,
            tags: Vec::new(),
        }
    }

    pub fn display(&self) {
        let status = if self.done { "✓" } else { "○" };
        let tags = if self.tags.is_empty() {
            String::new()
        } else {
            format!(" [{}]", self.tags.join(", "))
        };

        println!(
            "[{}] {} #{} {}{}",
            status,
            self.priority.icon(),
            self.id,
            self.text,
            tags
        );
    }
}
