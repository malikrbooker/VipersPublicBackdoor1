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
