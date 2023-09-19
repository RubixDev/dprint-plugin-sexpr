use std::path::Path;

use anyhow::bail;
use dprint_core::{
    configuration::{
        self, ConfigKeyMap, GlobalConfiguration, ResolveConfigurationResult,
        RECOMMENDED_GLOBAL_CONFIGURATION,
    },
    plugins::{FileMatchingInfo, FormatResult, PluginInfo, SyncPluginHandler, SyncPluginInfo},
};

use crate::config::Configuration;

pub struct SexprPluginHandler;

impl SyncPluginHandler<Configuration> for SexprPluginHandler {
    fn resolve_config(
        &mut self,
        config: ConfigKeyMap,
        global_config: &GlobalConfiguration,
    ) -> ResolveConfigurationResult<Configuration> {
        let mut config = config;
        let mut diagnostics = vec![];

        let resolved_config = Configuration {
            indent_width: configuration::get_value(
                &mut config,
                "indentWidth",
                global_config
                    .indent_width
                    .unwrap_or(RECOMMENDED_GLOBAL_CONFIGURATION.indent_width),
                &mut diagnostics,
            ),
        };

        diagnostics.extend(configuration::get_unknown_property_diagnostics(config));

        ResolveConfigurationResult {
            diagnostics,
            config: resolved_config,
        }
    }

    fn plugin_info(&mut self) -> SyncPluginInfo {
        SyncPluginInfo {
            info: PluginInfo {
                name: env!("CARGO_PKG_NAME").to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
                config_key: "sexpr".to_string(),
                help_url: concat!(env!("CARGO_PKG_REPOSITORY"), "#readme").to_string(),
                config_schema_url: "".to_string(),
                update_url: Some(
                    "https://plugins.dprint.dev/RubixDev/sexpr/latest.json".to_string(),
                ),
            },
            file_matching: FileMatchingInfo {
                file_extensions: vec!["scm".to_string()],
                file_names: vec![],
            },
        }
    }

    fn license_text(&mut self) -> String {
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/LICENSE")).into()
    }

    fn format(
        &mut self,
        _file_path: &Path,
        file_text: &str,
        config: &Configuration,
        _format_with_host: impl FnMut(&Path, String, &ConfigKeyMap) -> FormatResult,
    ) -> FormatResult {
        let result = format!(
            "{:#.indent$}",
            match rsexpr::from_slice_multi(file_text) {
                Ok(sexprs) => sexprs,
                Err(errs) => bail!(
                    "parsing of S-expressions failed: {}",
                    errs.iter()
                        .map(ToString::to_string)
                        .collect::<Vec<_>>()
                        .join(", ")
                ),
            },
            indent = config.indent_width as usize
        );
        if result == file_text {
            Ok(None)
        } else {
            Ok(Some(result))
        }
    }
}

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
dprint_core::generate_plugin_code!(SexprPluginHandler, SexprPluginHandler);
