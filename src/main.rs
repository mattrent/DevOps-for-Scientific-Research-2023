use std::{
    fs,
    io::{self, BufRead, BufReader},
};

use clap::{ArgAction, Parser};

// https://en.wikipedia.org/wiki/Wc_(Unix)

#[derive(Parser, Debug)]
#[command(author, version, about = "Very simple (and incomplete) clone of the wc command.", long_about = None)]
struct Args {
    /// target file
    #[arg(help = "Target file. Reads from stdin if None.")]
    file_name: Option<String>,

    /// count lines
    #[arg(short, long, action, help = "Print the newline counts.")]
    lines: bool,

    /// count bytes
    #[arg(short = 'c', long = "bytes", action, help = "Print the byte counts.")]
    bytes: bool,

    /// count bytes
    #[arg(short = 'w', long = "words", action, help = "Print the word counts.")]
    words: bool,

    /// be verbose
    #[arg(
        short = 'v',
        long = "verbose",
        action = ArgAction::Count,
        help = "Level of verbosity. Maximum value of 3.",
        long_help = "
        Level of verbosity. Maximum value of 3.
        Level 0 only prints the required amounts. This is the default.
        Level 1 prints the identifiers for the amounts.
        Level 2 prints the file name and the value for all flags.
        Level 3 prints the file contents."
    )]
    verbose: u8,
}

fn main() {
    let args = Args::parse();
    if let Err(e) = run(args) {
        eprintln!("{}", e);
        std::process::exit(1)
    }
}

fn run(args: Args) -> Result<(), io::Error> {
    let is_stdin = args.file_name.is_none();
    let all_false = !args.bytes && !args.lines && !args.words;

    let mut reader = open(args.file_name.to_owned())?;

    let mut content: String = String::new();
    let n_bytes = reader.read_to_string(&mut content)?;
    let lines = content.split('\n');
    let n_words = lines
        .to_owned()
        .map(|l| l.split(' ').filter(|&w| !w.is_empty()).count())
        .reduce(|acc, e| acc + e)
        .unwrap();

    let n_lines = lines.count() - 1;

    let mut out_string = String::new();

    if args.verbose == 0 {
        if args.lines || all_false {
            out_string = format!("{}\t", n_lines);
        }
        if args.words || all_false {
            out_string = format!("{}{}\t", out_string, n_words);
        }
        if args.bytes || all_false {
            out_string = format!("{}{}\t", out_string, n_bytes)
        }
        out_string = out_string.trim().to_owned();
    } else {
        if args.lines || all_false {
            out_string = format!("Lines: {}\n", n_lines);
        }
        if args.words || all_false {
            out_string = format!("{}Words: {}\n", out_string, n_words);
        }
        if args.bytes || all_false {
            out_string = format!("{}Bytes: {}\n", out_string, n_bytes)
        }
        out_string = out_string.trim().to_owned();
    }

    println!("{}", out_string);

    if args.verbose >= 2 {
        println!("############");
        if is_stdin {
            println!("Reading from stdin.")
        } else {
            println!("Reading from {}.", args.file_name.unwrap())
        }
        println!(
            "Words flag is set to {:?}.\nBytes flag is set to {:?}.\nLines flag is set to {:?}.\nVerbose flag is set to {:?} (if >3, counts as 3).",
            args.words, args.bytes, args.lines, args.verbose
        );
        println!("############");
    }

    if args.verbose >= 3 {
        println!("Content starts at next line:");
        println!("{:?}", content);
    }

    Ok(())
}

fn open(file_arg: Option<String>) -> Result<Box<dyn BufRead>, io::Error> {
    let reader: Box<dyn BufRead> = match file_arg {
        None => {
            let reader = BufReader::new(io::stdin());
            Box::new(reader)
        }
        Some(path) => {
            let file = fs::File::open(path)?;
            let reader = BufReader::new(file);
            Box::new(reader)
        }
    };

    Ok(reader)
}
