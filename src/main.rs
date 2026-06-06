use std::path::PathBuf;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "myapp", author, version, about, long_about = None)]
struct Args {
    // /// The file path
    // #[arg(short, long)]
    // file: String,
    //
    // /// Choose what process to use with file
    // #[arg(short, long, default_value = "read")]
    // process: String,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Web {
        #[arg(long)]
        outdir: PathBuf,
    },
    Email {
        #[arg(long)]
        to: String,
    },
    Grid {
        /// Image Filepath
        #[arg(long)]
        filepath: String,
        /// Number of rows to split image into
        #[arg(short, long, default_value_t = 0)]
        rows: u32,
        /// Number of columns to split image into
        #[arg(short, long, default_value_t = 0)]
        columns: u32
    },
}

fn main() {
    let args = Args::parse();
    //println!("File {}", args.file);

    // if args.process == "read" {
    //     read_file(args)
    // } else if args.process == "update" {
    //     update_file(args);
    // }

    match &args.command {
        Some(Commands::Web { outdir }) => {
            println!("outdir: {:?}", outdir);
        },
        Some(Commands::Email { to }) => {
            println!("to: {}", to);
        }
        Some(Commands::Grid { filepath, rows, columns }) => {
            println!("filepath: {}", filepath);
            println!("rows: {}", rows);
            println!("columns: {}", columns);
        }
        None => {
            println!("There was no subcommand given");
        }
    }
}

// fn read_file(args: Args) {
//     println!("Reading File {}", args.file);
// }
//
// fn update_file(args: Args) {
//     println!("Updating File {}", args.file);
// }