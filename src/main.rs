use std::io::{self, Write};

const VERSION: &str = "0.1.0";

fn main() {
    println!("🦀 crabstore v{}", VERSION);
    println!("Введите 'help' для списка команд\n");

    loop {
        // Показываем приглашение
        print!("> ");
        io::stdout().flush().unwrap();

        // Читаем ввод
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Ошибка чтения ввода");
            continue;
        }

        // Убираем пробелы и переводы строк
        let input = input.trim();

        // Пустая строка — пропускаем
        if input.is_empty() {
            continue;
        }

        // Разбираем команду
        let parts: Vec<&str> = input.splitn(2, ' ').collect();
        let command = parts[0];
        let args = if parts.len() > 1 { parts[1] } else { "" };

        // Обрабатываем команды
        match command {
            "help" | "h" => show_help(),
            "add" | "a" => {
                if args.is_empty() {
                    println!("Использование: add <текст задачи>");
                } else {
                    add_task(args);
                }
            }
            "list" | "ls" => list_tasks(),
            "done" | "d" => {
                if let Ok(id) = args.parse::<u32>() {
                    complete_task(id);
                } else {
                    println!("Использование: done <id>");
                }
            }
            "remove" | "rm" => {
                if let Ok(id) = args.parse::<u32>() {
                    remove_task(id);
                } else {
                    println!("Использование: remove <id>");
                }
            }
            "quit" | "q" | "exit" => {
                println!("До встречи! 👋");
                break;
            }
            _ => println!("Неизвестная команда: '{}'. Введите 'help'.", command),
        }
    }
}

fn show_help() {
    println!("Команды:");
    println!("  add, a <текст>   — добавить задачу");
    println!("  list, ls         — показать все задачи");
    println!("  done, d <id>     — отметить выполненной");
    println!("  remove, rm <id>  — удалить задачу");
    println!("  quit, q, exit    — выход");
}

fn add_task(text: &str) {
    println!("➕ Добавлена задача: '{}'", text);
}

fn list_tasks() {
    let tasks = [
        (1, "Изучить Rust", false),
        (2, "Написать crabstore", false),
        (3, "Купить молоко", true),
    ];

    if tasks.is_empty() {
        println!("Задач нет. Добавьте первую: add <текст>");
        return;
    }

    println!("\n📋 Список задач:");
    println!("{}", "-".repeat(40));

    for (id, text, done) in tasks {
        let status = if done { "✓" } else { "○" };
        let style = if done { "(выполнено)" } else { "" };
        println!("[{}] #{} {} {}", status, id, text, style);
    }

    println!("{}", "-".repeat(40));

    let done_count = tasks.iter().filter(|(_, _, done)| *done).count();
    println!("Всего: {} | Выполнено: {}\n", tasks.len(), done_count);
}

fn complete_task(id: u32) {
    println!("✓ Задача #{} отмечена выполненной", id);
}

fn remove_task(id: u32) {
    println!("🗑 Задача #{} удалена", id);
}
