use crate::args::Args;
use fua_core::config::{FormatterConfig, PluginConfig};
use fua_core::engine::FormatEngine;
use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

type CliResult<T> = Result<T, String>;

pub(crate) fn run(args: Args) -> CliResult<()> {
    let input = read_input(args.input.as_deref())?;
    ensure_input_not_empty(&input)?;

    let mut config = load_formatter_config(args.config.as_deref())?;
    apply_cli_overrides(&mut config, &args);

    let plugin_configs = resolve_plugin_configs(&config, args.config.as_deref(), &args.plugin);
    let output = run_engine(&input, config, &plugin_configs)?;
    write_output(args.output.as_deref(), &output)?;

    Ok(())
}

fn read_input(path: Option<&Path>) -> CliResult<String> {
    if let Some(path) = path {
        fs::read_to_string(path)
            .map_err(|error| format!("failed to read input file '{}': {error}", path.display()))
    } else {
        let mut input = String::new();
        io::stdin()
            .read_to_string(&mut input)
            .map_err(|error| format!("failed to read stdin: {error}"))?;
        Ok(input)
    }
}

fn ensure_input_not_empty(input: &str) -> CliResult<()> {
    if input.trim().is_empty() {
        Err("No HTML provided.".to_string())
    } else {
        Ok(())
    }
}

fn load_formatter_config(path: Option<&Path>) -> CliResult<FormatterConfig> {
    let Some(path) = path else {
        return Ok(FormatterConfig::default());
    };

    let config_str = fs::read_to_string(path)
        .map_err(|error| format!("failed to read config file '{}': {error}", path.display()))?;
    serde_json::from_str(&config_str)
        .map_err(|error| format!("failed to parse config file '{}': {error}", path.display()))
}

fn apply_cli_overrides(config: &mut FormatterConfig, args: &Args) {
    if let Some(indent_size) = args.indent_size {
        config.indent_size = indent_size;
    }

    if let Some(use_tabs) = args.use_tabs {
        config.use_tabs = use_tabs;
    }
}

fn resolve_plugin_configs(
    config: &FormatterConfig,
    config_path: Option<&Path>,
    cli_plugins: &[PathBuf],
) -> Vec<PluginConfig> {
    let mut plugins = resolve_configured_plugins(config, config_path);
    plugins.extend(cli_plugin_paths_to_configs(cli_plugins));
    plugins
}

fn resolve_configured_plugins(
    config: &FormatterConfig,
    config_path: Option<&Path>,
) -> Vec<PluginConfig> {
    config
        .plugin_configs()
        .into_iter()
        .map(|mut plugin| {
            if let Some(path) = plugin.path.as_deref() {
                plugin.path = Some(
                    resolve_plugin_path(Path::new(path), config_path)
                        .to_string_lossy()
                        .into_owned(),
                );
            }
            plugin
        })
        .collect()
}

fn cli_plugin_paths_to_configs(plugin_paths: &[PathBuf]) -> Vec<PluginConfig> {
    plugin_paths
        .iter()
        .map(|path| PluginConfig {
            path: Some(path.to_string_lossy().into_owned()),
            options: None,
        })
        .collect()
}

fn resolve_plugin_path(plugin_path: &Path, config_path: Option<&Path>) -> PathBuf {
    let path = plugin_path;
    if path.is_absolute() {
        return path.to_path_buf();
    }

    let Some(config_path) = config_path else {
        return path.to_path_buf();
    };

    let config_dir = config_path.parent().unwrap_or(Path::new("."));
    config_dir.join(path)
}

fn run_engine(input: &str, config: FormatterConfig, plugins: &[PluginConfig]) -> CliResult<String> {
    let mut engine = FormatEngine::new(config);
    load_plugin_configs(&mut engine, plugins)?;
    Ok(engine.format(input))
}

fn load_plugin_configs(engine: &mut FormatEngine, plugins: &[PluginConfig]) -> CliResult<()> {
    for plugin in plugins {
        if let Err(error) = engine.load_plugin_config(plugin) {
            let path = plugin.path.as_deref().unwrap_or("<missing>");
            return Err(format!("failed to load plugin '{path}': {error}"));
        }
    }

    Ok(())
}

fn write_output(path: Option<&Path>, output: &str) -> CliResult<()> {
    if let Some(path) = path {
        fs::write(path, output).map_err(|error| {
            format!("failed to write output file '{}': {error}", path.display())
        })?;
        println!("Successfully formatted into: {}", path.display());
    } else {
        print!("{output}");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_args() -> Args {
        Args {
            input: None,
            output: None,
            config: None,
            indent_size: None,
            use_tabs: None,
            plugin: Vec::new(),
        }
    }

    #[test]
    fn cli_overrides_replace_config_values() {
        let mut config = FormatterConfig::default();
        let mut args = sample_args();
        args.indent_size = Some(4);
        args.use_tabs = Some(true);

        apply_cli_overrides(&mut config, &args);

        assert_eq!(config.indent_size, 4);
        assert!(config.use_tabs);
    }

    #[test]
    fn relative_plugin_paths_are_resolved_from_config_directory() {
        let resolved = resolve_plugin_path(
            Path::new("plugins/angular.wasm"),
            Some(Path::new(r"C:\project\config.json")),
        );

        assert_eq!(resolved, PathBuf::from(r"C:\project\plugins\angular.wasm"));
    }

    #[test]
    fn requested_plugins_merge_configured_and_cli_sources() {
        let config = FormatterConfig {
            plugins: vec![PluginConfig {
                path: Some("plugins/configured.wasm".to_string()),
                options: None,
            }],
            plugin: PluginConfig {
                path: Some("plugins/legacy.wasm".to_string()),
                options: None,
            },
            ..FormatterConfig::default()
        };

        let plugins = resolve_plugin_configs(
            &config,
            Some(Path::new(r"C:\project\config.json")),
            &[PathBuf::from(r"C:\plugins\cli.wasm")],
        );

        assert_eq!(plugins.len(), 3);
        assert_eq!(
            plugins[0].path.as_deref().map(PathBuf::from),
            Some(PathBuf::from(r"C:\project\plugins\configured.wasm"))
        );
        assert_eq!(
            plugins[1].path.as_deref().map(PathBuf::from),
            Some(PathBuf::from(r"C:\project\plugins\legacy.wasm"))
        );
        assert_eq!(
            plugins[2].path.as_deref().map(PathBuf::from),
            Some(PathBuf::from(r"C:\plugins\cli.wasm"))
        );
    }
}
