use core::panic;
use std::{
    io::stdin,
    process::{Command, Stdio},
};

fn main() {
    println!("Welcome to auto reflector!");
    backup_mirror_list();
    grab_mirror_list();
}

fn backup_mirror_list() {
    let command = "sudo cp /etc/pacman.d/mirrorlist /etc/pacman.d/mirrorlist.backup";
    let mut child = Command::new(command)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to execute command");
    let status = child.wait().expect("Failed to wait for command");
    if status.success() {
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

fn grab_mirror_list() {
    let command = "sudo reflector --verbose --latest 10 --protocol https --sort rate --save /etc/pacman.d/mirrorlist";
    let output = Command::new(command)
        .output()
        .expect("Failed to execute reflector command...");
    if !output.status.success() {
        println!("Failed to retrieve mirror list");
    }
    println!("{}", String::from_utf8_lossy(&output.stdout));
}
