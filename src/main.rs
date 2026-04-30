use std::io::{self, Write};

const VERSION: &str = "0.1.0";

// ============ Модель данных ============

#[derive(Debug)]
struct Task {
    id: u32,
    text: String,
    done: bool,
    priority: Priority,
}

#[derive(Debug, Clone, Copy)]
enum Priority {
    Low,
    Normal,
    High,
    Critical,
}

impl Priority {
    fn from_u8(value: u8) -> Priority {
        match value {
            1 => Priority::Low,
            2 => Priority::Normal,
            3 => Priority::High,
            4 => Priority::Critical,
            _ => Priority::Normal,
        }
    }

    fn icon(&self) -> &str {
        match self {
            Priority::Low => "⬇",
            Priority::Normal => "➡",
            Priority::High => "⬆",
            Priority::Critical => "🔥",
        }
    }
}

impl Task {
    fn new(id: u32, text: String) -> Self {
        Self {
            id,
            text,
            done: false,
            priority: Priority::Normal,
        }
    }

    fn display(&self) {
        let status = if self.done { "✓" } else { "○" };
        let done_mark = if self.done {
            " (выполнено)"
        } else {
            ""
        };
        println!(
            "[{}] {} #{} {}{}",
            status,
            self.priority.icon(),
            self.id,
            self.text,
            done_mark
        );
    }

    fn complete(&mut self) {
        self.done = true;
    }

    fn set_priority(&mut self, priority: Priority) {
        self.priority = priority;
    }
}

// ============ Хранилище задач ============

struct TaskStore {
    tasks: Vec<Task>,
    next_id: u32,
}

impl TaskStore {
    fn new() -> Self {
        Self {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    fn add(&mut self, text: String) -> &Task {
        let task = Task::new(self.next_id, text);
        self.next_id += 1;
        self.tasks.push(task);
        self.tasks.last().unwrap()
    }

    fn find_mut(&mut self, id: u32) -> Option<&mut Task> {
        self.tasks.iter_mut().find(|t| t.id == id)
    }

    fn remove(&mut self, id: u32) -> Option<Task> {
        if let Some(pos) = self.tasks.iter().position(|t| t.id == id) {
            Some(self.tasks.remove(pos))
        } else {
            None
        }
    }

    fn list(&self) -> &[Task] {
        &self.tasks
    }

    fn count(&self) -> usize {
        self.tasks.len()
    }

    fn count_done(&self) -> usize {
        self.tasks.iter().filter(|t| t.done).count()
    }

    fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }
}

// ============ Приложение ============

struct App {
    store: TaskStore,
    running: bool,
}

impl App {
    fn new() -> Self {
        Self {
            store: TaskStore::new(),
            running: true,
        }
    }

    fn run(&mut self) {
        self.show_welcome();

        while self.running {
            if let Some(input) = self.read_input() {
                self.handle_command(&input);
            }
        }
    }

    fn show_welcome(&self) {
        println!("🦀 crabstore v{}", VERSION);
        println!("Введите 'help' для списка команд\n");
    }

    fn read_input(&self) -> Option<String> {
        print!("> ");
        io::stdout().flush().ok()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input).ok()?;

        let trimmed = input.trim();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed.to_string())
        }
    }

    fn handle_command(&mut self, input: &str) {
        let parts: Vec<&str> = input.splitn(2, ' ').collect();
        let command = parts[0];
        let args = parts.get(1).copied().unwrap_or("");

        match command {
            "help" | "h" => self.cmd_help(),
            "add" | "a" => self.cmd_add(args),
            "list" | "ls" => self.cmd_list(),
            "done" | "d" => self.cmd_done(args),
            "priority" | "p" => self.cmd_priority(args),
            "remove" | "rm" => self.cmd_remove(args),
            "quit" | "q" | "exit" => self.cmd_quit(),
            _ => println!("Неизвестная команда. Введите 'help'."),
        }
    }

    fn cmd_help(&self) {
        println!("Команды:");
        println!("  add, a <текст>        — добавить задачу");
        println!("  list, ls              — показать задачи");
        println!("  done, d <id>          — отметить выполненной");
        println!("  priority, p <id> <1-4> — установить приоритет");
        println!("  remove, rm <id>       — удалить задачу");
        println!("  quit, q               — выход");
        println!("\nПриоритеты: 1=низкий, 2=обычный, 3=высокий, 4=критический");
    }

    fn cmd_add(&mut self, args: &str) {
        if args.is_empty() {
            println!("Использование: add <текст задачи>");
            return;
        }

        let task = self.store.add(args.to_string());
        println!("➕ Добавлена задача #{}: '{}'", task.id, task.text);
    }

    fn cmd_list(&self) {
        if self.store.is_empty() {
            println!("Задач нет. Добавьте: add <текст>");
            return;
        }

        println!("\n📋 Список задач:");
        println!("{}", "─".repeat(45));

        for task in self.store.list() {
            task.display();
        }

        println!("{}", "─".repeat(45));
        println!(
            "Всего: {} | Выполнено: {}\n",
            self.store.count(),
            self.store.count_done()
        );
    }

    fn cmd_done(&mut self, args: &str) {
        let id = match args.parse::<u32>() {
            Ok(id) => id,
            Err(_) => {
                println!("Использование: done <id>");
                return;
            }
        };

        match self.store.find_mut(id) {
            Some(task) => {
                task.complete();
                println!("✓ Задача #{} выполнена", id);
            }
            None => println!("Задача #{} не найдена", id),
        }
    }

    fn cmd_priority(&mut self, args: &str) {
        let parts: Vec<&str> = args.split_whitespace().collect();

        if parts.len() != 2 {
            println!("Использование: priority <id> <1-4>");
            return;
        }

        let id = match parts[0].parse::<u32>() {
            Ok(id) => id,
            Err(_) => {
                println!("Неверный ID");
                return;
            }
        };

        let priority_value = match parts[1].parse::<u8>() {
            Ok(p) if p >= 1 && p <= 4 => p,
            _ => {
                println!("Приоритет должен быть от 1 до 4");
                return;
            }
        };

        match self.store.find_mut(id) {
            Some(task) => {
                task.set_priority(Priority::from_u8(priority_value));
                println!("🔄 Приоритет задачи #{} изменён", id);
            }
            None => println!("Задача #{} не найдена", id),
        }
    }

    fn cmd_remove(&mut self, args: &str) {
        let id = match args.parse::<u32>() {
            Ok(id) => id,
            Err(_) => {
                println!("Использование: remove <id>");
                return;
            }
        };

        match self.store.remove(id) {
            Some(task) => println!("🗑 Удалена задача #{}: '{}'", task.id, task.text),
            None => println!("Задача #{} не найдена", id),
        }
    }

    fn cmd_quit(&mut self) {
        self.running = false;
        println!("До встречи! 👋");
    }
}

// ============ Точка входа ============

fn main() {
    let mut app = App::new();
    app.run();
}
