use std::{ env::{ args, current_dir }, fs::File, io::{ BufRead, BufReader }, path::PathBuf };

const FILE_EXTENSION: &str = ".slpm";

fn main() {
    let file_path = get_file_path().expect("No valid filepath given");
    let mut file = File::open(&file_path).expect(
        format!("Failed to open {}", file_path.display()).as_str()
    );

    let tokens = get_tokens(&mut file);

    println!("{:?}", tokens);
}

fn get_file_path() -> Option<PathBuf> {
    let mut path_buf: PathBuf = current_dir().unwrap_or(PathBuf::new());
    for arg in args() {
        if !arg.ends_with(FILE_EXTENSION) {
            continue;
        }

        path_buf.push(arg);
        return Option::Some(path_buf);
    }

    Option::None
}

pub fn get_tokens(file: &mut File) -> Vec<Vec<String>> {
    let mut lines = Vec::<Vec<String>>::new();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let mut tokens = Vec::<String>::new();

        for token in line.unwrap().split_whitespace() {
            tokens.push(String::from(token));
        }

        lines.push(tokens);
    }

    lines
}
