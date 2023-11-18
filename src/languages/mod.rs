pub fn add_languages() -> Vec<Language> {
    let mut languages: Vec<Language> = vec![];

    // justfile for custom builds
    languages.push(Language {
        detect_file: ".justfile",
        main_command: "just",
        build_command: "build",
        clean_command: "clean",
        run_command: "run",
        test_command: "test",
        install_command: Some("install"),
        heading: "🤖 Justfile",
        lookup_full_path: true,
    });

    // justfile for custom builds
    languages.push(Language {
        detect_file: ".justfile",
        main_command: "just",
        build_command: "build",
        clean_command: "clean",
        run_command: "run",
        test_command: "test",
        install_command: Some("install"),
        heading: "🍥 Makefile",
        lookup_full_path: true,
    });

    // Rust
    languages.push(Language {
        detect_file: "Cargo.toml",
        main_command: "cargo",
        build_command: "build",
        clean_command: "clean",
        run_command: "run",
        test_command: "test",
        install_command: Some("install --path ."),
        heading: "🦀 Rust",
        lookup_full_path: true,
    });

    // If BOB_YARN is set, add yarn
    if std::env::var("BOB_USE_YARN").is_ok() {
        languages.push(Language {
            detect_file: "package.json",
            main_command: "yarn",
            build_command: "build",
            clean_command: "clean",
            run_command: "start",
            test_command: "test",
            install_command: Some("install"),
            heading: "🧶 Yarn",
            lookup_full_path: true,
        });
    } else {
        languages.push(Language {
            detect_file: "package.json",
            main_command: "npm",
            build_command: "run build",
            clean_command: "run clean",
            run_command: "run start",
            test_command: "run test",
            install_command: Some("install"),
            heading: "📦 NPM",
            lookup_full_path: true,
        });
    }

    // If gradle wrapper is present, add gradle
    languages.push(Language {
        detect_file: "gradlew",
        main_command: "./gradlew",
        build_command: "build",
        clean_command: "clean",
        run_command: "run",
        test_command: "test",
        install_command: None,
        heading: "🎁 Gradle Wrapper",
        lookup_full_path: false,
    });

    // gradle build
    languages.push(Language {
        detect_file: "build.gradle.kts",
        main_command: "gradle",
        build_command: "build",
        clean_command: "clean",
        run_command: "run",
        test_command: "test",
        install_command: None,
        heading: "🔩 Gradle",
        lookup_full_path: true,
    });



    languages
}

pub trait LanguageDetector {
    fn current_language(&self) -> bool;
}

pub struct Language {
    pub detect_file: &'static str,
    pub main_command: &'static str,
    pub build_command: &'static str,
    pub clean_command: &'static str,
    pub run_command: &'static str,
    pub test_command: &'static str,
    pub install_command: Option<&'static str>,
    pub heading: &'static str,
    pub lookup_full_path: bool,
}

impl LanguageDetector for Language {
    fn current_language(&self) -> bool {
        std::path::Path::new(&self.detect_file).exists()
    }
}
