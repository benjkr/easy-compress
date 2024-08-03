mod compress;
mod easy;

use std::{
    fs::File,
    path::{Path, PathBuf},
    process::ExitCode,
    time::Instant,
};

use clap::Parser;
use compress::Compressor;
use easy::Easy;

const FILE_EXTENSION: &str = ".ez";
const DEFAULT_OUTPUT: &str = "output";

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Decompress a file
    #[arg(short, long)]
    decompress: bool,

    /// Optional output file. Defaults to 'output'
    #[arg(short, long, default_value = DEFAULT_OUTPUT)]
    output: Option<PathBuf>,

    /// Overwrite existing output file
    #[arg(short, long)]
    force: bool,

    input: PathBuf,
}

fn get_compressed_file_name(path: &Path) -> PathBuf {
    let mut new_file_name = path.file_stem().unwrap().to_os_string();
    new_file_name.push(FILE_EXTENSION);
    PathBuf::from(new_file_name)
}

fn main() -> ExitCode {
    let start = Instant::now();
    let args = Args::parse();
    if !args.input.is_file() {
        eprintln!(
            "Input file '{:?}' does not exist. Check the path and try again.",
            args.input
        );
        return ExitCode::FAILURE;
    }
    let file = File::open(&args.input).unwrap();

    let mut easy = Easy::new(Compressor {});

    let new_file_name: PathBuf = if args.decompress {
        args.output.unwrap()
    } else {
        get_compressed_file_name(&args.input)
    };

    println!("Writing to file {}", new_file_name.display());

    if new_file_name.exists() && !args.force {
        eprintln!(
            "File '{}' already exists. Use (-f | --force) to overwrite.",
            new_file_name.display()
        );
        return ExitCode::FAILURE;
    }

    let mut new_file = File::create(new_file_name).unwrap();
    if args.decompress {
        easy.decompress(&file, &mut new_file)
    } else {
        easy.compress(&file, &mut new_file)
    };

    println!("Finished writing file");
    println!("Time taken: {:?}", start.elapsed());

    ExitCode::SUCCESS
}
