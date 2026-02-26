use std::env;
use std::io::{self, BufRead};
use file_encryption::crypto;

fn main() {
    let args: Vec<String> = env::args().collect();
    let base = &args[0];

    if args.len() > 1 {
        let types = &args[1];
        let key = &args[2];
        let filename = &args[3];
        let savepath = &args[4];
        let to_platform: i8 = args[5].parse().unwrap();

        if types == "e" {
            let mut base_clone = base.clone();

            if crypto::is_windows() && to_platform == 1 {
                base_clone = str::replace(&base_clone, "task.exe", "task_unix");
            }
            if crypto::is_windows() && to_platform == 2 {
                base_clone = str::replace(&base_clone, "task.exe", "task.exe");
            }
            if crypto::is_windows() && to_platform == 3 {
                base_clone = str::replace(&base_clone, "task.exe", "task_linux");
            }
            if !crypto::is_windows() && to_platform == 3 {
                base_clone = str::replace(&base_clone, "task_unix", "task_linux");
            }
            if !crypto::is_windows() && to_platform == 2 {
                base_clone = str::replace(&base_clone, "task_unix", "task.exe");
            }
            if !crypto::is_windows() && to_platform == 1 {
                base_clone = str::replace(&base_clone, "task_unix", "task_unix");
            }

            match crypto::encrypt_file(filename, key, savepath, &base_clone, to_platform == 2) {
                Ok(out) => println!("encrypt out file {}", out),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
    } else {
        // Self-extracting mode
        println!("Please enter password");
        let mut input = String::new();
        io::stdin().lock().read_line(&mut input).unwrap();
        let pass = input.trim_end().to_string();

        match crypto::decrypt_file(base, &pass) {
            Ok((filename, data)) => {
                println!("File Name = {}", filename);
                println!("out file  {}", filename);
                crypto::write_bin(data, &filename).unwrap();
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}
