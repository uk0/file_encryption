use std::fs;
use std::io::Write;
use std::path::Path;
use std::ptr;
use std::str::from_utf8;
use uuid::Uuid;

/// Write binary data to a file
pub fn write_bin<P: AsRef<Path>>(vb: Vec<u8>, filename: P) -> std::io::Result<()> {
    let mut f = fs::File::create(filename)?;
    f.write_all(&vb)?;
    f.sync_all()?;
    Ok(())
}

/// Prepend a slice to the beginning of a Vec.
///
/// # Safety
/// The caller must ensure `T` is `Copy` and `vec` has no outstanding references
/// into its buffer. The vec must not be accessed by other threads during this call.
pub unsafe fn prepend_slice<T: Copy>(vec: &mut Vec<T>, slice: &[T]) {
    let len = vec.len();
    let amt = slice.len();
    vec.reserve(amt);
    ptr::copy(vec.as_ptr(), vec.as_mut_ptr().add(amt), len);
    ptr::copy(slice.as_ptr(), vec.as_mut_ptr(), amt);
    vec.set_len(len + amt);
}

/// Convert a password slice to an 8-byte DES key
pub fn to_key(slice: &[u8]) -> [u8; 8] {
    let mut vec: Vec<u8> = slice.to_vec();
    let mut key = [0u8; 8];
    let diff = key.len().saturating_sub(vec.len());
    if diff > 0 {
        vec.append(&mut vec![0; diff]);
    }
    key.clone_from_slice(&vec[..8]);
    key
}

/// Convert path separators for current OS
pub fn convert_path(path_str: &str) -> String {
    if cfg!(target_os = "windows") {
        path_str.replace("\\", "/")
    } else {
        path_str.to_string()
    }
}

/// Get the path separator for current OS
pub fn path_separator() -> &'static str {
    if cfg!(target_os = "windows") { "\\" } else { "/" }
}

/// Check if running on Windows
pub fn is_windows() -> bool {
    path_separator() == "\\"
}

/// Convert usize to a 10-byte array representation
pub fn cover_usize_to_u8s(u: usize, ad: usize) -> [u8; 10] {
    let str_one = u.to_string();
    let mut data: Vec<u8> = vec![0; 10];
    let mut str2 = String::new();
    let mut insert_index = 0;

    for (i, c) in str_one.chars().enumerate() {
        let index = i + 1;
        if index % ad == 0 {
            str2.push(c);
            let val: i8 = str2.parse().unwrap_or(0);
            data[insert_index] = val as u8;
            insert_index += 1;
            str2.clear();
        } else {
            str2.push(c);
            if index == str_one.len() {
                let val: i8 = str2.parse().unwrap_or(0);
                data[insert_index] = val as u8;
                insert_index += 1;
            }
        }
    }
    <[u8; 10]>::try_from(data).unwrap()
}

/// Get platform binary filename
pub fn get_platform_binary(platform: i8) -> &'static str {
    match platform {
        1 => "task_unix",
        2 => "task.exe",
        3 => "task_linux",
        4 => "task_linux_arm64",
        _ => "",
    }
}

/// Extract filename from a file path
pub fn extract_filename(filepath: &str) -> String {
    let sep = path_separator();
    if filepath.contains(sep) {
        let parts: Vec<&str> = filepath.split(sep).collect();
        parts.last().unwrap_or(&filepath).to_string()
    } else if filepath.contains("/") {
        let parts: Vec<&str> = filepath.split("/").collect();
        parts.last().unwrap_or(&filepath).to_string()
    } else if filepath.contains("\\") {
        let parts: Vec<&str> = filepath.split("\\").collect();
        parts.last().unwrap_or(&filepath).to_string()
    } else {
        filepath.to_string()
    }
}

/// Encrypt a file and create a self-extracting binary.
///
/// - `input_path`: path to the file to encrypt
/// - `password`: encryption password
/// - `output_dir`: directory to save the output
/// - `platform_binary_path`: path to the platform stub binary (task_unix/task.exe/task_linux)
/// - `is_windows_target`: whether the target platform is Windows (adds .exe extension)
///
/// Returns the output filename on success.
pub fn encrypt_file(
    input_path: &str,
    password: &str,
    output_dir: &str,
    platform_binary_path: &str,
    is_windows_target: bool,
) -> Result<String, String> {
    // Read input file
    let mut result = fs::read(input_path)
        .map_err(|e| format!("Failed to read input file: {}", e))?;

    // Extract filename
    let filename = extract_filename(input_path);
    let merge_data = filename.as_bytes();

    // Prepend filename to data
    unsafe { prepend_slice(&mut result, merge_data) }
    result.insert(0, merge_data.len() as u8);

    // DES encrypt
    let data = des::encrypt(&result, &to_key(password.as_bytes()));

    // Generate output filename
    let mut out_name = Uuid::new_v4().to_string();
    if is_windows_target {
        out_name.push_str(".exe");
    }

    // Read platform stub binary
    let mut base_file = fs::read(platform_binary_path)
        .map_err(|e| format!("Failed to read platform binary '{}': {}", platform_binary_path, e))?;

    let de_code_index = base_file.len();

    // Append encrypted data to binary
    base_file
        .write_all(data.as_slice())
        .map_err(|e| format!("Failed to append data: {}", e))?;

    let insert_index = base_file.len();

    // Append 10-byte index (start position of encrypted data)
    let index_bytes = cover_usize_to_u8s(de_code_index, 2);
    for (i, idx) in index_bytes.iter().enumerate() {
        base_file.insert(insert_index + i, *idx);
    }

    // Save output file
    let out_path = format!(
        "{}{}{}",
        convert_path(output_dir),
        path_separator(),
        out_name
    );
    write_bin(base_file, &out_path)
        .map_err(|e| format!("Failed to write output: {}", e))?;

    Ok(out_name)
}

/// Decrypt a self-extracting binary file.
///
/// Returns (original_filename, decrypted_data) on success.
pub fn decrypt_file(
    encrypted_path: &str,
    password: &str,
) -> Result<(String, Vec<u8>), String> {
    let result_tmp = fs::read(encrypted_path)
        .map_err(|e| format!("Failed to read encrypted file: {}", e))?;

    // Read the 10-byte index from the end
    let mut index_str = String::new();
    for i in (1..11).rev() {
        let byte = result_tmp
            .get(result_tmp.len() - i)
            .ok_or("File too small to contain index")?;
        if byte.to_string() != "0" {
            index_str.push_str(&byte.to_string());
        }
    }

    let start: usize = index_str
        .parse()
        .map_err(|_| "Failed to parse data index".to_string())?;

    if start >= result_tmp.len() - 10 {
        return Err("Invalid encrypted file format".to_string());
    }

    let encrypted_data = &result_tmp[start..result_tmp.len() - 10];

    // DES decrypt
    let data = des::decrypt(encrypted_data, &to_key(password.as_bytes()));

    // Extract filename length and filename
    let filename_len = *data.first().ok_or("Decrypted data is empty")? as usize;
    if filename_len + 1 > data.len() {
        return Err("Invalid decrypted data format".to_string());
    }

    let filename_bytes = &data[1..filename_len + 1];
    let filename = from_utf8(filename_bytes)
        .map_err(|_| "Invalid filename in decrypted data".to_string())?
        .to_string();

    let file_data = data[filename_len + 1..].to_vec();

    Ok((filename, file_data))
}

/// Decrypt and save to a directory.
/// Returns the output file path.
pub fn decrypt_and_save(
    encrypted_path: &str,
    password: &str,
    output_dir: &str,
) -> Result<String, String> {
    let (filename, data) = decrypt_file(encrypted_path, password)?;
    let out_path = format!("{}{}{}", convert_path(output_dir), path_separator(), filename);
    write_bin(data, &out_path)
        .map_err(|e| format!("Failed to write decrypted file: {}", e))?;
    Ok(out_path)
}
