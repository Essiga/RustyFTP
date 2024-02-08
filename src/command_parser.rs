use std::io::stdin;

#[derive(Debug)]
pub(crate) enum Command {
    LS,
    GET,
    MGET,
    NONE,
}

pub(crate) fn determine_command(input: String) -> Command {
    let filtered_string = remove_line_breaks(input);
    let parts: Vec<&str> = filtered_string.split(" ").collect();

    match parts[0] {
        "ls" => Command::LS,
        "get" => Command::GET,
        "mget" => Command::MGET,
        _ => Command::NONE
    }
}

fn remove_line_breaks(input: String) -> String {
    return input.replace("\r\n", "");
}