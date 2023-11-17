pub fn add_languages() -> Vec<Language> {
    let mut languages: Vec<Language> = vec![];

    // justfile for custom builds
    languages.push(Language {
        detect_file: ".justfile".to_string(),
        main_command: "just".to_string(),
        build_command: "build".to_string(),
        clean_command: "clean".to_string(),
        run_command: "run".to_string(),
        test_command: "test".to_string(),
        heading: "🤖Justfile".to_string(),
        lookup_full_path: true,
    });

    // justfile for custom builds
    languages.push(Language {
        detect_file: ".justfile".to_string(),
        main_command: "just".to_string(),
        build_command: "build".to_string(),
        clean_command: "clean".to_string(),
        run_command: "run".to_string(),
        test_command: "test".to_string(),
        heading: "🍥Makefile".to_string(),
        lookup_full_path: true,
    });

    // Rust
    languages.push(Language {
        detect_file: "Cargo.toml".to_string(),
        main_command: "cargo".to_string(),
        build_command: "build".to_string(),
        clean_command: "clean".to_string(),
        run_command: "run".to_string(),
        test_command: "test".to_string(),
        heading: "🦀Rust".to_string(),
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
            heading: "🧶Yarn".to_string(),
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
            heading: "📦NPM".to_string(),
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
        heading: "🎁Gradle Wrapper".to_string(),
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
        heading: "🔩Gradle".to_string(),
        lookup_full_path: true,
    });



    languages
}

pub trait LanguageDetector {
    fn current_language(&self) -> bool;
}

pub struct Language {
    pub detect_file: String,
    pub main_command: String,
    pub build_command: String,
    pub clean_command: String,
    pub run_command: String,
    pub test_command: String,
    pub heading: String,
    pub lookup_full_path: bool,
}

impl LanguageDetector for Language {
    fn current_language(&self) -> bool {
        std::path::Path::new(&self.detect_file).exists()
    }
}
