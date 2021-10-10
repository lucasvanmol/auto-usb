use std::{fs, thread::sleep, time::Duration, process::Command};

use bindings::{
    Windows::Win32::Storage::FileSystem::GetLogicalDrives,
};

// Run in background as windows service
// https://crates.io/crates/windows-service

fn main() -> windows::Result<()> {
    let letter: char;
    unsafe {
        letter = get_new_drive().unwrap();    
    }
    println!("Found drive {:?}", letter);

    for entry in  fs::read_dir(letter.to_string() + ":").unwrap() {
        match entry {
            Ok(entry) => {
                println!("Found file {:?}", entry.path());

                if let Some(ext) = entry.path().extension() {
                    if ext == "html" {
                        let p = entry.path();
                        println!("Opening {:?}", &p);

                        // Run using kiosk mode in Edge
                        Command::new("cmd")
                            .arg("/C")
                            .arg("start")
                            .arg("msedge") 
                            .arg("--kiosk")
                            .arg(p)
                            .arg("--edge-kiosk-type=fullscreen")
                            .arg("--no-first-run")
                            .output()
                            .expect("failed to execute process");
                        
                        // Command::new("cmd")
                        //     .arg("/C")
                        //     .arg("C:/Program Files/Mozilla Firefox/firefox.exe")
                        //     .arg("-new-window")
                        //     .arg(p)
                        //     .arg("--kiosk")
                        //     .output()
                        //     .expect("failed to execute process");
                    }
                }
            },
            Err(_) => todo!(),
        }
    }
    
    
    Ok(())
}

unsafe fn get_new_drive() -> Option<char> {
    let mut drives = GetLogicalDrives();
    println!("Listening for USB...");
    loop {
        sleep(Duration::new(0, 500 * 10^6));
        let d = GetLogicalDrives();
        if d > drives {
            let mask = d ^ drives;
            if mask.count_ones() == 1 {
                return get_drive_letter(mask);
            }
        } else if d < drives {
            drives = d;
        }
    }  
}

const LETTERS: [char; 26] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];
fn get_drive_letter(bitmask: u32) -> Option<char> {
    let index = bitmask.trailing_zeros() as usize;
    if LETTERS.len() > index {
        Some(LETTERS[index])
    } else {
        None
    }
}