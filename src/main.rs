use std::{fs, io, process::Command, thread::sleep, time::Duration};

use bindings::{
    Windows::Win32::Storage::FileSystem::GetLogicalDrives,
};

// Run in background as windows service
// https://crates.io/crates/windows-service

fn main() {
    loop {
        let letter: char;
        unsafe {
            letter = get_new_drive().unwrap();    
        }
        println!("Found drive {:?}", letter);
        
        match open_html(letter) {
            Ok(_) => {},
            Err(err) => eprintln!("Error opening file: {}", err),
        };
    }
}

fn open_html(drive: char) -> io::Result<()> {
    for entry in  fs::read_dir(drive.to_string() + ":")? {
        match entry {
            Ok(entry) => {

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
                            .output()?;
                        
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
            Err(err) => eprintln!("Error during directory traversal: {}", err),
        }
    }

    Ok(())
}

unsafe fn get_new_drive() -> Option<char> {
    let mut drives = GetLogicalDrives();
    println!("Listening for new drives...");
    loop {
        sleep(Duration::new(0, 500 * 10^6));
        let d = GetLogicalDrives();
        if d > drives {
            let mask = d ^ drives;
            // Return one drive letter even if two drives are inserted at same times
            return get_drive_letter(mask);
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