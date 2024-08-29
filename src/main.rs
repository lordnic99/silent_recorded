#![windows_subsystem = "windows"]

use std::fs::File;
use std::os::windows::process::CommandExt;
use std::process::Command;
use std::thread;
use std::time::Duration;
use winapi::um::winbase::CREATE_NO_WINDOW;

fn main() {
    let control_file_path = "DANG_QUAY_MAN_HINH";
    let _ = File::create(control_file_path).expect("Failed to create control file");
    let mut command = Command::new("record_check")
        .args(&[
            "core",
            "-y",
            "-f",
            "gdigrab",
            "-framerate",
            "30",
            "-i",
            "desktop",
            "recorded.mkv",
        ])
        .creation_flags(CREATE_NO_WINDOW)
        .spawn()
        .expect("Failed to start core");

    loop {
        if !std::path::Path::new(control_file_path).exists() {
            break;
        }
        thread::sleep(Duration::from_secs(1));
    }
    let pid = command.id();

    let _ = Command::new("taskkill")
        .args(&["/PID", &pid.to_string()])
        .creation_flags(CREATE_NO_WINDOW)
        .output()
        .expect("Failed to execute taskkill");

    // Wait for the process to exit
    let _ = command.wait().expect("Failed to wait on ffmpeg process");
}
