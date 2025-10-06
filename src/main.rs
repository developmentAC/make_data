use clap::Parser;
use colored::Colorize;
use csv::Writer;
use fake::{
    faker::{lorem::en::Word, name::en::Name, phone_number::en::PhoneNumber},
    Fake,
};
use rand::Rng;
use std::{
    fs,
//    io::{self, Write}, //not necessary anymore ...
    path::{Path, PathBuf},
};

mod toml_extract;

/// Command-line argument structure using clap
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of rows to generate
    #[arg(short, long, default_value = "10")]
    rows: usize,

    /// Default column types, comma-separated (e.g. int,float,word,name,phone)
    #[arg(short, long, default_value = "int,float,word,name,phone")]
    columns: String,

    /// Output CSV file name
    #[arg(short, long, default_value = "output.csv")]
    output: String,

    /// Range for random number generation
    #[arg(short, long, default_value = "100")]
    myrange: usize,

    /// Show extended help
    #[arg(short, long, default_value = "false")]
    bighelp: bool,
}

/// Enum for supported column types
#[derive(Debug, Clone)]
enum ColType {
    Int,
    Float,
    Word,
    Name,
    Phone,
}

/// Main entry point for the program.
/// Handles argument parsing, data generation, and writing to CSV.
fn main() {
    show_banner();
    toml_extract::main();

    let args = Args::parse();

    if args.bighelp {
        print_big_help();
        return;
    }

    let out_dir = "0_out";
    ensure_output_directory_exists(out_dir);

    let output_path = get_unique_filename(out_dir, &args.output);
    println!("Output will be written to: {}", output_path.display());

    let col_types = parse_col_types(&args.columns);
    let mut wtr = Writer::from_path(&output_path).expect("Cannot create file");

    let myrange = args.myrange;
    println!("Range for random numbers: {}", myrange);

    // Write CSV header row
    let header: Vec<String> = col_types
        .iter()
        .enumerate()
        .map(|(i, t)| format!("col{}_{}", i + 1, t.to_string()))
        .collect();
    wtr.write_record(&header).unwrap();

    let mut rng = rand::rng();

    // Generate and write each data row
    for _ in 0..args.rows {
        let row: Vec<String> = col_types
            .iter()
            .map(|t| match t {
                ColType::Int => rng.random_range(0..myrange).to_string(),
                ColType::Float => format!("{:.3}", rng.random_range(0.0..myrange as f64)),
                ColType::Word => Word().fake::<String>(),
                ColType::Name => Name().fake::<String>(),
                ColType::Phone => PhoneNumber().fake::<String>(),
            })
            .collect();
        wtr.write_record(&row).unwrap();
    }
    wtr.flush().unwrap();

    println!(
        "\tGenerated {} rows, using range {}",
        args.rows, myrange
    );
    println!("\tOutput written to: {}", output_path.display());
}

/// Prints a stylized banner to the terminal at program start.
fn show_banner() {
    let banner = r#"
    ███╗   ███╗       ███╗    ██╗  ██╗   ███████╗
    ████╗ ████║      ████╗    ██║ ██╔╝   ██╔════╝
    ██╔████╔██║     ██╔██╗    █████╔╝    █████╗ 
    ██║╚██╔╝██║    ██╔╝██╗    ██╔═██╗    ██╔══╝
    ██║ ╚═╝ ██║   ███████╗    ██║  ██╗   ███████╗
    ╚═╝     ╚═╝   ╚══════╝    ╚═╝  ╚═╝   ╚══════╝

    ██████╗        ███╗    ████████╗       ███╗
    ██╔══██╗      ████╗    ╚══██╔══╝      ████╗
    ██║  ██║     ██╔██╗       ██║        ██╔██╗
    ██║  ██║    ██╔╝██╗       ██║       ██╔╝██╗
    ██████╔╝   ███████╗       ██║      ███████╗
    ╚═════╝    ╚══════╝       ╚═╝      ╚══════╝
    Make data for testing, using fake data generator
"#;
    colour_print(banner, "cyan");
}

/// Prints colored text to the terminal using the specified color name.
fn colour_print(text: &str, colour: &str) {
    let colored_text = match colour {
        "green" => text.bright_green().bold(),
        "red" => text.bright_red().bold(),
        "cyan" => text.bright_cyan().bold(),
        "purple" => text.bright_purple().bold(),
        "blue" => text.bright_blue().bold(),
        "yellow" => text.bright_yellow().bold(),
        _ => text.bright_yellow().bold(),
    };
    println!("{}", colored_text);
}

/// Parses a comma-separated string into a vector of ColType enums.
/// Defaults to ColType::Word for unknown types.
fn parse_col_types(columns: &str) -> Vec<ColType> {
    columns
        .split(',')
        .map(|s| match s.trim().to_lowercase().as_str() {
            "int" => ColType::Int,
            "float" => ColType::Float,
            "word" => ColType::Word,
            "name" => ColType::Name,
            "phone" => ColType::Phone,
            _ => ColType::Word,
        })
        .collect()
}

/// Prints extended help and usage examples to the terminal.
fn print_big_help() {
    println!("\tColumn types can be: int, float, word, name, phone");
    println!("\tRange for random numbers is a single number (e.g., 1000)");
    colour_print(
        "\tcargo run -- --rows <number> --columns <types> --output <file> --myrange <range>",
        "green",
    );
    println!("\tExamples:");
    colour_print(
        "\tcargo run -- --rows 1000 --columns int,float,word,name,phone --output output.csv --myrange 1000",
        "green",
    );
    colour_print(
        "\tcargo run -- --rows 10 --columns int,int,int --output output.csv --myrange 10",
        "green",
    );
}

/// Ensures the output directory exists, creating it if necessary.
fn ensure_output_directory_exists(dir: &str) {
    let path = Path::new(dir);
    if !path.exists() {
        if let Err(e) = fs::create_dir(path) {
            eprintln!("Error creating directory {}: {}", dir, e);
        }
    }
}

/// Generates a unique filename in the given directory to avoid overwriting existing files.
/// If the file exists, appends a counter to the filename.
fn get_unique_filename(dir: &str, filename: &str) -> PathBuf {
    let mut path = PathBuf::from(dir).join(filename);
    let mut counter = 1;
    while path.exists() {
        let file_stem = Path::new(filename)
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy();
        let extension = Path::new(filename)
            .extension()
            .map_or("".to_string(), |ext| format!(".{}", ext.to_string_lossy()));
        let new_filename = format!("{}_{}{}", file_stem, counter, extension);
        path = PathBuf::from(dir).join(new_filename);
        counter += 1;
    }
    path
}

/// Implements ToString for ColType for easy string conversion.
impl ToString for ColType {
    fn to_string(&self) -> String {
        match self {
            ColType::Int => "int",
            ColType::Float => "float",
            ColType::Word => "word",
            ColType::Name => "name",
            ColType::Phone => "phone",
        }
        .to_string()
    }
}
