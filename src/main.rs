use core::panic;
use std::{
    env::current_dir,
    io::{stdin, BufRead, BufReader},
    process::{Command, Stdio},
};

fn main() {
    println!("Welcome to auto reflector!");
    backup_mirror_list();
    match grab_mirror_list() {
        Ok(_x) => println!("Updated mirror list successfully"),
        Err(x) => println!("{x}"),
    }
}

fn backup_mirror_list() {
    let command = "sudo";
    let arg = "/etc/pacman.d/mirrorlist";
    let arg2 = "/etc/pacman.d/mirrorlist.backup";
    let current_dir = current_dir().unwrap();
    let dir_path = current_dir.to_str().unwrap();
    let child = Command::new(command)
        .arg("cp")
        .arg(arg)
        .arg(arg2)
        .current_dir(dir_path)
        .output()
        .expect("Failed to execute command");
    if child.status.success() {
        println!("Mirror list backup successful");
    } else {
        println!("Could not backup mirror list. Continue anyway? (Y/n)");
        loop {
            let mut input = String::new();
            stdin()
                .read_line(&mut input)
                .expect("Failed to read command");
            let input = input.trim();
            if input.to_lowercase() == String::from("y") {
                return;
            } else if input.to_lowercase() == String::from("n") {
                panic!("Exiting script");
            } else {
                println!("Invalid command...");
            }
        }
    }
}

fn grab_mirror_list() -> std::io::Result<()> {
    let command = "sudo";
    let mut output = Command::new(command)
        .arg("reflector")
        .arg("--verbose")
        .arg("--latest")
        .arg("10")
        .arg("--protocol")
        .arg("https")
        .arg("--sort")
        .arg("rate")
        .arg("--save")
        .arg("/etc/pacman.d/mirrorlist")
        .stdout(Stdio::piped())
        .spawn()?;
    let stdout = output.stdout.as_mut().expect("Failed to open stdout");
    let reader = BufReader::new(stdout);
    for line in reader.lines() {
        println!("{}", line?);
    }
    output.wait()?;
    Ok(())
}
