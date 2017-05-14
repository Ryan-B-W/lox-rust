extern crate libc;

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


#[cfg(target_os = "macos")]
pub fn get_parent_shell() -> String {
    extern crate libproc;

    let pid: i32;
    unsafe {
        pid = libc::getppid() as i32;
    }

    match libproc::libproc::proc_pid::name(pid) {
        Ok(v) => v,
        Err(_) => panic!("Unable to get parent process name")
    }
}

#[cfg(target_os = "linux")]
pub fn get_parent_shell() -> String {
    extern crate pentry;

    let pid: i32;
    unsafe {
        pid = libc::getppid() as i32;
    }

    if let Ok(ps) = pentry::find(pid) {
        let prog_option = ps.path().unwrap().split("/").collect::<Vec<&str>>();

        match prog_option.last() {
            Some(&v) => return v.to_owned(),
            _ => panic!("Unable to get shell name"),
        };
    } else {
        panic!("Unable to find shell PID")
    }
}
