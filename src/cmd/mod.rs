use pathsearch::find_executable_in_path;
use duct::cmd;
use crate::languages::{Language, LanguageDetector};

pub fn exec_test(languages: Vec<Language>) {
    for language in languages.iter() {
        if language.current_language() {
            run_cmd(&language.heading, &language.main_command, &language.test_command, language.lookup_full_path);
            break;
        }
    }
}


pub fn exec_run(languages: &Vec<Language>) {
    for language in languages.iter() {
        if language.current_language() {
            run_cmd(&language.heading, &language.main_command, &language.run_command, language.lookup_full_path);
            break;
        }
    }
}

pub fn exec_clean(languages: &Vec<Language>) {
    for language in languages.iter() {
        if language.current_language() {
            run_cmd(&language.heading, &language.main_command, &language.clean_command, language.lookup_full_path);
            break;
        }
    }
}

pub fn exec_build(languages: &Vec<Language>) {
    for language in languages.iter() {
        if language.current_language() {
            run_cmd(&language.heading, &language.main_command, &language.build_command, language.lookup_full_path);
            break;
        }
    }
}

fn run_cmd(heading: &String, command: &String, args: &String, detect_path: bool) {
    // Search path for command
    println!("{} ⚙️ '{} {}'", heading, command.clone(), args.clone());
    if detect_path {
        match find_executable_in_path(&command.clone()) {
            None => {
                println!("Command not found: {:}", command.clone());
                return;
            }
            Some(exe) => {
                cmd!(exe.to_str().expect("Unable to unwrap executable"), args).run().expect("Failed to execute command");
            }
        };
    } else {
        cmd!(command, args).run().expect("Failed to execute command");
    };
}
