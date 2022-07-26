use std::fs;
mod helper;
mod parser;


fn main() {
    // .DESKTOP FILE LAUNCHER
    helper::debug("Starting");
    // 1: Check OS
    helper::debug("Checking OS");
    // TODO: Add Windows Support
    if std::env::consts::OS != "linux" {
        helper::terminate("Application must run on linux", 1);
    }
    // 2: Check if the folders exist and add them to a list of some sort
    // then, check for applications contained inside those folders.
    helper::debug("Checking applications");
    let dirs = parser::get_app_dirs();
    let apps = parser::get_apps(dirs);
    // 5: Get their attributes through REGEX
    for path in apps {
        let mut path_c: String = String::new();
        if parser::dir_exists(&path) == false {
            path_c = fs::read_to_string(&path).
                expect("Unable to read file");
        }
        // Start parsing from here
        let name: String;
        let gname: String;
        let cat: String;
        let cmd: String;
        let icon: String;
        if ! path_c.contains("NoDisplay=true") {
            if path_c.contains("[Desktop Entry]") && path_c.contains("Type=Application") {
                name = parser::regex_grab(
                    "Name=", r"Name=(.*)", &path_c);
                gname = parser::regex_grab(
                    "GenericName", "GenericName=(.*)", &path_c);
                cat = parser::regex_grab(
                    "Categories=", r"Categories=(\w+)", &path_c);
                cmd = parser::regex_grab(
                    "Exec=", r"Exec=(.*)", &path_c);
                icon = parser::regex_grab(
                    "Icon=", "Icon=(.*)", &path_c);
                parser::fancy_print(&path, name, gname, cat, cmd, icon);
            }
        }
    }
    // TODO: Add these steps ↓
    // 6: Initialize the program launcher window
    // 7: Execute the user selection quietly
}
