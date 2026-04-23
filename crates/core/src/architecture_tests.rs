#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::{Path, PathBuf};

    fn collect_rust_sources(dir: &Path) -> Vec<PathBuf> {
        let mut files = Vec::new();
        collect_rust_sources_into(dir, &mut files);
        files
    }

    fn collect_rust_sources_into(dir: &Path, files: &mut Vec<PathBuf>) {
        for entry in fs::read_dir(dir).expect("read core source directory") {
            let entry = entry.expect("directory entry");
            let path = entry.path();
            if path.is_dir() {
                collect_rust_sources_into(&path, files);
            } else if path.extension().and_then(|ext| ext.to_str()) == Some("rs") {
                files.push(path);
            }
        }
    }

    fn source_path(relative: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join(relative)
    }

    fn read_source(path: &Path) -> String {
        fs::read_to_string(path).expect("read source file")
    }

    #[test]
    fn core_source_stays_framework_agnostic() {
        let src_root = source_path("");
        let files = collect_rust_sources(&src_root);

        let forbidden_terms = [
            "angular", "vue", "jsx", "tailwind", "react", "svelte", "ngif", "ngclass", "ngmodel",
        ];

        for file in files {
            if file.file_name().and_then(|name| name.to_str()) == Some("architecture_tests.rs") {
                continue;
            }
            let contents = read_source(&file);
            let lower = contents.to_lowercase();
            for term in forbidden_terms {
                assert!(
                    !lower.contains(term),
                    "found forbidden framework term '{term}' in {}",
                    file.display()
                );
            }
        }
    }

    #[test]
    fn engine_stays_free_of_cli_and_io_concerns() {
        let engine_source = read_source(&source_path("engine.rs")).to_lowercase();
        let forbidden_terms = [
            "std::fs",
            "stdin",
            "stdout",
            "println!",
            "eprintln!",
            "clap::",
        ];

        for term in forbidden_terms {
            assert!(
                !engine_source.contains(term),
                "engine.rs should remain an orchestration layer without '{term}'"
            );
        }
    }

    #[test]
    fn formatter_modules_do_not_depend_on_the_parser_layer() {
        let formatter_root = source_path("formatter");

        for file in collect_rust_sources(&formatter_root) {
            let contents = read_source(&file);
            assert!(
                !contents.contains("crate::parser"),
                "formatter module should not import the parser directly: {}",
                file.display()
            );
            assert!(
                !contents.contains("Parser::new"),
                "formatter module should not construct parsers directly: {}",
                file.display()
            );
        }
    }
}
