mod commands;
mod store;
mod task;

use std::io::{self, Write};

use commands::{Command, CommandResult, ListFilter, parse};
use store::TaskStore;
use task::Priority;

const VERSION: &str = "0.1.0";

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

            match parse(&input) {
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
        println!("    list #tag             — по тегу");
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

        let tasks: Vec<_> = match filter {
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
                task.complete();
                println!("✓ Задача #{} выполнена", id);
            }
            None => println!("Задача #{} не найдена", id),
        }
    }

    fn cmd_priority(&mut self, id: u32, level: Priority) {
        match self.store.find_mut(id) {
            Some(task) => {
                task.set_priority(level);
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
