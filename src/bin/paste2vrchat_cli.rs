use std::collections::HashMap;
use std::ffi::OsStr;
use std::iter::once;
use std::os::windows::ffi::OsStrExt;
use std::ptr::{null};
use clipboard_win::{formats, set_clipboard};
use winapi::{
    um::winuser::{FindWindowW, SetForegroundWindow, keybd_event, KEYEVENTF_KEYUP, VK_LCONTROL},
};
use winreg::{
    RegKey,
    enums::{HKEY_CURRENT_USER},
};

const REG_PATH: &str = "Software\\Classes\\p2vrc";
const COMMAND_PATH: &str = "Software\\Classes\\p2vrc\\shell\\open\\command";

struct Args {
    enter: bool,
    open_url: String,
    text: String,
}

fn parse_args() -> Result<Args, lexopt::Error> {
    use lexopt::prelude::{Long, Short, Value, ValueExt};
    let mut enter = false;
    let mut open_url = String::new();
    let mut text = String::new();
    let mut parser = lexopt::Parser::from_env();
    while let Some(arg) = parser.next()? {
        match arg {
            Long("install") => install(),
            Long("uninstall") => uninstall(),
            Long("enter") => { enter = true; }
            Long("open-url") | Short('o') => { open_url = parser.value()?.parse()?; }
            Value(val) if text.is_empty() => {
                text = match val.into_string() {
                    Ok(t) => t,
                    Err(_) => text,
                }
            }
            _ => return Err(arg.unexpected()),
        }
    }
    Ok(Args { enter, open_url, text })
}

fn main() -> Result<(), lexopt::Error> {
    let mut args = parse_args()?;

    if !args.text.is_empty() {
        match set_clipboard(formats::Unicode, args.text.as_str()) {
            Ok(_) => {
                switch_to_window("", "VRChat");
                paste(args.enter)
            }
            Err(_) => ()
        }
    } else if !args.open_url.is_empty() {
        let params = get_params(args.open_url.as_str());
        if let Some(enter) = params.get("enter") {
            args.enter = enter.eq("true");
        }
        if let Some(text) = params.get("text") {
            if let Ok(t) = urlencoding::decode(text) {
                match set_clipboard(formats::Unicode, t.to_string().as_str()) {
                    Ok(_) => {
                        switch_to_window("", "VRChat");
                        paste(args.enter)
                    }
                    Err(_) => ()
                }
            }
        }
    }
    Ok(())
}

#[cfg(windows)]
fn paste(enter: bool) {
    unsafe {
        keybd_event(VK_LCONTROL as u8, 0, 0, 0);
        keybd_event(0x56, 0, 0, 0);
        keybd_event(0x56, 0, KEYEVENTF_KEYUP, 0);
        keybd_event(VK_LCONTROL as u8, 0, KEYEVENTF_KEYUP, 0);
        if enter {
            keybd_event(0x0D, 0, 0, 0);
            keybd_event(0x0D, 0, KEYEVENTF_KEYUP, 0);
        }
    }
}

#[cfg(windows)]
fn install() {
    let current_user = RegKey::predef(HKEY_CURRENT_USER);
    if let Ok((key, _)) = current_user.create_subkey(REG_PATH) {
        let _ = key.set_value("", &"URL:p2vrc");
        let _ = key.set_value("URL Protocol", &"");
    }
    if let Ok((key, _)) = current_user.create_subkey(COMMAND_PATH) {
        if let Ok(exe_path) = std::env::current_exe() {
            let _ = key.set_value(
                "",
                &format!("\"{}\" --open-url \"%1\"", exe_path.display()),
            );
        }
    }
}

#[cfg(windows)]
fn uninstall() {
    let _ = RegKey::predef(HKEY_CURRENT_USER)
        .delete_subkey_all(REG_PATH);
}

fn get_params(u: &str) -> HashMap<String, String> {
    let u: &str = match u.find('?') {
        Some(index) => &u[index + 1..],
        None => u,
    };
    u.split('&')
        .map(|s| s.split('=').collect::<Vec<&str>>())
        .map(|v| (v[0].to_string(), v[1].to_string()))
        .collect()
}

#[cfg(windows)]
fn win32_string(value: &str) -> Vec<u16> {
    OsStr::new(value).encode_wide().chain(once(0)).collect()
}

#[cfg(windows)]
fn switch_to_window(class_name: &str, title_name: &str) {
    let class_str = if class_name.is_empty() { null() } else { win32_string(class_name).as_ptr() };
    let title_str = if title_name.is_empty() { null() } else { win32_string(title_name).as_ptr() };
    let win = unsafe { FindWindowW(class_str, title_str) };
    if !win.is_null() { unsafe { SetForegroundWindow(win) }; }
}