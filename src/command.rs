use std::error::Error;
use std::process::{Command, Output};

pub fn exec(c: String, args: Vec<String>) -> Result<Output, Box<dyn Error>> {
    let output = Command::new(c).args(args).output()?;
    Ok(output)
}

#[cfg(test)]
mod tests {
    use crate::command::exec;

    #[test]
    fn exec_ls() {
        let output = exec(String::from("ls"), vec![]).unwrap();
        assert_eq!(0, output.status.code().unwrap());
        let lines = String::from_utf8(output.stdout).unwrap();
        assert_eq!(true, lines.contains("src"))
    }

    #[test]
    fn exec_error() {
        let output = exec(String::from("aaa"), vec![]);
        if let Err(e) = output {
            let err_msg = e.to_string();
            err_msg.contains("No such file or directory");
        }
    }
}
