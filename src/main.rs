/**
 * Application:  concept.exe
 *
 * Created by:   VPR
 * Created:      February 10, 2024
 *
 * Updated by:   VPR
 * Updated:      February 12, 2024
 *
 * Description: This is a concept for malware written in rust for security
 *              research & reverse engineering purposes.
 *
 *              It CAN move itself into the user's AppData/Roaming and change
 *              it's file attributes to hidden, so if you execute on yourself,
 *              that's where you will end up finding it.
 *
 * Disclaimer:  I won't take any responsibility for any crimes or malicious
 *              activity used with either fragments or sections of this source
 *              code. Don't be an idiot.
**/

use std::env;
use std::path::{ Path, PathBuf };
use std::process::{ Command, Stdio };

fn is_in_target_path() -> bool {
    true
}

fn is_connected_to_server() -> bool {
    true
}

fn is_temp_file() {

}

fn move_self() {

}

fn rerun_self(new_path: &String) {
    Command::new(new_path)
        .args(["acbdefg"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("failed to execute");

    std::process::exit(0)
}

fn send_notification() {

}

fn hide_file(string: &PathBuf) {
    println!("Hiding file: {}", string.display());
}

fn main() {
    let current_exe_path: PathBuf = std::env::current_exe().unwrap();

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        hide_file(&current_exe_path);
        move_self();

        rerun_self(&args[0]);
    }

    if is_in_target_path() && is_connected_to_server() {
        send_notification();
    }

    println!("re-ran as: {} {}", current_exe_path.display(), args[1]);
}