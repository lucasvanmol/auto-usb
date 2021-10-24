use std::{fs, io::{self, Error, ErrorKind}, process::Command, thread::sleep, time::Duration};

use bindings::{Windows::Win32::Foundation::RECT, Windows::Win32::{Foundation::{BOOL, LPARAM}, Graphics::Gdi::{EnumDisplayMonitors, HDC, HMONITOR}}, Windows::Win32::Storage::FileSystem::GetLogicalDrives};

// Time period between drive polling in ms
const DRIVE_POLLING_MS: u32 = 50;

fn main() {
    let monitors;
    unsafe {
        monitors = get_monitors().expect("winapi: error detecting monitors");
    }

    println!("Detected {} monitors", monitors.len());
    loop {
        let letter: char;
        unsafe {
            match get_new_drive() {
                Ok(l) => letter = l,
                Err(err) => {
                    eprintln!("{}", err);
                    continue;
                },
            }
        }
        println!("Found drive {:?}", letter);
        
        for monitor in &monitors {
            match open_html(letter, monitor) {
                Ok(_) => {},
                Err(err) => eprintln!("Error opening file: {}", err),
            };
        }
    }
}

// https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enumdisplaymonitors
unsafe fn get_monitors() -> io::Result<Vec<RECT>> {
    // Clipping rectangle
    let lprclip = RECT {
        left: i32::MIN,
        top: i32::MIN,
        right: i32::MAX,
        bottom: i32::MAX,
    };

    // Pass in raw pointer (as isize) to a Vec<RECT>
    // to keep track of all monitor positions
    let monitors: Vec<RECT> = Vec::new();
    let ptr = &monitors as *const Vec<RECT> as isize;

    let ret = EnumDisplayMonitors(
        None, 
        &lprclip, 
    Some(monitor_cb), 
        LPARAM(ptr)
    );
    
    if ret == BOOL(0) {
        return Err(Error::from(ErrorKind::Other));
    }
    return Ok(monitors);
}

// https://docs.microsoft.com/en-us/windows/win32/api/winuser/nc-winuser-monitorenumproc
unsafe extern "system" fn monitor_cb(_monitor: HMONITOR, _hdc: HDC, rect: *mut RECT, monitor_ptr: LPARAM) -> BOOL {
    // Dereference monitor list and push current one
    let monitors = &mut *(monitor_ptr.0 as *mut Vec<RECT>);
    monitors.push(*rect);
    
    // Return 1 to continue iteration
    BOOL(1)
}

fn open_html(drive: char, position: &RECT) -> io::Result<()> {
    for entry in  fs::read_dir(drive.to_string() + ":")? {
        match entry {
            Ok(entry) => {

                if let Some(ext) = entry.path().extension() {
                    if ext == "html" {
                        let p = entry.path();
                        println!("Opening {:?} at {}, {}", &p, position.left, position.top);

                        // Run using kiosk mode in Edge
                        Command::new("cmd")
                            .arg("/C")
                            .arg("start")
                            .arg("msedge") 
                            .arg(format!("--window-position={},{}", position.left, position.top))
                            .arg("--kiosk")
                            .arg(format!("--user-data-dir=%TEMP%\\auto-usb\\{},{}", position.left, position.top))
                            .arg(p)
                            .arg("--edge-kiosk-type=fullscreen")
                            .arg("--no-first-run")
                            .output()?;
                    }
                }
            },
            Err(err) => eprintln!("Error during directory traversal: {}", err),
        }
    }

    Ok(())
}

unsafe fn get_new_drive() -> io::Result<char> {
    let mut drives = GetLogicalDrives();
    println!("Listening for new drives...");
    loop {
        sleep(Duration::new(0, DRIVE_POLLING_MS * 10^6));
        let d = GetLogicalDrives();
        if d > drives {
            let mask = d ^ drives;
            // Return one drive letter even if two drives are inserted at same time
            return get_drive_letter(mask);
        } else if d < drives {
            drives = d;
        }
    }  
}

const LETTERS: [char; 26] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];
fn get_drive_letter(bitmask: u32) -> io::Result<char> {
    let index = bitmask.trailing_zeros() as usize;
    if LETTERS.len() > index {
        Ok(LETTERS[index])
    } else {
        Err(Error::from(ErrorKind::InvalidData))
    }
}