use clap::App;
use clap::Arg;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(file) => {
                let mut last_num = 0;
                for (line_num, line) in file.lines().enumerate() {
                    let line = line?;
                    if config.number_lines {
                        println!("{:>6}\t{}", line_num + 1, line);
                    } else if config.number_nonblank_lines {
                        if !line.is_empty() {
                            last_num += 1;
                            println!("{:>6}\t{}", last_num, line)
                        } else {
                            println!();
                        }
                    } else {
                        println!("{}", line);
                    }
                }
            }
        }
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
pub fn get_args() -> MyResult<Config> {
    let matches = App::new("rcat")
        .version("0.1.0")
        .about("Rust cat")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .default_value("-")
                .multiple(true)
                .help("Input files"),
        )
        .arg(
            Arg::with_name("line_numbers")
                .short("n")
                .long("number")
                .help("Show line numbers")
                .conflicts_with("nonblank_lines")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("nonblank_lines")
                .short("-b")
                .long("number-nonblank")
                .help("Number only non-blank lines")
                .takes_value(false),
        )
        .get_matches();

    let files = matches.values_of_lossy("files").unwrap();
    let line_numbers = matches.is_present("line_numbers");
    let nonblank_lines: bool = matches.is_present("nonblank_lines");
    Ok(Config {
        files: files,
        number_lines: line_numbers,
        number_nonblank_lines: nonblank_lines,
    })
}
