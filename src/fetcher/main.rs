// This is free and unencumbered software released into the public domain.

#[cfg(not(feature = "std"))]
compile_error!("asimov-signal-fetcher requires the 'std' feature");

use asimov_module::SysexitsError::{self, *};
use clap::Parser;
use clientele::StandardOptions;
use dogma::{Uri, UriScheme, UriValueParser};
use std::error::Error;

/// asimov-signal-fetcher
#[derive(Debug, Parser)]
#[command(arg_required_else_help = false)]
struct Options {
    #[clap(flatten)]
    flags: StandardOptions,

    /// Set the output format [default: jsonl] [possible values: jsonl]
    #[arg(value_name = "FORMAT", short = 'o', long)]
    output: Option<String>,

    /// The Signal URL to fetch (e.g., `sgnl://signal.me/#p/...`)
    #[arg(value_name = "URL", value_parser = UriValueParser::new(&[
        UriScheme::Other("sgnl".into()),
    ]))]
    url: Uri<'static>,
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

    // Locate the program path:
    let mut program_path = std::env::current_exe()?;
    program_path.set_file_name("asimov-signal-reader");
    if cfg!(windows) {
        program_path.set_extension("exe");
    }

    // Construct the program arguments:
    let mut program_args = vec![format!(
        "--output={}",
        options.output.as_deref().unwrap_or("jsonl")
    )];
    if options.flags.verbose > 0 {
        program_args.push(format!("-{}", "v".repeat(options.flags.verbose as usize)));
    }
    if options.flags.debug {
        program_args.push("-d".into());
    }

    // On Unix-like systems, use `execvp(3)` to replace the current process:
    #[cfg(unix)]
    {
        use std::{os::unix::process::CommandExt, process::Command};
        let error = Command::new(&program_path).args(program_args).exec();
        return Err(format!("failed to exec: {}", error).into());
    }

    // On Windows or when `execvp(3)` is not available, spawn a subprocess:
    #[cfg(not(unix))]
    {
        use std::process::{Command, exit};
        let status = Command::new(&program_path).args(program_args).status()?;
        exit(status.code().unwrap_or(SysexitsError::EX_UNAVAILABLE as _));
    }
}
