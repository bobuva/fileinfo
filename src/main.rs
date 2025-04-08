use std::env;
use std::process;

struct Config {
    instruction: String,
    file_name: String,
}

#[derive(Debug)]
enum FileType {
    Undefined,
    Text,
    Binary,
}

// TODO: How to store a list of file extensions that represent a text file? e.g., .txt, .cs, .json

fn main() {

 /*   let x = 5;
    let y: Option<i32> = None;
    let z = x + y.unwrap_or_default();
    let a :i32 = Default::default();
    println!("The value of z is: {}", z);
    println!("The value of a is: {}", a);
*/
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    let filename = config.file_name.trim();

    println!("INSTRUCTION: {}", config.instruction);
    println!("FILENAME: {}", filename);

    let mut file_type = FileType::Undefined;
    // assign file type based on extension
    let extension = filename.split('.').last().unwrap();
    if extension == "txt" {
        file_type = FileType::Text;
    } else if extension == "exe" || extension == "dll" || extension == "so" {
        file_type = FileType::Binary;
    }

    process_file(filename, file_type);
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str>
    {
        if args.len() < 3 {
            return Err("too few arguments");
        }
        let instruction = args[1].clone();
        let file_name = args[2].clone();

        Ok(Config { instruction, file_name })
    }
}

fn process_file(file_name: &str, file_type: FileType) {

    println!("Processing file: {}  of type: {:?}", file_name, file_type);

    match file_type {
        FileType::Undefined => {
            println!("The file is undefined. Exiting.");
        }
        FileType::Text => {
            println!("Dumping contents of file:\n\n");
            dump_file_contents(file_name);
        }
        FileType::Binary => {
        }
    }
}

fn dump_file_contents(file_name: &str) {
    let file = std::fs::OpenOptions::new().read(true).open(file_name);
    match file {
        Ok(_) => {
            // dump the contents of the file if it is all UTF-8
            let contents = std::fs::read_to_string(file_name).unwrap();
            if contents.is_empty() || contents.is_ascii() || contents.chars().all(|c| c.is_alphabetic()
                || c.is_ascii() || c.is_numeric() || c.is_whitespace()) {
                println!("{}", contents);
            } else {
                println!("The file, {}, contains non-UTF-8 characters. Exiting.", file_name);
            }
        }
        Err(_) => {
            println!("The file, {}, does not exist.", file_name);
        }
    }
}