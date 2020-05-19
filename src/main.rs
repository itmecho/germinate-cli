#[macro_use]
extern crate clap;

use anyhow::Result;
use clap::{App, Arg};
use germinate::Seed;

#[tokio::main]
async fn main() -> Result<()> {
    let matches = App::new("germinate")
        .about("Simple variable injection templating tool with the ability to load variables from multiple sources")
        .author(crate_authors!())
        .version(crate_version!())
        .arg(
            Arg::with_name("INPUT_FILE")
            .required(true)
            )
        .arg(
            Arg::with_name("OUTPUT_FILE")
            .required(true)
            )
        .get_matches();

    let template = std::fs::read_to_string(matches.value_of("INPUT_FILE").unwrap())?;

    let mut seed = Seed::new(template);

    Ok(std::fs::write(
        matches.value_of("OUTPUT_FILE").unwrap(),
        seed.germinate().await?,
    )?)
}
