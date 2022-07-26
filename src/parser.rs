use std::{env,fs,path::Path};
use regex::Regex;


/// Grab a String from 'input_text', by checking if 'check' is contained in
/// 'input_text', if so use the regex 'pattern' to parse 'input_text'.
/// eg.: 
/// ```
/// let text: String = "I love cats".to_string();
/// let mut result: String = regex_grab("I love", "love (\w+)", &text);
/// println!("{}", &result);
///
/// assert_eq!("cats", result);
/// ```
pub fn regex_grab(check: &str, pattern: &str, input_text: &String) -> String {
    let _re = Regex::new(pattern).unwrap();
    let mut _result: String = String::new();
    if input_text.contains(&check) {
        // ! DEBUG: Print the captures entirely
        //println!("{:?}", re.captures(&input_text));
        let mut _caps = _re.captures(&input_text).unwrap();
        _result = _caps.get(1).map_or("None", |m| m.as_str()).to_string();
    } else {
        _result = String::from("None");
    }
    return _result;
}

/// Check if a path exists, returns a bool
pub fn dir_exists(dir: &String) -> bool {
    return Path::new(dir).is_dir()
}


/// Returns a Vec<String> of directories for parsing applications
pub fn get_app_dirs() -> Vec<String> {
    let uhome: &str = env!("HOME");
    let _base_dirv: Vec<String> = vec![
        "/usr/share/applications".to_string(),
        "usr/local/share/applications".to_string(),
        "uhome/.local/share/applications".replace("uhome", uhome)
    ];
    let mut _existing_dirv: Vec<String> = Vec::new();
    for item in _base_dirv.into_iter().enumerate() {
        let (_index, _dir) = item;
        if dir_exists(&_dir) {
            _existing_dirv.push(_dir);
        }
    }
    return _existing_dirv
}


/// Returns a Vec<String> of applications, requires a Vec<String> with paths.
pub fn get_apps(app_dirv: Vec<String>) -> Vec<String> {
    let mut _app_vec: Vec<String> = Vec::new();
    for dir in app_dirv {
        // ! DEBUG: Print the paths
        //println!("{:?}", fs::read_dir(&dir).unwrap());
        let paths = fs::read_dir(dir).unwrap();
        for file in paths {
            // ! DEBUG: Print each file object
            //println!("{:?}", &file);
            _app_vec.push(
                file.unwrap()
                .path()
                .display()
                .to_string());
        }

    }
    _app_vec.sort_unstable();
    return _app_vec
}

/// Fancy print the 
pub fn fancy_print(
    _path:&String, _name: String, _gname: String, _cat: String, _cmd: String, _icon: String) {
    let info: String;
    if _gname == "None" {
        info = _cat;
    } else {
        info = _gname;
    }
    println!(
        "\x1b[0m{}\n\x1b[31m{}\x1b[0m:\x1b[32m{}\x1b[0m:\x1b[33m{}\x1b[0m:\x1b[34m{}\n",
        _path, _name, info, _cmd, _icon);
}
