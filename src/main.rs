mod commands;
mod error;
mod store;
mod task;

use std::io::{self, Write};

use commands::{Command, ListFilter, parse};
use error::Result;
use store::TaskStore;
use task::Priority;

const VERSION: &str = "0.1.0";
const DATA_FILE: &str = "crabstore.json";

struct App {
    store: TaskStore,
}

impl App {
    fn new() -> Result<Self> {
        let store = TaskStore::load_or_create(DATA_FILE)?;
        Ok(Self { store })
    }

    fn run(&mut self) -> Result<()> {
        self.show_welcome();

        loop {
            let Some(input) = self.read_input() else {
                continue;
            };

            match self.process(&input) {
                Ok(true) => break, // quit
                Ok(false) => {}    // continue
                Err(e) => println!("❌ {}", e),
            }
        }

        Ok(())
    }

    fn process(&mut self, input: &str) -> Result<bool> {
        let cmd = parse(input)?;

        match cmd {
            Command::Empty => {}
            Command::Help => self.cmd_help(),
            Command::Add { text } => self.cmd_add(text)?,
            Command::List { filter } => self.cmd_list(filter),
            Command::Done { id } => self.cmd_done(id)?,
            Command::Priority { id, level } => self.cmd_priority(id, level)?,
            Command::Tag { id, tag } => self.cmd_tag(id, tag)?,
            Command::Stats => self.cmd_stats(),
            Command::Remove { id } => self.cmd_remove(id)?,
            Command::Quit => {
                println!("До встречи! 👋");
                return Ok(true);
            }
        }

        Ok(false)
    }

    fn show_welcome(&self) {
        println!("🦀 crabstore v{}", VERSION);
        let (total, done) = self.store.stats();
        if total > 0 {
            println!("📂 Загружено задач: {} (выполнено: {})", total, done);
        }
        println!("Введите 'help' для справки\n");
    }

    fn read_input(&self) -> Option<String> {
        print!("> ");
        io::stdout().flush().ok()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input).ok()?;
        Some(input)
    }

    fn cmd_help(&self) {
        println!("Команды:");
        println!("  add, a <текст>          — добавить задачу");
        println!("  list, ls [фильтр]       — показать задачи");
        println!("    list done             — только выполненные");
        println!("    list pending          — только активные");
        println!("    list sorted           — по приоритету");
        println!("    list p:N              — по приоритету N");
        println!("    list #tag             — по тегу");
        println!("  done, d <id>            — отметить выполненной");
        println!("  priority, p <id> <1-4>  — установить приоритет");
        println!("  tag, t <id> <тег>       — добавить тег");
        println!("  stats                   — статистика");
        println!("  remove, rm <id>         — удалить");
        println!("  quit, q                 — выход");
    }

    fn cmd_add(&mut self, text: String) -> Result<()> {
        let task = self.store.add(text)?;
        println!("➕ Добавлена: #{} '{}'", task.id, task.text);
        Ok(())
    }

    fn cmd_list(&self, filter: Option<ListFilter>) {
        if self.store.is_empty() {
            println!("Задач нет");
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

    fn cmd_done(&mut self, id: u32) -> Result<()> {
        self.store.update(id, |task| task.done = true)?;
        println!("✓ Задача #{} выполнена", id);
        Ok(())
    }

    fn cmd_priority(&mut self, id: u32, level: Priority) -> Result<()> {
        self.store.update(id, |task| task.priority = level)?;
        println!("🔄 Приоритет #{} → {}", id, level.icon());
        Ok(())
    }

    fn cmd_tag(&mut self, id: u32, tag: String) -> Result<()> {
        self.store.update(id, |task| {
            if !task.tags.contains(&tag) {
                task.tags.push(tag.clone());
            }
        })?;
        println!("🏷 Задача #{} + #{}", id, tag);
        Ok(())
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

    fn cmd_remove(&mut self, id: u32) -> Result<()> {
        let task = self.store.remove(id)?;
        println!("🗑 Удалена: #{} '{}'", task.id, task.text);
        Ok(())
    }
}

fn main() {
    let result = App::new().and_then(|mut app| app.run());

    if let Err(e) = result {
        eprintln!("Критическая ошибка: {}", e);
        std::process::exit(1);
    }
}
