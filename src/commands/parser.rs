use super::{Command, ListFilter, ParseError};
use crate::task::Priority;

pub fn parse(input: &str) -> Result<Command, ParseError> {
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
            let filter = parse_list_filter(args)?;
            Ok(Command::List { filter })
        }

        "done" | "d" => {
            let id = parse_id(args)?;
            Ok(Command::Done { id })
        }

        "priority" | "p" => {
            let (id, level) = parse_priority_args(args)?;
            Ok(Command::Priority { id, level })
        }

        "tag" | "t" => {
            let (id, tag) = parse_tag_args(args)?;
            Ok(Command::Tag { id, tag })
        }

        "stats" => Ok(Command::Stats),

        "remove" | "rm" => {
            let id = parse_id(args)?;
            Ok(Command::Remove { id })
        }

        "quit" | "q" => Ok(Command::Quit),

        other => Err(ParseError::UnknownCommand(other.to_string())),
    }
}

fn parse_id(s: &str) -> Result<u32, ParseError> {
    s.trim().parse().map_err(|_| ParseError::InvalidId)
}

fn parse_list_filter(args: &str) -> Result<Option<ListFilter>, ParseError> {
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
        arg if arg.starts_with('#') => Some(ListFilter::Tag(arg[1..].to_string())),
        _ => None,
    };
    Ok(filter)
}

fn parse_priority_args(args: &str) -> Result<(u32, Priority), ParseError> {
    let parts: Vec<&str> = args.split_whitespace().collect();

    if parts.len() < 2 {
        return Err(ParseError::MissingArgument("id и приоритет"));
    }

    let id = parse_id(parts[0])?;
    let level = parts[1]
        .parse::<u8>()
        .ok()
        .and_then(Priority::from_u8)
        .ok_or(ParseError::InvalidPriority)?;

    Ok((id, level))
}

fn parse_tag_args(args: &str) -> Result<(u32, String), ParseError> {
    let parts: Vec<&str> = args.split_whitespace().collect();

    if parts.len() < 2 {
        return Err(ParseError::MissingArgument("id и тег"));
    }

    let id = parse_id(parts[0])?;
    let tag = parts[1].trim_start_matches('#').to_string();

    Ok((id, tag))
}
