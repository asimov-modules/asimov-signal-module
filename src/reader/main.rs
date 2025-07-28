// This is free and unencumbered software released into the public domain.

#[cfg(not(feature = "std"))]
compile_error!("asimov-signal-reader requires the 'std' feature");

use asimov_module::{
    SysexitsError::{self, *},
    json::SkipNulls,
    secrecy::ExposeSecret,
};
use clap::Parser;
use clientele::StandardOptions;
use rusqlite::{Connection, Result};
use serde_json::{Value, json};
use std::{error::Error, io::Write, path::PathBuf};

/// asimov-signal-reader
#[derive(Debug, Parser)]
#[command(arg_required_else_help = true)]
struct Options {
    #[clap(flatten)]
    flags: StandardOptions,

    /// Set the output format [default: jsonl] [possible values: jsonl]
    #[arg(value_name = "FORMAT", short = 'o', long)]
    output: Option<String>,

    /// Path to the (unencrypted!) Signal database file
    #[clap(value_name = "SIGNAL-DB-FILE")]
    path: PathBuf,
}

pub fn main() -> Result<SysexitsError, Box<dyn Error>> {
    // Load environment variables from `.env`:
    asimov_module::dotenv().ok();

    // Expand wildcards and @argfiles:
    let args = asimov_module::args_os()?;

    // Parse command-line options:
    let options = Options::parse_from(args);

    // Handle the `--version` flag:
    if options.flags.version {
        println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        return Ok(EX_OK);
    }

    // Handle the `--license` flag:
    if options.flags.license {
        print!("{}", include_str!("../../UNLICENSE"));
        return Ok(EX_OK);
    }

    // Configure logging & tracing:
    #[cfg(feature = "tracing")]
    asimov_module::init_tracing_subscriber(&options.flags).expect("failed to initialize logging");

    let key = asimov_module::getenv::var_secret("ASIMOV_SIGNAL_KEY");

    let conn = Connection::open(&options.path)?;

    if let Some(key) = key {
        conn.pragma_update(None, "key", format!("x'{}'", key.expose_secret()))?;
    }

    let mut stmt = conn.prepare("SELECT id, type, json FROM conversations")?;
    let mut rows = stmt.query([])?;

    let stdout = std::io::stdout();
    while let Some(row) = rows.next()? {
        let id = row.get::<usize, String>(0)?;
        let uri = format!("urn:uuid:{}", id);
        let r#type = row.get::<usize, String>(1)?;
        let json: Value = serde_json::from_str(&row.get::<usize, String>(2)?)?;

        let output = match r#type.as_ref() {
            "private" => json!({
                "@type": "SignalChat",
                "@id": uri,
                "label": json["profileName"],
                "peer": {
                    "@type": "SignalAccount",
                    "@id": format!("urn:uuid:{}", json["serviceId"].as_str().unwrap_or("").replace("PNI:", "")),
                    "username": json["username"],
                    "phone": json["e164"].as_str().map(|f| format!("tel:{}", f)),
                    "name": json["systemGivenName"],
                },
            }),
            "group" => {
                let members = json["membersV2"].as_array().unwrap();
                json!({
                    "@type": "SignalGroupChat",
                    "@id": uri,
                    "label": json["name"],
                    "members": {
                        "count": members.len(),
                        "items": members.iter().map(|member| {
                            let member = member.as_object().unwrap();
                            format!("urn:uuid:{}", member["aci"].as_str().unwrap_or(""))
                        }).collect::<Vec<_>>()
                    },
                })
            },
            _ => unreachable!(),
        };

        serde_json::to_writer(&stdout, &SkipNulls(output))?;
        writeln!(&stdout)?;
    }

    Ok(EX_OK)
}
