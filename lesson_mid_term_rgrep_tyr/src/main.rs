use anyhow::Result;
use clap::Parser;

use lesson_mid_term_rgrep_tyr::GrepConfig;

fn main() -> Result<()> {
    let config: GrepConfig = GrepConfig::parse();
    config.match_with_default_strategy()?;

    Ok(())
}
