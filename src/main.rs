use std::env;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file: &str = match args.len() > 1 {
        true => &args[1],
        false => panic!("no file was given"),
    };

    if cfg!(target_os = "windows") {
        Command::new("powershell")
            .args(&[format!("({}).LastWriteTime = Get-Date", file)])
            .status()
            .expect(&format!("Could not update mtime of {}", file));
    } else {
        Command::new("touch")
            .arg(format!("{}", file))
            .status()
            .expect(&format!("Could not update mtime of {}", file));
    };
}
