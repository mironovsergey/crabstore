const VERSION: &str = "0.1.0";
const MAX_TASKS: usize = 100;

fn main() {
    show_header();
    show_help();

    // Пример данных задачи (пока без хранения)
    let task_id: u32 = 1;
    let task_text = "Изучить Rust";
    let is_done = false;
    let priority: u8 = 3;

    println!();
    show_task(task_id, task_text, is_done, priority);
}

fn show_header() {
    println!("🦀 crabstore v{}", VERSION);
    println!("Максимум задач: {}", MAX_TASKS);
    println!();
}

fn show_help() {
    println!("Команды:");
    println!("  add <текст>    — добавить задачу");
    println!("  list           — показать все задачи");
    println!("  done <id>      — отметить выполненной");
    println!("  remove <id>    — удалить задачу");
}

fn show_task(id: u32, text: &str, done: bool, priority: u8) {
    let status = format_status(done);
    println!("[{}] #{} [P{}] {}", status, id, priority, text);
}

fn format_status(done: bool) -> &'static str {
    if done { "✓" } else { "○" }
}
