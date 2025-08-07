pub fn run(
  m: &clap::ArgMatches,
) -> anyhow::Result<()> {
  use std::path::PathBuf;

  let input = m.get_one::<PathBuf>("input").expect("[BUG] No input");
  let doc = lopdf::Document::load(input)?;
  println!("[[Doc Info]]");
  println!("  Version: {}", doc.version);
  println!("-- Objects (len={}) --", doc.objects.len());
  for (id, obj) in doc.objects {
    println!("[[Object: ({} {})]]", id.0, id.1);
    inspect(2, &obj)?;
  }
  println!("-- Trailer --");
  inspect(2, &lopdf::Object::Dictionary(doc.trailer.clone()))?;
  Ok(())
}

fn inspect(indent: usize, obj: &lopdf::Object) -> anyhow::Result<()> {
  use lopdf::content::Content;
  use lopdf::Object;

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
      println!("{:indent$}<<Name: \"{}\">>", "", String::from_utf8_lossy(v), indent = indent);
    },
    Object::String(str, fmt) => {
      println!("{:indent$}<<String: \"{}\", format={:?}>>", "", String::from_utf8_lossy(str), fmt, indent = indent);
    },
    Object::Array(array) => {
      println!("{:indent$}<<Array len={}>>", "", array.len(), indent = indent);
      for obj in array {
        inspect(indent + 2, obj)?;
      }
    },
    Object::Dictionary(dict) => {
      println!("{:indent$}<<Dictionary (len={})>>", "", dict.len(), indent = indent);
      for (key, obj) in dict {
        println!("{:indent$}[[Key: \"{}\"]]", "", String::from_utf8_lossy(key), indent = indent + 2);
        inspect(indent + 4, obj)?;
      }
    },
    Object::Stream(stream) => {
      println!("{:indent$}<<Stream>>", "", indent = indent);
      println!("{:indent$}[[Dictionary]]", "", indent = indent + 2);
      inspect(indent + 4, &Object::Dictionary(stream.dict.clone()))?;
      println!("{:indent$}[[Data (len={})]]", "", stream.content.len(), indent = indent + 2);
      if stream.dict.get(b"Subtype").and_then(Object::as_name).ok() == Some(b"Image") {
        println!("{:indent$}  [[ImageData Blob]]", "", indent = indent + 4);
      } else {
        let bytes = match stream.decompressed_content() {
          Ok(bytes) => bytes,
          Err(_) => stream.content.clone(),
        };
        match Content::decode(&bytes) {
          Ok(content) => {
            println!("{:indent$}<<Content (len={})>>", "", content.operations.len(), indent = indent + 4);
            for op in content.operations {
              println!("{:indent$}[[Op: \"{}\" (len={})]]", "", &op.operator, op.operands.len(), indent = indent + 6);
              let mut idx = 0;
              for operand in op.operands {
                println!("{:indent$}[[Operand: \"{}\"]]", "", idx, indent = indent + 8);
                inspect(indent + 8, &operand)?;
                idx += 1;
              }
            }
          },
          Err(err) => {
            println!("{:indent$}[err] Failed to decode: {:?}", "", err, indent = indent + 4);
          }
        }
      }
    },
    Object::Reference((id, ver)) => {
      println!("{:indent$}<<Reference: ({}, {})>>", "", id, ver, indent = indent);
    },
  }
  Ok(())
}
