use std::{fs, env, thread};
use std::io::Write;
use std::path::Path;
use uuid::Uuid;

use std::ptr;

fn write_bin<P: AsRef<Path>>(vb: Vec<u8>, filename: P) -> std::io::Result<()> {
    let mut f = fs::File::create(filename)?;
    f.write_all(&vb)?;
    f.sync_all()?;
    Ok(())
}

unsafe fn prepend_slice<T: Copy>(vec: &mut Vec<T>, slice: &[T]) {
    let len = vec.len();
    let amt = slice.len();
    vec.reserve(amt);

    ptr::copy(vec.as_ptr(), vec.as_mut_ptr().add(amt), len);
    ptr::copy(slice.as_ptr(), vec.as_mut_ptr(), amt);
    vec.set_len(len + amt);
}

fn to_key(slice: &[u8]) -> [u8; 8] {
    let mut vec: Vec<u8> = slice.iter().cloned().collect();
    let mut key = [0; 8];
    if vec.len() < key.len() {
        vec.extend(std::iter::repeat(0).take(key.len() - vec.len()));
    }
    key.copy_from_slice(&vec[..8]);
    key
}

fn process_file(types: &str, key: &str, filename: &str) {
    let mut result = fs::read(filename).expect("read file failed");
    let default_key = key.as_bytes();
    let mut hidd_path_str = if let Some(pos) = filename.rfind('/') {
        &filename[pos+1..]
    } else {
        filename
    };
    extern crate des;
    let mut data = Vec::<u8>::new();
    if types == "e" {
        let merge_data = hidd_path_str.as_bytes();
        unsafe { prepend_slice(&mut result, merge_data) }
        result.insert(0, merge_data.len() as u8);
        data = des::encrypt(&result, &to_key(default_key));
        let path = Uuid::new_v4().to_string();
        println!("encrypt out file {}", path);
        write_bin(data, path).unwrap();
    } else if types == "d" {
        data = des::decrypt(&result, &to_key(default_key));
        let name_len = data[0] as usize;
        let path_bytes = &data[1..name_len + 1];
        let path_str = std::str::from_utf8(path_bytes).expect("utf8");
        println!("File Name = {}", path_str);
        let temp_write = &data[name_len + 1..];
        write_bin(temp_write.to_vec(), path_str).unwrap();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        eprintln!("Usage: {} <e|d> <key> <file1,file2,...>", args[0]);
        return;
    }
    let types = args[1].clone();
    let key = args[2].clone();
    let files_arg = &args[3];
    let files: Vec<&str> = files_arg.split(',').collect();

    let mut handles = Vec::new();
    for file in files {
        let t = types.clone();
        let k = key.clone();
        let f = file.to_string();
        handles.push(thread::spawn(move || {
            process_file(&t, &k, &f);
        }));
    }

    for h in handles {
        h.join().expect("thread failed");
    }
}
