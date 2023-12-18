// use std::path::Path;
use std::process::Command;

pub fn wgquick(args: Vec<&str>) -> Option<i32> {
    let output = Command::new("wg-quick")
        .args(args)
        .output()
        .expect("Unable to exec wg-quick");

    // println!("{:?}", output);

    match output.status.code() {
        Some(code) => eprintln!("Exited with status code: {code}"),
        None => eprintln!("Process terminated by signal"),
    }

    output.status.code()
}

pub fn active_wg(tun: &str) -> Option<i32> {
    return wgquick(vec!["up", tun]);
}

pub fn deactive_wg(tun: &str) -> Option<i32> {
    return wgquick(vec!["down", tun]);
}
