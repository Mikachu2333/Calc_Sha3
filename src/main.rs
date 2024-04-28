use arboard::Clipboard;
use sha3::{Digest, Sha3_256};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        panic!("Wrong args.");
    }
    let input_path = PathBuf::from(&args[1]);
    if (!input_path.exists()) || input_path.is_dir() {
        panic!("Wrong file types.")
    }

    let name = input_path.file_name().unwrap().to_str().unwrap();
    let sha3_256 = calc_sha3_256(&input_path);
    let summary_info = format!("{} SHA3_256={}", name, sha3_256);
    set_clipboard(&summary_info);
}

fn calc_sha3_256(file_path: &PathBuf) -> String {
    let mut file = File::open(file_path).expect("无法打开文件");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("无法读取文件内容");

    let mut hasher = Sha3_256::new();
    hasher.update(&buffer);
    let result = hasher.finalize();

    format!("{:x}", result)
}

fn set_clipboard(information: &String) {
    let mut clipboard = Clipboard::new().expect("Error read clipboard.");
    clipboard.set_text(&information.to_owned()).expect("Error write clipboard.");
    println!("Success! {}",&information);
}
