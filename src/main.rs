use std::fs::File;
use std::io::prelude::*;
use std::io::Result;

mod cli;

fn main() {
    match cli::cli().get_matches().subcommand() {
        Some(("build", sub_matches)) => {
            println!("building");
        }
        Some(("run", sub_matches)) => {
            println!("running");
        }
        _ => {}
    }
}

fn build(input: &str, output: Option<&str>) -> Result<()> {
    let _ = transpile(File::open(input)?, create_file(output)?);

    Ok(())
}

fn create_file(output: Option<&str>) -> Result<File> {
    let mut out = File::create("out.c")?;

    // Write basic required C
    out.write(b"#include <stdio.h>\n")?;
    out.write(b"#include <stdlib.h>\n\n")?;
    out.write(b"int main() {\n")?;
    out.write(b"\tchar data[30000];\n")?;
    out.write(b"\tchar* pointer = buffer;\n")?;

    return Ok(out);
}

fn transpile(mut input: File, mut out: File) -> Result<()> {
    let mut input_str = String::new();
    input.read_to_string(&mut input_str)?;

    for c in input_str.chars() {
        let _ = match c {
            '>' => out.write(b"\tpointer++;"),
            '<' => out.write(b"\tpointer--;"),
            '+' => out.write(b"\t++*pointer;"),
            '-' => out.write(b"\t--*pointer;"),
            '[' => out.write(b"\twhile(*pointer) {"),
            ']' => out.write(b"\t}"),
            '.' => out.write(b"\tputchar(*pointer);"),
            ',' => out.write(b"\t*pointer = getchar();"),
            _ => out.write(b""),
        };
        out.write(b"\n")?;
    }

    Ok(())
}
