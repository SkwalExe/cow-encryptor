use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use std::process::{exit, Command};

// ------------------------------
// -- BEGIN RUST LOGGING SNIPPET --
// https://github.com/SkwalExe/rust-logging

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

// -- END RUST LOGGING SNIPPET --
// ------------------------------

// this function checks if cow-translator is installed on the system, else, prints an error and exit
fn check_cow_translator_exists() {
    info("checking if cow-translator is installed");
    match Command::new("cow-translator").arg("-v").output() { // try to execute cow-translator -v 
        Ok(_) => {
            // if the command is found
            success("cow-translator is installed");
        }

        Err(_) => {
            // if an error occured (not found)
            error("cow-translator is not installed, or is not in your PATH");
            error("please install cow-translator and try again");
            exit(1);
        }
    }
}

// this function check if the file path is empty 
fn check_file_path(path: &str) {

    if path.is_empty() {
        error("Please specify a file path");
        exit(1);
    }
}

// this function checks the version of cow-translator
// version 1.* or higher is required
fn check_version() {
    info("checking if cow-encryptor is up to date");
    let version = Command::new("cow-translator") // try to execute cow-translator --version-check
        .arg("--version-check")
        .output()
        .unwrap();

    let version = String::from_utf8_lossy(&version.stdout); // convert the output to a string
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
    let mut command = "default";        // the action to execute
    let mut file_path = String::new();  // the file to encrypt/decrypt
    let mut overwrite = false;          // should the program overwrite the destination file if it already exists
    let mut write = true;               // should the program write to the destination file or just print to stdout

    let mut args: Vec<String> = std::env::args().collect();
    args.remove(0); // remove the first argument, which is the name of the binary

    while args.len() > 0 { // parse args
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
            "-p" | "--print" => {
                write = false;
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
            check_version();                // check if cow-translator is up to date

            

            info(format!("Encrypting file : {}", &file_path).as_str());
            

            // the original file 
            let mut original_file = match File::open(&file_path) {
                Ok(file) => file,
                Err(e) => {
                    error(format!("Error while opening file : {}", e).as_str());
                    exit(1);
                }
            };


            let mut original_file_content = String::new(); // the content of the original file
            let encrypted_file_content: String; // the content of the original file but encrypted

            // read the original file to original_file_content
            match original_file.read_to_string(&mut original_file_content) {
                Ok(_) => success("Successfully read file"),
                Err(e) => {
                    error(format!("Error while reading file : {}", e).as_str());
                    exit(1);
                }
            };

            // encrypt the original file content
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
            

            // if -p is specified, print the encrypted file content to stdout else write it to the destination file
            if write {
                let cow_file = format!("{}.cow", &file_path); // encrypted file path

                // check if a destination file already exists
                if Path::new(cow_file.as_str()).is_file() {
            
                    // if --overwrite is not specified
                    if !overwrite {

                        error(format!("Cow file already exists : {}", &cow_file).as_str());
                        error("Please use --overwrite to overwrite the file");
                        exit(1);
                    }
                }

                // the encrypted file
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

                // write the encrypted content to the destination file
                match encrypted_file.write_all(encrypted_file_content.as_bytes()) {
                    Ok(_) => success("Successfully wrote file"),
                    Err(e) => {
                        error(format!("Error while writing file : {}", e).as_str());
                        exit(1);
                    }
                };
                success(format!("Finished : {}", &cow_file).as_str());
            } else {
                // just print to stdout
                println!("{}", encrypted_file_content);
                success("Finished");
            }

        }
        "decrypt" => {
            check_file_path(&file_path); // check if a file path is specified
            check_cow_translator_exists(); // check if cow-translator is installed
            check_version();            // check the version of cow-translator

            // check if the file is a cow file
            if !file_path.ends_with(".cow") {
                error(format!("File is not a .cow file : {}", &file_path).as_str());
                exit(1);
            }

            info(format!("Decrypting file : {}", &file_path).as_str());

            

            // the encrypted file
            let mut original_file = match File::open(&file_path) {
                Ok(file) => file,
                Err(e) => {
                    error(format!("Error while opening file : {}", e).as_str());
                    exit(1);
                }
            };

            let mut original_file_content = String::new(); // the content of the encrypted file
            let decrypted_file_content: String; // the content of the encrypted file but decrypted

            // read the encrypted file to original_file_content
            match original_file.read_to_string(&mut original_file_content) {
                Ok(_) => success("Successfully read file"),
                Err(e) => {
                    error(format!("Error while reading file : {}", e).as_str());
                    exit(1);
                }
            };

            // try to decrypt the encrypted file content
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

            // if -p is specified, print the decrypted file content to stdout else write it to the destination file
            if write {
                    
                // remove last 4 characters from the file path (.cow)
                let decrypted_file_path = file_path.to_string().trim_end_matches(".cow").to_string();
                // if the destination file already exists
                if Path::new(&decrypted_file_path).is_file() {
                    if !overwrite {
                        // if --overwrite is not specified
                        error(format!("File already exists : {}", &decrypted_file_path).as_str());
                        error("Please use --overwrite to overwrite the file");
                        exit(1);
                    }
                }
                // the destination file
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

                // write the decrypted content to the destination file
                match decrypted_file.write_all(decrypted_file_content.as_bytes()) {
                    Ok(_) => success("Successfully wrote file"),
                    Err(e) => {
                        error(format!("Error while writing file : {}", e).as_str());
                        exit(1);
                    }
                };
                success(format!("Finished : {}", &decrypted_file_path).as_str());
            } else {
                println!("{}", decrypted_file_content);
                success("Finished");
            }

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

            println!(
                "\t{}--print, -p: {}Don't write result to a file, just print it{}",
                MAGENTA, YELLOW, RESET
            );

            println!("{}━━━━━━━━━━━━━━━━━{}", MAGENTA, RESET);
            info("This program require cow-translator version 1.1.* or higher");
        }

        _ => {}
    }
}
