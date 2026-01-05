pub enum Command {
    ModeToUartChoice,
    ModeToRateChoice,
    Open,
    Quit,
}

pub struct ParsedCommand {
    pub name: String,      // 命令名，如 "help"
    pub args: Vec<String>, // 参数列表，如 ["file.txt", "--verbose"]
}

fn tokenize(input: &str) -> Result<Vec<String>, String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            '"' if !in_quotes => in_quotes = true,
            '"' if in_quotes => in_quotes = false,
            ' ' if !in_quotes => {
                if !current.is_empty() {
                    tokens.push(std::mem::take(&mut current));
                }
            }
            '\\' if chars.peek() == Some(&'"') || chars.peek() == Some(&'\\') => {
                current.push(chars.next().unwrap()); // 转义
            }
            _ => current.push(ch),
        }
    }

    if !current.is_empty() {
        tokens.push(current);
    }

    if in_quotes {
        return Err("Unclosed quote".to_string());
    }

    Ok(tokens)
}

pub fn parse_command(input: &str) -> Result<Command, String> {
    if input.trim().is_empty() {
        return Err("Empty command".to_string());
    }

    let tokens = tokenize(input)?;
    if tokens.is_empty() {
        return Err("No command found".to_string());
    }

    let mut tokens = tokens.into_iter();
    let name = tokens.next().unwrap(); // safe after is_empty() check

    let command = ParsedCommand {
        name,
        args: tokens.collect(),
    };
    dispatch(command)
}

fn dispatch(cmd: ParsedCommand) -> Result<Command, String> {
    match cmd.name.as_str() {
        "c" => Ok(Command::ModeToUartChoice),
        "q" => Ok(Command::Quit),
        "r" => Ok(Command::ModeToRateChoice),
        "o" => Ok(Command::Open),
        _ => Err(format!("Unknown command: {}", cmd.name)),
    }
}
