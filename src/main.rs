use std::io::{self, Write, Result};
use std::env;

fn main() -> Result<()> {
    let mut args = env::args_os().skip(1);
    let mut out = io::stdout().lock();

    let mut newline = true;
    let mut first = true;

    while let Some(arg) = args.next() {
        if first && arg == "-n" {
            newline = false;
            continue;
        }

        if !first {
            out.write_all(b" ")?;
        }

        out.write_all(arg.as_encoded_bytes())?;
        first = false;
    }

    if newline {
        out.write_all(b"\n")?;
    }

    Ok(())
}
