use std::path::PathBuf;

struct Cli {
    file1: PathBuf,
    file2: PathBuf,
}

fn main() {
    let file1 = std::env::args().nth(1).expect("no pattern given");
    let file2 = std::env::args().nth(2).expect("no path given");

    let args: Cli = Cli {
        file1: PathBuf::try_from(&file1).expect("File1 is an invalid path !"),
        file2: PathBuf::try_from(&file2).expect("File2 is an invalid path !"),
    };
    println!("pattern: {:?}, path: {:?}", file1, file2);
}
