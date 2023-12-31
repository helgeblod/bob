use crate::languages::{Language, LanguageDetector};
use duct::cmd;
use pathsearch::find_executable_in_path;

pub fn exec_test(languages: Vec<Language>) {
    for language in languages.iter() {
        if language.current_language() {
            run_cmd(
                language.heading,
                language.main_command,
                language.test_command,
                language.lookup_full_path,
            );
            break;
        }
    }
}

pub fn exec_run(languages: &Vec<Language>) {
    for language in languages.iter() {
        if language.current_language() {
            run_cmd(
                language.heading,
                language.main_command,
                language.run_command,
                language.lookup_full_path,
            );
            break;
        }
    }
}

pub fn exec_clean(languages: &Vec<Language>) {
    for language in languages.iter() {
        if language.current_language() {
            run_cmd(
                language.heading,
                language.main_command,
                language.clean_command,
                language.lookup_full_path,
            );
            break;
        }
    }
}

pub fn exec_build(languages: &Vec<Language>) {
    for language in languages.iter() {
        if language.current_language() {
            run_cmd(
                language.heading,
                language.main_command,
                language.build_command,
                language.lookup_full_path,
            );
            break;
        }
    }
}

pub(crate) fn exec_install(languages: &Vec<Language>) {
    for language in languages.iter() {
        if language.current_language() {
            run_cmd(
                language.heading,
                language.main_command,
                language.install_command.expect("No install command found"),
                language.lookup_full_path,
            );
            break;
        }
    }
}

fn run_cmd(heading: &str, command: &str, args: &str, detect_path: bool) {
    // Search path for command
    println!("{} ⚙️ '{} {}'", heading, command, args);
    if detect_path {
        match find_executable_in_path(&command) {
            None => {
                println!("Command not found: {:}", command);
                return;
            }
            Some(exe) => {
                cmd!(exe.to_str().expect("Unable to unwrap executable"), args)
                    .run()
                    .expect("Failed to execute command");
            }
        };
    } else {
        cmd!(command, args)
            .run()
            .expect("Failed to execute command");
    };
}
