use std::collections::HashMap;
use std::io::{self, Write};

const VERSION: &str = "0.1.0";

#[derive(Debug, Clone)]
struct Task {
    id: u32,
    text: String,
    done: bool,
    priority: Priority,
    tags: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Priority {
    Low = 1,
    Normal = 2,
    High = 3,
    Critical = 4,
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
}

impl Task {
    fn new(id: u32, text: String) -> Self {
        Self {
            id,
            text,
            done: false,
            priority: Priority::Normal,
            tags: Vec::new(),
        }
    }

    fn display(&self) {
        let status = if self.done { "✓" } else { "○" };
        let tags_str = if self.tags.is_empty() {
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
            tags_str
        );
    }

    fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
        }
    }

    fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
    }
}

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

    // Фильтрация по статусу
    fn filter_by_done(&self, done: bool) -> Vec<&Task> {
        self.tasks.iter().filter(|t| t.done == done).collect()
    }

    // Фильтрация по тегу
    fn filter_by_tag(&self, tag: &str) -> Vec<&Task> {
        self.tasks.iter().filter(|t| t.has_tag(tag)).collect()
    }

    // Фильтрация по приоритету
    fn filter_by_priority(&self, priority: Priority) -> Vec<&Task> {
        self.tasks
            .iter()
            .filter(|t| t.priority == priority)
            .collect()
    }

    // Сортировка по приоритету
    fn sorted_by_priority(&self) -> Vec<&Task> {
        let mut sorted: Vec<&Task> = self.tasks.iter().collect();
        sorted.sort_by(|a, b| b.priority.cmp(&a.priority)); // по убыванию
        sorted
    }

    // Статистика по тегам
    fn tag_stats(&self) -> HashMap<&str, usize> {
        let mut stats: HashMap<&str, usize> = HashMap::new();

        for task in &self.tasks {
            for tag in &task.tags {
                let count = stats.entry(tag.as_str()).or_insert(0);
                *count += 1;
            }
        }

        stats
    }

    // Статистика по приоритетам
    fn priority_stats(&self) -> HashMap<Priority, usize> {
        let mut stats: HashMap<Priority, usize> = HashMap::new();

        for task in &self.tasks {
            let count = stats.entry(task.priority).or_insert(0);
            *count += 1;
        }

        stats
    }

    fn all(&self) -> &[Task] {
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

#[derive(Debug)]
enum Command {
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

#[derive(Debug)]
enum ListFilter {
    Done,
    Pending,
    Priority(Priority),
    Tag(String),
    Sorted,
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
                Err(ParseError::MissingArgument("текст"))
            } else {
                Ok(Command::Add {
                    text: args.to_string(),
                })
            }
        }

        "list" | "ls" => {
            let filter = match args.trim() {
                "" => None,
                "done" => Some(ListFilter::Done),
                "pending" | "todo" => Some(ListFilter::Pending),
                "sorted" | "sort" => Some(ListFilter::Sorted),
                arg if arg.starts_with("p:") => {
                    let p = arg[2..]
                        .parse::<u8>()
                        .ok()
                        .and_then(Priority::from_u8)
                        .ok_or(ParseError::InvalidPriority)?;
                    Some(ListFilter::Priority(p))
                }
                arg if arg.starts_with("#") => Some(ListFilter::Tag(arg[1..].to_string())),
                _ => None,
            };
            Ok(Command::List { filter })
        }

        "done" | "d" => {
            let id = args.parse().map_err(|_| ParseError::InvalidId)?;
            Ok(Command::Done { id })
        }

        "priority" | "p" => {
            let parts: Vec<&str> = args.split_whitespace().collect();
            if parts.len() < 2 {
                return Err(ParseError::MissingArgument("id и приоритет"));
            }
            let id = parts[0].parse().map_err(|_| ParseError::InvalidId)?;
            let level = parts[1]
                .parse::<u8>()
                .ok()
                .and_then(Priority::from_u8)
                .ok_or(ParseError::InvalidPriority)?;
            Ok(Command::Priority { id, level })
        }

        "tag" | "t" => {
            let parts: Vec<&str> = args.split_whitespace().collect();
            if parts.len() < 2 {
                return Err(ParseError::MissingArgument("id и тег"));
            }
            let id = parts[0].parse().map_err(|_| ParseError::InvalidId)?;
            let tag = parts[1].trim_start_matches('#').to_string();
            Ok(Command::Tag { id, tag })
        }

        "stats" => Ok(Command::Stats),

        "remove" | "rm" => {
            let id = args.parse().map_err(|_| ParseError::InvalidId)?;
            Ok(Command::Remove { id })
        }

        "quit" | "q" | "exit" => Ok(Command::Quit),

        other => Err(ParseError::UnknownCommand(other.to_string())),
    }
}

enum CommandResult {
    Continue,
    Quit,
}

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
            Command::List { filter } => self.cmd_list(filter),
            Command::Done { id } => self.cmd_done(id),
            Command::Priority { id, level } => self.cmd_priority(id, level),
            Command::Tag { id, tag } => self.cmd_tag(id, tag),
            Command::Stats => self.cmd_stats(),
            Command::Remove { id } => self.cmd_remove(id),
            Command::Quit => return CommandResult::Quit,
        }
        CommandResult::Continue
    }

    fn cmd_help(&self) {
        println!("Команды:");
        println!("  add, a <текст>          — добавить задачу");
        println!("  list, ls [фильтр]       — показать задачи");
        println!("    list done             — только выполненные");
        println!("    list pending          — только активные");
        println!("    list sorted           — по приоритету");
        println!("    list p:3              — по приоритету 3");
        println!("    list #work            — по тегу");
        println!("  done, d <id>            — отметить выполненной");
        println!("  priority, p <id> <1-4>  — установить приоритет");
        println!("  tag, t <id> <тег>       — добавить тег");
        println!("  stats                   — статистика");
        println!("  remove, rm <id>         — удалить");
        println!("  quit, q                 — выход");
    }

    fn cmd_add(&mut self, text: String) {
        let task = self.store.add(text);
        println!("➕ Добавлена: #{} '{}'", task.id, task.text);
    }

    fn cmd_list(&self, filter: Option<ListFilter>) {
        if self.store.is_empty() {
            println!("Задач нет. Добавьте: add <текст>");
            return;
        }

        let tasks: Vec<&Task> = match filter {
            None => self.store.all().iter().collect(),
            Some(ListFilter::Done) => self.store.filter_by_done(true),
            Some(ListFilter::Pending) => self.store.filter_by_done(false),
            Some(ListFilter::Priority(p)) => self.store.filter_by_priority(p),
            Some(ListFilter::Tag(ref tag)) => self.store.filter_by_tag(tag),
            Some(ListFilter::Sorted) => self.store.sorted_by_priority(),
        };

        if tasks.is_empty() {
            println!("Ничего не найдено");
            return;
        }

        println!("\n📋 Задачи:");
        println!("{}", "─".repeat(50));

        for task in tasks {
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
                println!("🔄 Приоритет #{} → {}", id, level.icon());
            }
            None => println!("Задача #{} не найдена", id),
        }
    }

    fn cmd_tag(&mut self, id: u32, tag: String) {
        match self.store.find_mut(id) {
            Some(task) => {
                task.add_tag(tag.clone());
                println!("🏷 Задача #{} + #{}", id, tag);
            }
            None => println!("Задача #{} не найдена", id),
        }
    }

    fn cmd_stats(&self) {
        let (total, done) = self.store.stats();

        println!("\n📊 Статистика:");
        println!("{}", "─".repeat(30));
        println!("Всего задач: {}", total);
        println!(
            "Выполнено: {} ({}%)",
            done,
            if total > 0 { done * 100 / total } else { 0 }
        );
        println!("Активных: {}", total - done);

        // Статистика по приоритетам
        let priority_stats = self.store.priority_stats();
        if !priority_stats.is_empty() {
            println!("\nПо приоритетам:");
            for (priority, count) in &priority_stats {
                println!("  {} : {}", priority.icon(), count);
            }
        }

        // Статистика по тегам
        let tag_stats = self.store.tag_stats();
        if !tag_stats.is_empty() {
            println!("\nПо тегам:");
            for (tag, count) in &tag_stats {
                println!("  #{}: {}", tag, count);
            }
        }

        println!();
    }

    fn cmd_remove(&mut self, id: u32) {
        match self.store.remove(id) {
            Some(task) => println!("🗑 Удалена: #{} '{}'", task.id, task.text),
            None => println!("Задача #{} не найдена", id),
        }
    }
}

fn main() {
    let mut app = App::new();
    app.run();
}
