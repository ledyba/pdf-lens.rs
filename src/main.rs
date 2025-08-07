mod app;

use std::path::PathBuf;
use anyhow::anyhow;
use clap::Command;

fn app() -> clap::Command {
  use clap::{Arg, ArgAction, value_parser};
  clap::Command::new("pdf-lens")
    .bin_name("pdf-lens")
    .author("Kaede Fujisaki")
    .about("PDF Inspector")
    .version("0.1.0")
    .arg(Arg::new("verbose")
      .long("verbose")
      .short('v')
      .required(false)
      .action(ArgAction::Count)
      .value_parser(value_parser!(u8))
      .help("Show verbose message"))
    .subcommand(Command::new("inspect")
      .arg(Arg::new("input")
      .help("Input file")
      .required(true)
      .value_parser(value_parser!(PathBuf))
      .index(1)))
}

fn main() -> anyhow::Result<()> {
  use tracing_subscriber::util::SubscriberInitExt;
  let app = app();
  let m = app.get_matches();
  let log_level = match m.get_one::<u8>("verbose") {
    None | Some(0) => tracing::Level::INFO,
    Some(1) => tracing::Level::DEBUG,
    _ => tracing::Level::TRACE,
  };
  tracing_subscriber::fmt()
    .with_timer(tracing_subscriber::fmt::time::ChronoLocal::new("%Y/%m/%d %H:%M:%S%.3f".to_string()))
    .with_max_level(log_level)
    .with_line_number(true)
    .with_file(true)
    .with_writer(std::io::stderr)
    .finish()
    .init();
  match m.subcommand() {
    Some(("inspect", m)) => app::inspect::run(m),
    Some((name, _)) => Err(anyhow!("unknown subcommand '{}'", name)),
    None => Err(anyhow!("No subcommand given")),
  }
}
