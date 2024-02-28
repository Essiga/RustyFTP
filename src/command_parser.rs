#[derive(Debug)] //used for printing the enum using the {:?} format
pub(crate) enum Command {
    LS,
    GET,
    ASCII,
    BINARY,
    QUIT,
    NONE,
}

pub(crate) fn determine_command(input: String) -> Command {
    let filtered_string = remove_line_breaks(input);
    let parts: Vec<&str> = filtered_string.split(" ").collect();

    match parts[0] {
        "ls" => Command::LS,
        "get" => Command::GET,
        "ascii" => Command::ASCII,
        "binary" => Command::BINARY,
        "quit" => Command::QUIT,
        _ => Command::NONE
    }
}

pub(crate) fn determine_parameter(input: String) -> String {
    let filtered_string = remove_line_breaks(input);
    let parts: Vec<&str> = filtered_string.split(" ").collect();
    parts[1..].join(" ")
}

fn remove_line_breaks(input: String) -> String {
    return input.replace("\r\n", "");
}