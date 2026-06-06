use clap:: Parser;

#[derive(Parser)]
#[command(name = "myapp", author, version, about, long_about = None)]
struct Args {
    /// The file path
    #[arg(short, long)]
    file: String,

    /// Choose what process to use with file
    #[arg(short, long, default_value = "read")]
    process: String
}

fn main() {
    let args = Args::parse();
    println!("File {}", args.file);

    if args.process == "read" {
        read_file(args)
    } else if args.process == "update" {
        update_file(args);
    }
}

fn read_file(args: Args) {
    println!("Reading File {}", args.file);
}

fn update_file(args: Args) {
    println!("Updating File {}", args.file);
}