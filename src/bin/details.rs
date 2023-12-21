use std::{io::BufRead, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = std::io::stdin().lock();
    let mut lines = stdin.lines();

    let first = lines.next().ok_or("no input")??;
    let first = first.trim();
    if first.starts_with("<details>") {
        Err("undo details not supported yet".into())
    } else {
        make_details(lines, first).map_err(Into::into)
    }
}

fn make_details<E>(lines: impl Iterator<Item = Result<String, E>>, summary: &str) -> Result<(), E> {
    println!("<details>");
    println!("  <summary>{summary}</summary>");
    println!();

    // the rest of the input is body
    for line in lines {
        let line = line?;
        println!("  {line}")
    }

    // footer
    println!();
    println!("</details>");

    Ok(())
}
