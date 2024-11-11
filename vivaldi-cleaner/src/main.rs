use mouse_keyboard_input::key_codes::*;
use mouse_keyboard_input::VirtualDevice;
use std::io::Read;
use std::{thread, time::Duration};
use wl_clipboard_rs::paste::{get_contents, ClipboardType, Error, MimeType, Seat};

fn remove_all_workspaces() {
    let mut device = VirtualDevice::default().unwrap();

    thread::sleep(Duration::from_secs(5));

    for _ in 0..100 {
        thread::sleep(Duration::from_millis(500));
        device.click(BTN_LEFT).unwrap();

        thread::sleep(Duration::from_millis(500));
        device.click(KEY_TAB).unwrap();

        thread::sleep(Duration::from_millis(500));
        device.press(KEY_LEFTSHIFT).unwrap();
        thread::sleep(Duration::from_millis(500));
        device.click(KEY_F10).unwrap();
        thread::sleep(Duration::from_millis(500));
        device.release(KEY_LEFTSHIFT).unwrap();

        thread::sleep(Duration::from_millis(500));
        device.press(KEY_LEFTSHIFT).unwrap();
        thread::sleep(Duration::from_millis(500));
        device.click(KEY_D).unwrap();
        thread::sleep(Duration::from_millis(500));
        device.release(KEY_LEFTSHIFT).unwrap();

        thread::sleep(Duration::from_millis(500));
        device.click(KEY_TAB).unwrap();

        thread::sleep(Duration::from_millis(500));
        device.click(KEY_ENTER).unwrap();

        thread::sleep(Duration::from_millis(500));
    }
}

fn main() {
    remove_all_workspaces();
}
