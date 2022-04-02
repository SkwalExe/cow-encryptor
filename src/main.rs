use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use std::process::{exit, Command};

const RED: &str = "\x1b[91m";
const GREEN: &str = "\x1b[92m";
const YELLOW: &str = "\x1b[93m";
const MAGENTA: &str = "\x1b[95m";
const CYAN: &str = "\x1b[96m";
const WHITE: &str = "\x1b[97m";
const RESET: &str = "\x1b[0m";
const BG_RED: &str = "\x1b[41m";
const BG_MAGENTA: &str = "\x1b[45m";
const BG_CYAN: &str = "\x1b[46m";
const BG_GREEN: &str = "\x1b[42m";

const ICON_CONNECTOR: &str = "->";
const ERROR_ICON: &str = "[ x ]";
const INFO_ICON: &str = "[ i ]";
const SUCCESS_ICON: &str = "[ v ]";
const HIGHLIGHT: bool = true;

fn success(msg: &str) {
    let msg = String::from(msg);

    let mut msg_vec = msg.split(":").collect::<Vec<&str>>();

    println!(
        "{}{} {} {}{}{}",
        GREEN,
        SUCCESS_ICON,
        ICON_CONNECTOR,
        msg_vec.remove(0),
        if msg_vec.len() > 0 {
            format!(
                ":{}{} ",
                if HIGHLIGHT {
                    format!(" {}{}", BG_GREEN, WHITE)
                } else {
                    "".to_string()
                },
                msg_vec.join(":")
            )
        } else {
            "".to_string()
        },
        RESET
    );
}

fn info(msg: &str) {
    let msg = String::from(msg);

    let mut msg_vec = msg.split(":").collect::<Vec<&str>>();

    println!(
        "{}{} {} {}{}{}",
        CYAN,
        INFO_ICON,
        ICON_CONNECTOR,
        msg_vec.remove(0),
        if msg_vec.len() > 0 {
            format!(
                ":{}{} ",
                if HIGHLIGHT {
                    format!(" {}{}", BG_CYAN, WHITE)
                } else {
                    "".to_string()
                },
                msg_vec.join(":")
            )
        } else {
            "".to_string()
        },
        RESET
    );
}

fn error(msg: &str) {
    let msg = String::from(msg);

    let mut msg_vec = msg.split(":").collect::<Vec<&str>>();

    eprintln!(
        "{}{} {} {}{}{}",
        RED,
        ERROR_ICON,
        ICON_CONNECTOR,
        msg_vec.remove(0),
        if msg_vec.len() > 0 {
            format!(
                ":{}{} ",
                if HIGHLIGHT {
                    format!(" {}{}", BG_RED, WHITE)
                } else {
                    "".to_string()
                },
                msg_vec.join(":")
            )
        } else {
            "".to_string()
        },
        RESET
    );
}

// this function checks if cow-translator is installed on the system, else, prints an error and exit
fn check_cow_translator_exists() {
    info("checking if cow-translator is installed");
    match Command::new("cow-translator").arg("-v").output() {
        Ok(_) => {
            success("cow-translator is installed");
        }

        Err(_) => {
            error("cow-translator is not installed, or is not in your PATH");
            error("please install cow-translator and try again");
            exit(1);
        }
    }
}

fn check_file_path(path: &str) {
    if path.is_empty() {
        error("Please specify a file path");
        exit(1);
    }
}

fn check_version() {
    info("checking if cow-encryptor is up to date");
    let version = Command::new("cow-translator")
        .arg("--version-check")
        .output()
        .unwrap();

    let version = String::from_utf8_lossy(&version.stdout);
    // if the version doesn't starts with 1.
    if !version.starts_with("1.") {
        error("cow-encryptor is not up to date");
        error("please update cow-encryptor and try again");
        exit(1);
    } else {
        success("cow-encryptor is up to date");
    }
}

fn main() {
    let mut command = "default";
    let mut file_path = String::new();
    let mut overwrite = false;

    let mut args: Vec<String> = std::env::args().collect();
    args.remove(0); // remove the first argument, which is the name of the binary

    while args.len() > 0 {
        match args[0].as_str() {
            "-e" | "--encrypt" => {
                command = "encrypt";
                args.remove(0);
            }

            "-d" | "--decrypt" => {
                command = "decrypt";
                args.remove(0);
            }
            "--overwrite" => {
                overwrite = true;
                args.remove(0);
            }
            "--version" | "-v" => {
                command = "version";
                args.remove(0);
            }
            "--help" | "-h" => {
                command = "help";
                args.remove(0);
            }
            _ => {
                // check if the argument is a file path

                if Path::new(&args[0]).is_file() {
                    success(format!("Foud file : {}", args[0]).as_str());
                    file_path = args.remove(0);

                    if file_path.ends_with(".cow") && command == "default" {
                        command = "decrypt";
                    }
                } else {
                    error(format!("Not a valid file path : {}", args[0]).as_str());
                    exit(1);
                }
            }
        }
    }
    match command {
        "version" => info(format!("Cow-encryptor : {}", env!("CARGO_PKG_VERSION")).as_str()),

        "default" | "encrypt" => {
            check_file_path(&file_path); // check if a file path is specified
            check_cow_translator_exists(); // check if cow-translator is installed
            check_version();

            let cow_file = format!("{}.cow", &file_path);

            if Path::new(cow_file.as_str()).is_file() {
                if !overwrite {
                    error(format!("Cow file already exists : {}", &cow_file).as_str());
                    error("Please use --overwrite to overwrite the file");
                    exit(1);
                }
            }

            info(format!("Encrypting file : {}", &file_path).as_str());
            let mut encrypted_file = match OpenOptions::new()
                .write(true)
                .truncate(true)
                .create(true)
                .open(cow_file.as_str())
            {
                Ok(file) => file,
                Err(e) => {
                    error(format!("Error while opening file : {}", e).as_str());
                    exit(1);
                }
            };

            let mut original_file = match File::open(&file_path) {
                Ok(file) => file,
                Err(e) => {
                    error(format!("Error while opening file : {}", e).as_str());
                    exit(1);
                }
            };

            let mut original_file_content = String::new();
            let encrypted_file_content: String;

            match original_file.read_to_string(&mut original_file_content) {
                Ok(_) => success("Successfully read file"),
                Err(e) => {
                    error(format!("Error while reading file : {}", e).as_str());
                    exit(1);
                }
            };

            match Command::new("cow-translator")
                .arg("-nc")
                .arg("-i")
                .arg("--")
                .arg(&original_file_content)
                .output()
            {
                Ok(output) => {
                    encrypted_file_content = String::from_utf8_lossy(&output.stdout)
                        .to_string()
                        .trim()
                        .to_string();
                    success("Successfully encrypted file content");
                }
                Err(e) => {
                    error(format!("Error while encrypting file : {}", e).as_str());
                    exit(1);
                }
            };
            match encrypted_file.write_all(encrypted_file_content.as_bytes()) {
                Ok(_) => success("Successfully wrote file"),
                Err(e) => {
                    error(format!("Error while writing file : {}", e).as_str());
                    exit(1);
                }
            };

            success(format!("Finished : {}", &cow_file).as_str());
        }
        "decrypt" => {
            check_file_path(&file_path); // check if a file path is specified
            check_cow_translator_exists(); // check if cow-translator is installed
            check_version();

            // check if the file is a cow file
            if !file_path.ends_with(".cow") {
                error(format!("File is not a .cow file : {}", &file_path).as_str());
                exit(1);
            }

            info(format!("Decrypting file : {}", &file_path).as_str());

            // remove last 4 characters from the file path
            let decrypted_file_path = file_path.to_string().trim_end_matches(".cow").to_string();
            if Path::new(&decrypted_file_path).is_file() {
                if !overwrite {
                    error(format!("File already exists : {}", &decrypted_file_path).as_str());
                    error("Please use --overwrite to overwrite the file");
                    exit(1);
                }
            }

            let mut decrypted_file = match OpenOptions::new()
                .write(true)
                .truncate(true)
                .create(true)
                .open(decrypted_file_path.as_str())
            {
                Ok(file) => file,
                Err(e) => {
                    error(format!("Error while opening file : {}", e).as_str());
                    exit(1);
                }
            };

            let mut original_file = match File::open(&file_path) {
                Ok(file) => file,
                Err(e) => {
                    error(format!("Error while opening file : {}", e).as_str());
                    exit(1);
                }
            };

            let mut original_file_content = String::new();
            let decrypted_file_content: String;

            match original_file.read_to_string(&mut original_file_content) {
                Ok(_) => success("Successfully read file"),
                Err(e) => {
                    error(format!("Error while reading file : {}", e).as_str());
                    exit(1);
                }
            };

            match Command::new("cow-translator")
                .arg("-nc")
                .arg("-t")
                .arg("--")
                .arg(&original_file_content)
                .output()
            {
                Ok(output) => {
                    decrypted_file_content = String::from_utf8_lossy(&output.stdout)
                        .to_string()
                        .trim()
                        .to_string();
                    success("Successfully decrypted file content");
                }
                Err(e) => {
                    error(format!("Error while decrypting file : {}", e).as_str());
                    exit(1);
                }
            };

            match decrypted_file.write_all(decrypted_file_content.as_bytes()) {
                Ok(_) => success("Successfully wrote file"),
                Err(e) => {
                    error(format!("Error while writing file : {}", e).as_str());
                    exit(1);
                }
            };

            success(format!("Finished : {}", &decrypted_file_path).as_str());
        }
        "help" => {
            println!("{}{} Cow-encryptor {}", BG_MAGENTA, WHITE, RESET);
            println!("{}━━━━━━━━━━━━━━━━━{}", MAGENTA, RESET);
            println!("Author: {}SkwalExe{}", MAGENTA, RESET);
            println!("Github: {}https://github.com/SkwalExe{}", MAGENTA, RESET);
            println!("{}━━━━━━━━━━━━━━━━━{}", MAGENTA, RESET);
            println!("Encrypt your files in cow language");
            println!("{}━━━━━━━━━━━━━━━━━{}", MAGENTA, RESET);
            println!("Options : ");
            println!(
                "\t{}--version, -v: {}Prints the version of the program{}",
                MAGENTA, YELLOW, RESET
            );
            println!(
                "\t{}--help, -h: {}Prints this help message{}",
                MAGENTA, YELLOW, RESET
            );
            println!(
                "\t{}--overwrite, -o: {}Overwrites the output file if it already exists{}",
                MAGENTA, YELLOW, RESET
            );

            println!(
                "\t{}--decrypt, -d: {}Decription mode{}",
                MAGENTA, YELLOW, RESET
            );

            println!(
                "\t{}--encrypt, -e: {}Encryption mode{}",
                MAGENTA, YELLOW, RESET
            );

            println!("{}━━━━━━━━━━━━━━━━━{}", MAGENTA, RESET);
            info("This program require cow-translator version 1.1.* or higher");
        }

        _ => {}
    }
}
