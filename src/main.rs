use fltk::{app, prelude::*, *};

fn main() {
    if std::env::args().len() < 2 {
        //println!("usage: fileinfo filename");
        //return;
        initialize_ui(None);
        return;
    }

    // get the filename as a program argument
    let filename = std::env::args().nth(1).unwrap();
    let filename = filename.trim();

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
            initialize_ui(Some(filename));

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

fn initialize_ui(filename: Option<&str>) {
    let title = construct_title(filename);

    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let mut win = window::Window::default()
        .with_size(400, 300)
        .with_label(&title); // Used the computed title here

    if filename.is_some() {
        let mut buf = text::TextBuffer::default();
        let mut txt = text::TextEditor::default()
            .with_size(400, 200);
        txt.set_buffer(buf.clone());

        let contents = std::fs::read_to_string(filename.unwrap()).unwrap();
        buf.set_text(contents.as_str());
    }
    win.end();
    win.show();


    app.run().unwrap();
}

fn construct_title(filename: Option<&str>) -> String {
    match filename {
        Some(name) => format!("File Info: {}", name), // Renamed to avoid shadowing
        None => {
            println!("No filename provided");
            "File Info".to_string()
        }
    }
}
