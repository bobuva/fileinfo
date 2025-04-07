use std::env;

fn main() {

    let args: Vec<String> = env::args().collect();
    let filename = parse_config(&args);
    let filename = filename.trim();

    if args.len() < 2 {
        println!("usage: fileinfo filename");
        return;
    }

    println!("FILENAME: {}", filename);

    // get file extension and print error message if there is no extension
    if filename.contains('.') == false {
        println!("no filename extension. just dump as if text file.");
    }

    // error if file is a known binary extension
    let extension = filename.split('.').last().unwrap();
    if extension == "exe" || extension == "dll" || extension == "so" {
        println!("The file, {}, is binary. Exiting.", filename);
        return;
    }

    let file = std::fs::OpenOptions::new().read(true).open(filename);
    match file {
        Ok(_) => {
            // dump the contents of the file if it is all UTF-8
            let contents = std::fs::read_to_string(filename).unwrap();
            if contents.is_empty() || contents.is_ascii() || contents.chars().all(|c| c.is_alphabetic()
                || c.is_ascii() || c.is_numeric() || c.is_whitespace()) {
                println!("{}", contents);
            } else {
                println!("The file, {}, contains non-UTF-8 characters. Exiting.", filename);
            }
        }
        Err(_) => {
            println!("The file, {}, does not exist.", filename);
        }
    }
}


fn parse_config(args: &[String]) -> &str
{
    let filename = &args[1];

    filename
}