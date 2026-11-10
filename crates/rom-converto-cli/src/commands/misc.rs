use clap::Parser;

use anyhow::Result;

/// Print the installed binary's supported operations and formats as JSON
#[derive(Parser, Debug, Clone, Eq, PartialEq)]
pub struct CapabilitiesCommand {}

/// Runs one `capabilities` subcommand.
pub async fn run(_cmd: CapabilitiesCommand) -> Result<()> {
    let manifest = serde_json::json!({
        "schema": "rom-converto.capabilities.v1",
        "name": "rom-converto",
        "version": env!("ROM_CONVERTO_DISPLAY_VERSION"),
        "info_extensions": rom_converto_lib::info::SUPPORTED_INFO_EXTENSIONS,
        "runner": rom_converto_lib::runner::schema_json(),
    });
    println!("{}", serde_json::to_string_pretty(&manifest)?);
    Ok(())
}
