use std::path::PathBuf;

pub enum Mode {
    Empty,
    File(PathBuf),
    Picker(PathBuf),
}

pub fn parse() -> Mode {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        return Mode::Empty;
    }

    let path = PathBuf::from(&args[1]);

    if path.is_dir() {
        return Mode::Picker(path);
    }

    Mode::File(path)
}
