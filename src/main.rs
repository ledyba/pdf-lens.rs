use std::path::PathBuf;
use lopdf::content::Content;
use lopdf::Object;

fn app() -> clap::Command {
  use clap::{Arg, ArgAction, value_parser};
  clap::Command::new("pittari")
    .bin_name("pittari")
    .author("Kaede Fujisaki")
    .about("ぴったり印刷くん")
    .version("0.1.0")
    .arg(Arg::new("verbose")
      .long("verbose")
      .short('v')
      .required(false)
      .action(ArgAction::Count)
      .value_parser(value_parser!(u8))
      .help("Show verbose message"))
    .arg(Arg::new("input")
      .help("Input file")
      .required(true)
      .value_parser(value_parser!(PathBuf))
      .index(1))
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
  let input = m.get_one::<PathBuf>("input").expect("[BUG] No input");
  let doc = lopdf::Document::load(input)?;
  for (id, obj) in doc.objects {
    println!("[Object: ({} {})]", id.0, id.1);
    inspect(2, &obj);
  }
  Ok(())
}

fn inspect(indent: usize, obj: &Object) {
  match obj {
    Object::Null => {
      println!("{:indent$}<<null>>", "", indent = indent);
    },
    Object::Boolean(v) => {
      println!("{:indent$}<<Boolean: {}>>", "", v, indent = indent)
    },
    Object::Integer(v) => {
      println!("{:indent$}<<Integer: {}>>", "", v, indent = indent);
    },
    Object::Real(v) => {
      println!("{:indent$}<<Real: {}>>", "", v, indent = indent);
    },
    Object::Name(v) => {
      println!("{:indent$}<<Name: {}>>", "", String::from_utf8_lossy(v), indent = indent);
    },
    Object::String(str, fmt) => {
      println!("{:indent$}<<String: {}, fmt={:?}>>", "", String::from_utf8_lossy(str), fmt, indent = indent);
    },
    Object::Array(array) => {
      println!("{:indent$}<<Array, len={}>>", "", array.len(), indent = indent);
      for obj in array {
        inspect(indent + 2, obj);
      }
    },
    Object::Dictionary(dict) => {
      println!("{:indent$}<<Dictionary, len={}>>", "", dict.len(), indent = indent);
      for (key, obj) in dict {
        println!("{:indent$}<<Dictionary>>", "", indent = indent + 2);
        println!("{:indent$}Key: {}", "", String::from_utf8_lossy(key), indent = indent + 4);
        inspect(indent + 4, obj);
      }
    },
    Object::Stream(stream) => {
      println!("{:indent$}<<Stream>>", "", indent = indent);
      println!("{:indent$}<<Dictionary>>", "", indent = indent + 2);
      inspect(indent + 4, &Object::Dictionary(stream.dict.clone()));
      println!("{:indent$}<<Data, len={}>>", "", stream.content.len(), indent = indent + 2);
      if stream.dict.get(b"Subtype").and_then(Object::as_name_str).ok() == Some("Image") {
        println!("{:indent$}  [[ImageData]]", "", indent = indent + 4);
      } else {
        match stream.decompressed_content() {
          Ok(bytes) => {
            match Content::decode(&bytes) {
              Ok(content) => {
                println!("{:indent$}<<Content, len={}>>", "", content.operations.len(), indent = indent + 4);
                for op in content.operations {
                  println!("{:indent$}<<Op: {}, len={}>>", "", &op.operator, op.operands.len(), indent = indent + 6);
                  let mut idx = 0;
                  for operand in op.operands {
                    println!("{:indent$}<<Operand: {}>>", "", idx, indent = indent + 8);
                    inspect(indent + 10, &operand);
                    idx += 1;
                  }
                }
              }
              Err(err) => {
                println!("{:indent$}[err] Failed to decode: {:?}", "", err, indent = indent + 4);
              }
            }
          },
          Err(err) => {
            println!("{:indent$}[err] Failed to decompress: {:?}", "", err, indent = indent + 4);
          },
        }
      }
    },
    Object::Reference((id, ver)) => {
      println!("{:indent$}<<Reference: ({}, {})>>", "", id, ver, indent = indent);
    },
  }
}
