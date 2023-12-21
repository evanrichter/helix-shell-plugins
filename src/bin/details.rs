/// Creates markdown "details" tag (really html I guess).
///
/// Input:
///     summary line
///     body1
///     body2
/// Output:
///     <details>
///       <summary>summary line</summary>
///     
///       body1
///       body2
///     
///     </details>
///
/// This command is reversible, and will turn a details tag back into plaintext lines

use std::{io::BufRead, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = std::io::stdin().lock();
    let mut lines = stdin.lines();

    let first = lines.next().ok_or("no input")??;
    let first = first.trim();
    if first.starts_with("<details>") {
        undo_details(lines).map_err(Into::into)
    } else {
        make_details(lines, first).map_err(Into::into)
    }
}

fn make_details<E: Error + 'static>(lines: impl Iterator<Item = Result<String, E>>, summary: &str) -> Result<(), Box<dyn Error>> {
    eprintln!("Creating details tag. ");

    println!("<details>");
    println!("  <summary>{summary}</summary>");
    println!();

    // the rest of the input is body
    let mut body = false;
    for line in lines {
        let line = line?;
        println!("  {line}");
        body = true;
    }

    if !body {
        return Err("no body contents".into());
    }

    // footer
    println!();
    println!("</details>");

    Ok(())
}

fn undo_details<E: Error + 'static>(mut lines: impl Iterator<Item = Result<String, E>>) -> Result<(), Box<dyn Error>> {
    eprintln!("Undoing details tag. ");

    let summary = lines.next().ok_or("no summary line")??;
    let summary = summary.trim();
    let summary = summary.strip_prefix("<summary>").ok_or("malformed summary start tag")?;
    let summary = summary.strip_suffix("</summary>").ok_or("malformed summary end tag")?;
    let summary = summary.trim();

    println!("{summary}");

    let _ = lines.next().ok_or("no empty line before body")??;

    for line in &mut lines {
        let line = line?;
        if line.is_empty() {
            break;
        }

        let line = line.strip_prefix("  ").ok_or("malformed body leading whitespace")?;
        println!("{line}")
    }

    // footer
    let footer = lines.next().ok_or("no footer")??;
    if footer != "</details>" {
        return Err("malformed details end tag".into());
    }

    Ok(())
}
