use std::io::{self, Write};

const VERSION: &str = "0.1.0";

// Структура владеет своими данными
struct Task {
    id: u32,
    text: String, // Task владеет строкой
    done: bool,
}

impl Task {
    // Конструктор — принимает владение строкой
    fn new(id: u32, text: String) -> Self {
        Task {
            id,
            text,
            done: false,
        }
    }

    // Метод — заимствует self неизменяемо
    fn display(&self) {
        let status = if self.done { "✓" } else { "○" };
        println!("[{}] #{} {}", status, self.id, self.text);
    }

    // Метод — заимствует self изменяемо
    fn complete(&mut self) {
        self.done = true;
    }
}

fn main() {
    // Vec владеет задачами, задачи владеют своими строками
    let mut tasks: Vec<Task> = Vec::new();
    let mut next_id: u32 = 1;

    println!("🦀 crabstore v{}", VERSION);
    println!("Введите 'help' для списка команд\n");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            continue;
        }

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        let parts: Vec<&str> = input.splitn(2, ' ').collect();
        let command = parts[0];
        let args = if parts.len() > 1 { parts[1] } else { "" };

        match command {
            "help" | "h" => show_help(),

            "add" | "a" => {
                if args.is_empty() {
                    println!("Использование: add <текст>");
                } else {
                    // args — это &str, создаём owned String
                    let task = Task::new(next_id, args.to_string());
                    println!("➕ Добавлена: #{} '{}'", task.id, task.text);
                    tasks.push(task); // task перемещается в вектор
                    next_id += 1;
                }
            }

            "list" | "ls" => {
                if tasks.is_empty() {
                    println!("Задач нет. Добавьте: add <текст>");
                } else {
                    println!("\n📋 Задачи:");
                    // &task — заимствуем, не забираем из вектора
                    for task in &tasks {
                        task.display();
                    }
                    let done = tasks.iter().filter(|t| t.done).count();
                    println!("Всего: {} | Выполнено: {}\n", tasks.len(), done);
                }
            }

            "done" | "d" => {
                if let Ok(id) = args.parse::<u32>() {
                    // &mut task — изменяемое заимствование
                    if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
                        task.complete();
                        println!("✓ Задача #{} выполнена", id);
                    } else {
                        println!("Задача #{} не найдена", id);
                    }
                } else {
                    println!("Использование: done <id>");
                }
            }

            "remove" | "rm" => {
                if let Ok(id) = args.parse::<u32>() {
                    if let Some(pos) = tasks.iter().position(|t| t.id == id) {
                        let removed = tasks.remove(pos); // забираем владение
                        println!("🗑 Удалена: #{} '{}'", removed.id, removed.text);
                        // removed уничтожается здесь
                    } else {
                        println!("Задача #{} не найдена", id);
                    }
                } else {
                    println!("Использование: remove <id>");
                }
            }

            "quit" | "q" | "exit" => {
                println!("До встречи! 👋");
                break;
            }

            _ => println!("Неизвестная команда. Введите 'help'."),
        }
    }
} // tasks уничтожается, все Task уничтожаются, вся память освобождается

fn show_help() {
    println!("Команды:");
    println!("  add, a <текст>   — добавить задачу");
    println!("  list, ls         — показать задачи");
    println!("  done, d <id>     — отметить выполненной");
    println!("  remove, rm <id>  — удалить задачу");
    println!("  quit, q          — выход");
}
