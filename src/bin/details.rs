use std::{io::BufRead, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = std::io::stdin().lock();
    let mut lines = stdin.lines();

    let first = lines.next().ok_or("no input")??;
    if first.starts_with("<details>") {
        return Err("undo details not supported yet".into());
    }

    // make the first line the summary text
    println!("<details>");
    println!("  <summary>{first}</summary>");
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
