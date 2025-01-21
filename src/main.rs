use clap::Parser;

#[derive(Parser)]
#[clap(
    name = "dataengineering_deduplication",
    version = "0.1.0", 
    about = "Finds duplicate files",
    //after_help = "Example: dataengineering_deduplication search --path . --pattern .txt"
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    Search {
        #[clap(long, default_value = ".")]
        path: String,
        #[clap(long, default_value = "")]
        pattern: String,
    },
    Dedupe {
        #[clap(long, default_value = ".")]
        path: String,
        #[clap(long, default_value = "")]
        pattern: String,
    },
    Count {
        #[clap(long, default_value = ".")]
        path: String,
        #[clap(long, default_value = "")]
        pattern: String,
    },
}

fn main() {
    // let cli = Cli::parse();
    //let cli = Cli::parse_from(["dataengineering_deduplication", "dedupe", "--path", "tests/inputs", "--pattern", ".txt"]);
    let cli = Cli::parse_from(["dataengineering_deduplication", "count", "--path", "tests/inputs", "--pattern", ".txt"]);
    match cli.command {
        Some(Commands::Search { path, pattern }) => {
            println!("Searching for files in {} matching {}", path, pattern);
            let files = dataengineering_deduplication::walk(&path).unwrap();
            let files = dataengineering_deduplication::find(files, &pattern);
            println!("Found {} files matching {}", files.len(), pattern);
            for file in files {
                println!("{}", file);
            }
        }
        Some(Commands::Dedupe { path, pattern }) => {
            println!("Deduplicating files in {} with pattern {}", path, pattern);
            let result = dataengineering_deduplication::run(&path, &pattern);
            match result {
                Ok(_) => println!("Deduplication complete"),
                Err(e) => println!("Error: {}", e),
            }
        }
        Some(Commands::Count { path, pattern }) => {
            println!("Counting files in {} with pattern {}", path, pattern);
            let files = dataengineering_deduplication::walk(&path).unwrap();
            let files = dataengineering_deduplication::find(files, &pattern);
            println!("Found {} files matching {}", files.len(), pattern);
        }
        None => {
            println!("No command provided");
        }
    }
}
