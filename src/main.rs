use std::fs::read_dir;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    pattern: String,

    #[arg(short, long, default_value_t = String::from("wasm"))]
    file_type: String,

    #[arg(short, long, default_value_t = String::from("."))]
    search_path: String,
}

fn main() {

    let args = Args::parse();
    let file_type_len = args.file_type.len();
    let find_attempt = read_dir(&args.search_path).unwrap().filter_map(|entry| {
        match entry {
            Ok(val) => {
                return Some(String::from(val.file_name().to_str().unwrap()));
            }
            Err(_) => None,
        }
    }).find(|file_name| {
        let file_len = file_name.len();
        file_name.len() > file_type_len + 1 && &file_name[(file_len - (file_type_len))..] == &args.file_type 
    });

    println!("Found file name: {:?}", find_attempt);

    if let Some(resulting_file_name) = find_attempt {
        println!("What is happening");

        let change_file_path = format!("{search_path}/index.html", search_path = &args.search_path);
        let file_read_attempt = std::fs::read_to_string(&change_file_path);

        match file_read_attempt {
            Ok(file) => {
                let new_file = file.replace(&args.pattern, &resulting_file_name);
                println!("{}", new_file);
                let write_result = std::fs::write(&change_file_path, new_file);

            }
            Err(err) => println!("File read failed: {:?}", err),
        }
    }
}
