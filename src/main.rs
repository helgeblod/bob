extern crate clap;

use clap::{Parser, Subcommand};
use duct::cmd;
use pathsearch::find_executable_in_path;

#[derive(Parser)]
#[command(author, version, about = "Build command shortcuts", long_about = "Utility for running build commands for different build systems")]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Build code ⚒️
    Build {},

    /// Clean artifacts 🧹
    Clean {},

    /// Run code 🚀
    Run {},

    /// Run tests 🧪
    Test {},
}

fn main() {
    let cli = Cli::parse();
    let languages = add_languages();

    match &cli.command {
        Commands::Build {} => {
            exec_build(&languages);
        }
        Commands::Clean {} => {
            exec_clean(&languages);
        }
        Commands::Run {} => {
            exec_run(&languages);
        }
        Commands::Test {} => {
            exec_test(languages);
        }
    }
}

fn exec_test(languages: Vec<Language>) {
    for language in languages.iter() {
        if language.current_language() {
            system_run(&language.heading, &language.main_command, &language.test_command, language.lookup_full_path);
            break;
        }
    }
}


fn exec_run(languages: &Vec<Language>) {
    for language in languages.iter() {
        if language.current_language() {
            system_run(&language.heading, &language.main_command, &language.run_command, language.lookup_full_path);
            break;
        }
    }
}

fn exec_clean(languages: &Vec<Language>) {
    for language in languages.iter() {
        if language.current_language() {
            system_run(&language.heading, &language.main_command, &language.clean_command, language.lookup_full_path);
            break;
        }
    }
}

fn exec_build(languages: &Vec<Language>) {
    for language in languages.iter() {
        if language.current_language() {
            system_run(&language.heading, &language.main_command, &language.build_command, language.lookup_full_path);
            break;
        }
    }
}

fn system_run(heading: &String, command: &String, args: &String, detect_path: bool) {
    // Search path for command
    println!("{} {} {}", heading, command.clone(), args.clone());
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

fn add_languages() -> Vec<Language> {
    let mut languages: Vec<Language> = vec![];

    // Rust
    languages.push(Language {
        detect_file: "Cargo.toml".to_string(),
        main_command: "cargo".to_string(),
        build_command: "build".to_string(),
        clean_command: "clean".to_string(),
        run_command: "run".to_string(),
        test_command: "test".to_string(),
        heading: "🦀 Rust".to_string(),
        lookup_full_path: true,
    });

    // If BOB_YARN is set, add yarn
    if std::env::var("BOB_USE_YARN").is_ok() {
        languages.push(Language {
            detect_file: "package.json".to_string(),
            main_command: "yarn".to_string(),
            build_command: "build".to_string(),
            clean_command: "clean".to_string(),
            run_command: "start".to_string(),
            test_command: "test".to_string(),
            heading: "🧶 Yarn".to_string(),
            lookup_full_path: true,
        });
    } else {
        languages.push(Language {
            detect_file: "package.json".to_string(),
            main_command: "npm".to_string(),
            build_command: "run build".to_string(),
            clean_command: "run clean".to_string(),
            run_command: "run start".to_string(),
            test_command: "run test".to_string(),
            heading: "📦 NPM".to_string(),
            lookup_full_path: true,
        });
    }

    // If gradle wrapper is present, add gradle
    languages.push(Language {
        detect_file: "gradlew".to_string(),
        main_command: "./gradlew".to_string(),
        build_command: "build".to_string(),
        clean_command: "clean".to_string(),
        run_command: "run".to_string(),
        test_command: "test".to_string(),
        heading: "🎁 Gradle Wrapper".to_string(),
        lookup_full_path: false,
    });

    // gradle build
    languages.push(Language {
        detect_file: "build.gradle.kts".to_string(),
        main_command: "gradle".to_string(),
        build_command: "build".to_string(),
        clean_command: "clean".to_string(),
        run_command: "run".to_string(),
        test_command: "test".to_string(),
        heading: "🤖 Gradle".to_string(),
        lookup_full_path: true,
    });

    languages
}

struct Language {
    detect_file: String,
    main_command: String,
    build_command: String,
    clean_command: String,
    run_command: String,
    test_command: String,
    heading: String,
    lookup_full_path: bool,
}

trait LanguageDetector {
    fn current_language(&self) -> bool;
}

impl LanguageDetector for Language {
    fn current_language(&self) -> bool {
        std::path::Path::new(&self.detect_file).exists()
    }
}
