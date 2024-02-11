/**
 * Application:  concept.exe
 *
 * Created by:   VPR
 * Created:      February 10, 2024
 *
 * Updated by:   VPR
 * Updated:      February 10, 2024
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

fn is_in_target_path() {

}

fn is_connected_to_server() {

}

fn is_temp_file() {

}

fn move_self() {

}

fn rerun_self() {
    std::process::exit(1);
}

fn send_notification() {

}

fn hide_file(string: &String) {
    println!("Hiding file: {}", string);
}

fn main() {
    // Collect the command line arguments into a vector of strings
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        hide_file(&args[0]);

        rerun_self();
    }

    println!("re-ran as: {}", args[0]);
}
