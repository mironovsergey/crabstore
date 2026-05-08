use std::io::{self, Write};

const VERSION: &str = "0.1.0";

// ============ Модель данных ============

#[derive(Debug, Clone)]
struct Task {
    id: u32,
    text: String,
    done: bool,
    priority: Priority,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Priority {
    Low,
    Normal,
    High,
    Critical,
}

impl Priority {
    fn from_u8(value: u8) -> Option<Priority> {
        match value {
            1 => Some(Priority::Low),
            2 => Some(Priority::Normal),
            3 => Some(Priority::High),
            4 => Some(Priority::Critical),
            _ => None,
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

    fn label(&self) -> &str {
        match self {
            Priority::Low => "низкий",
            Priority::Normal => "обычный",
            Priority::High => "высокий",
            Priority::Critical => "критический",
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
        let done_label = if self.done {
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
            done_label
        );
    }
}

// ============ Команды как enum ============

#[derive(Debug)]
enum Command {
    Help,
    Add { text: String },
    List,
    Done { id: u32 },
    Priority { id: u32, level: Priority },
    Remove { id: u32 },
    Quit,
}

#[derive(Debug)]
enum ParseError {
    EmptyInput,
    UnknownCommand(String),
    MissingArgument(&'static str),
    InvalidId,
    InvalidPriority,
}

impl ParseError {
    fn message(&self) -> String {
        match self {
            ParseError::EmptyInput => "Введите команду".to_string(),
            ParseError::UnknownCommand(cmd) => {
                format!("Неизвестная команда: '{}'. Введите 'help'.", cmd)
            }
            ParseError::MissingArgument(arg) => {
                format!("Не указан аргумент: {}", arg)
            }
            ParseError::InvalidId => "Неверный ID (должно быть число)".to_string(),
            ParseError::InvalidPriority => "Неверный приоритет (должно быть 1-4)".to_string(),
        }
    }
}

fn parse_command(input: &str) -> Result<Command, ParseError> {
    let input = input.trim();

    if input.is_empty() {
        return Err(ParseError::EmptyInput);
    }

    let parts: Vec<&str> = input.splitn(2, ' ').collect();
    let cmd = parts[0];
    let args = parts.get(1).copied().unwrap_or("");

    match cmd {
        "help" | "h" => Ok(Command::Help),

        "add" | "a" => {
            if args.is_empty() {
                Err(ParseError::MissingArgument("текст задачи"))
            } else {
                Ok(Command::Add {
                    text: args.to_string(),
                })
            }
        }

        "list" | "ls" => Ok(Command::List),

        "done" | "d" => {
            let id = args.parse::<u32>().map_err(|_| ParseError::InvalidId)?;
            Ok(Command::Done { id })
        }

        "priority" | "p" => {
            let parts: Vec<&str> = args.split_whitespace().collect();

            if parts.len() < 2 {
                return Err(ParseError::MissingArgument("id и приоритет"));
            }

            let id = parts[0].parse::<u32>().map_err(|_| ParseError::InvalidId)?;

            let level_num = parts[1]
                .parse::<u8>()
                .map_err(|_| ParseError::InvalidPriority)?;

            let level = Priority::from_u8(level_num).ok_or(ParseError::InvalidPriority)?;

            Ok(Command::Priority { id, level })
        }

        "remove" | "rm" => {
            let id = args.parse::<u32>().map_err(|_| ParseError::InvalidId)?;
            Ok(Command::Remove { id })
        }

        "quit" | "q" | "exit" => Ok(Command::Quit),

        other => Err(ParseError::UnknownCommand(other.to_string())),
    }
}

// ============ Хранилище ============

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
        let pos = self.tasks.iter().position(|t| t.id == id)?;
        Some(self.tasks.remove(pos))
    }

    fn list(&self) -> &[Task] {
        &self.tasks
    }

    fn stats(&self) -> (usize, usize) {
        let total = self.tasks.len();
        let done = self.tasks.iter().filter(|t| t.done).count();
        (total, done)
    }

    fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }
}

// ============ Результат выполнения команды ============

enum CommandResult {
    Continue,
    Quit,
}

// ============ Приложение ============

struct App {
    store: TaskStore,
}

impl App {
    fn new() -> Self {
        Self {
            store: TaskStore::new(),
        }
    }

    fn run(&mut self) {
        self.show_welcome();

        loop {
            let Some(input) = self.read_input() else {
                continue;
            };

            match parse_command(&input) {
                Ok(cmd) => {
                    if let CommandResult::Quit = self.execute(cmd) {
                        break;
                    }
                }
                Err(e) => println!("{}", e.message()),
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

        Some(input)
    }

    fn execute(&mut self, cmd: Command) -> CommandResult {
        match cmd {
            Command::Help => self.cmd_help(),
            Command::Add { text } => self.cmd_add(text),
            Command::List => self.cmd_list(),
            Command::Done { id } => self.cmd_done(id),
            Command::Priority { id, level } => self.cmd_priority(id, level),
            Command::Remove { id } => self.cmd_remove(id),
            Command::Quit => return CommandResult::Quit,
        }
        CommandResult::Continue
    }

    fn cmd_help(&self) {
        println!("Команды:");
        println!("  add, a <текст>         — добавить задачу");
        println!("  list, ls               — показать задачи");
        println!("  done, d <id>           — отметить выполненной");
        println!("  priority, p <id> <1-4> — установить приоритет");
        println!("  remove, rm <id>        — удалить задачу");
        println!("  quit, q                — выход");
        println!();
        println!("Приоритеты:");
        println!("  1 — {} низкий", Priority::Low.icon());
        println!("  2 — {} обычный", Priority::Normal.icon());
        println!("  3 — {} высокий", Priority::High.icon());
        println!("  4 — {} критический", Priority::Critical.icon());
    }

    fn cmd_add(&mut self, text: String) {
        let task = self.store.add(text);
        println!("➕ Добавлена: #{} '{}'", task.id, task.text);
    }

    fn cmd_list(&self) {
        if self.store.is_empty() {
            println!("Задач нет. Добавьте: add <текст>");
            return;
        }

        println!("\n📋 Список задач:");
        println!("{}", "─".repeat(50));

        for task in self.store.list() {
            task.display();
        }

        println!("{}", "─".repeat(50));

        let (total, done) = self.store.stats();
        println!("Всего: {} | Выполнено: {}\n", total, done);
    }

    fn cmd_done(&mut self, id: u32) {
        match self.store.find_mut(id) {
            Some(task) => {
                task.done = true;
                println!("✓ Задача #{} выполнена", id);
            }
            None => println!("Задача #{} не найдена", id),
        }
    }

    fn cmd_priority(&mut self, id: u32, level: Priority) {
        match self.store.find_mut(id) {
            Some(task) => {
                task.priority = level;
                println!(
                    "🔄 Задача #{}: приоритет → {} {}",
                    id,
                    level.icon(),
                    level.label()
                );
            }
            None => println!("Задача #{} не найдена", id),
        }
    }

    fn cmd_remove(&mut self, id: u32) {
        match self.store.remove(id) {
            Some(task) => {
                println!("🗑 Удалена: #{} '{}'", task.id, task.text)
            }
            None => println!("Задача #{} не найдена", id),
        }
    }
}

// ============ Точка входа ============

fn main() {
    let mut app = App::new();
    app.run();
}
