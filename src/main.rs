extern crate clap;

use std::process::Command;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about="Build command shortcuts", long_about = "Utility for running build commands for different build systems")]
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
            system_run(&language.heading, &language.test_command);
            break;
        }
    }
}


fn exec_run(languages: &Vec<Language>) {
    for language in languages.iter() {
        if language.current_language() {
            system_run(&language.heading, &language.run_command);
            break;
        }
    }
}

fn exec_clean(languages: &Vec<Language>) {
    for language in languages.iter() {
        if language.current_language() {
            system_run(&language.heading, &language.clean_command);
            break;
        }
    }
}

fn exec_build(languages: &Vec<Language>) {
    for language in languages.iter() {
        if language.current_language() {
            system_run(&language.heading, &language.build_command);
            break;
        }
    }
}

fn system_run(heading: &String, command: &String) {
    println!("{} ({})", heading, command.clone());
    Command::new(command.as_str()).spawn().expect("😿 Failed to run command");

}

fn add_languages() -> Vec<Language> {
    let mut languages: Vec<Language> = vec![];

    // Rust
    languages.push(Language {
        detect_file: "Cargo.toml".to_string(),
        build_command: "cargo build".to_string(),
        clean_command: "cargo clean".to_string(),
        run_command: "cargo run".to_string(),
        test_command: "cargo test".to_string(),
        heading: "🦀 Rust".to_string(),
    });

    // If BOB_YARN is set, add yarn
    if std::env::var("BOB_USE_YARN").is_ok() {
        languages.push(Language {
            detect_file: "package.json".to_string(),
            build_command: "yarn build".to_string(),
            clean_command: "yarn clean".to_string(),
            run_command: "yarn start".to_string(),
            test_command: "yarn test".to_string(),
            heading: "🧶 Yarn".to_string(),
        });
    } else {
        languages.push(Language {
            detect_file: "package.json".to_string(),
            build_command: "npm run build".to_string(),
            clean_command: "npm run clean".to_string(),
            run_command: "npm run start".to_string(),
            test_command: "npm run test".to_string(),
            heading: "📦 NPM".to_string(),
        });
    }

    // If gradle wrapper is present, add gradle
    languages.push(Language {
        detect_file: "gradlew".to_string(),
        build_command: "./gradlew build".to_string(),
        clean_command: "./gradlew clean".to_string(),
        run_command: "./gradlew run".to_string(),
        test_command: "./gradlew test".to_string(),
        heading: "🎁 Gradle Wrapper".to_string(),
    });

    // gradle build
    languages.push(Language {
        detect_file: "build.gradle.kts".to_string(),
        build_command: "gradle build".to_string(),
        clean_command: "gradle clean".to_string(),
        run_command: "gradle run".to_string(),
        test_command: "gradle test".to_string(),
        heading: "🤖 Gradle".to_string(),
    });

    languages
}

struct Language {
    detect_file: String,
    build_command: String,
    clean_command: String,
    run_command: String,
    test_command: String,
    heading: String,
}

trait LanguageDetector {
    fn current_language(&self) -> bool;
}

impl LanguageDetector for Language {
    fn current_language(&self) -> bool {
        std::path::Path::new(&self.detect_file).exists()
    }
}
