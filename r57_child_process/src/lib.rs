
use std::process::Command;

pub fn execute() {
    let output = Command::new("rustc").arg("--version")
                          .output().unwrap_or_else(|e| {

       panic!("faild to execute process: {}", e)
    });

    if output.status.success() {
        let s = String::from_utf8_lossy(&output.stdout);
        println!("rustc succeeded and stdout was :\n{}", s);
    }
    else {
        let s = String::from_utf8_lossy(&output.stderr);
        println!("rustc faild and stderr was:\n {}", s);
    }

}