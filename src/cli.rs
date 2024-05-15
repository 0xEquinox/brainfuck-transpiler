use clap::{arg, Command};

pub fn cli() -> Command {
    Command::new("bf")
        .about("Brainfuck to C transpiler")
        .arg(arg!(-c --clang "sets the compiler to use clang instead of gcc"))
        .arg(arg!(-g --gcc "sets the compile to use gcc explictly (default)"))
        .subcommand_required(true)
        .subcommand(
            Command::new("build")
                .about("builds files")
                .arg(arg!(<INPUT> "Input file to build"))
                .arg(arg!(-o <OUTPUT> "Output file"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("run")
                .about("run files")
                .arg(arg!(<INPUT> "Input file to run"))
                .arg(arg!(-o [OUTPUT] "Output file"))
                .arg_required_else_help(false),
        )
}
