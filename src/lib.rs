#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}


#[derive(Debug)]
pub struct Command {
    pub time: i64,
    pub cmd: String,
}

#[derive(Debug)]
pub struct ShellHistory {
    pub history: Vec<Command>,
    pub shell: Shell
}


#[derive(Debug)]
pub enum Shell {
    Fish,
    Bash
}