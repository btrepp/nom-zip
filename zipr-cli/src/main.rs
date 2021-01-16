mod args;
mod display;
mod error;
mod procedural;
mod sequence;
use anyhow::Result;
use args::zipr::Opt;
use std::env;

const UNZIP: &str = "unzip";

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let head = args.first().map(String::as_str);

    // if arg0 matches a different name, parse those arguments
    // occurs when symlinked as a different name
    let opt = match head {
        Some(UNZIP) => args::unzip::parse_args(),
        _ => args::zipr::parse_args(),
    };

    // Run logic;
    match opt {
        Opt::List { file } => procedural::list_files(file),
        Opt::ShowComment { file } => procedural::show_comment(file),
        Opt::Extract {
            file,
            files,
            output,
        } => procedural::extract_files(file, files, output),
        Opt::Add {
            file,
            files,
            compress,
        } => procedural::add_files(file, files, compress),
        Opt::Inspect {
            file,
            offset,
            kind,
            take,
        } => procedural::inspect(file, kind, offset.0, take.map(|x| x.into())),
    }
}
