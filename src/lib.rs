use std::{fs, thread, env};
use std::io::{Write, BufReader, Read};
use std::path::{Path, PathBuf};
use std::fs::File;
use std::ptr::null;
use std::ops::Index;
use std::str::from_utf8;
use uuid::Uuid;


fn write_bin<P: AsRef<Path>>(vb: Vec<u8>, filename: P) -> std::io::Result<()> {
    let mut f = fs::File::create(filename)?;
    f.write(&vb)?;
    f.sync_all()?;
    Ok(())
}

use std::ptr;

unsafe fn prepend_slice<T: Copy>(vec: &mut Vec<T>, slice: &[T]) {
    let len = vec.len();
    let amt = slice.len();
    vec.reserve(amt);

    ptr::copy(vec.as_ptr(),
              vec.as_mut_ptr().offset((amt) as isize),
              len);

    ptr::copy(slice.as_ptr(),
              vec.as_mut_ptr(),
              amt);

    vec.set_len(len + amt);
}

fn to_key(slice: &[u8]) -> [u8; 8] {
    let mut vec: Vec<u8> = slice.iter().cloned().collect();
    let mut key = [0; 8];
    let diff = key.len() - vec.len();
    if diff > 0 {
        vec.append(&mut vec![0; diff]);
    }
    key.clone_from_slice(&vec);
    key
}

pub fn generate_the_string(n: i32) -> String {
    let mut ans = String::new();
    if n % 2 == 0 {
        for num in (0..n-1).rev() {
            ans.push_str("a");
        }
        ans.push_str("b");
    } else {
        for num in (0..n).rev() {
            ans.push_str("a");
        }
    }
    return ans;
}


fn main(){
    let args: Vec<String> = env::args().collect();
    let types = &args[1];
    let key = &args[2];
    let filename = &args[3];
    println!("encryption start !");
    let mut result = fs::read(filename).unwrap();
    println!("File Size = {:?}", result.len());
    let mut default_key = key.as_bytes();
    let mut path_str = "";
    let mut hidd_path_str = "";
    let mut path = "";
    // 找到文件名
    if filename.find("/").is_some() {
        let sp_lit = filename.split("/").collect::<Vec<&str>>();
        hidd_path_str = sp_lit.get(sp_lit.len() - 1).unwrap();
    } else {
        hidd_path_str = filename
    }
    extern crate des;
    let mut data = Vec::<u8>::new();
    if types == "e" {
        let merge_data = hidd_path_str.as_bytes();
        // let merge_data = "test.pdf".as_bytes();
        unsafe { prepend_slice(&mut result, &merge_data) }
        // println!("{:?}",merge_data.len() as u8);
        result.insert(0, merge_data.len() as u8);
        data = des::encrypt(&result, &to_key(&default_key));
        // println!("{:?}", data.len());
        let t1 = Uuid::new_v4().to_string();
        let path = t1.as_str();
        // let path = generate_the_string(32);
        println!("encrypt out file {:}", path.clone());
        write_bin(data, path);
    } else if types == "d" {
        data = des::decrypt(&result, &to_key(&default_key));
        println!("decrypt file ");
        let tmp_data_len = data.get(0).unwrap();// 获取文件名长度从1开始找
        let start_usize = *tmp_data_len as usize + 1;
        //  找到文件名字
        let mut path_u8_tmp = &data[1..start_usize];
        println!("File Name = {:}", String::from_utf8(path_u8_tmp.to_vec()).unwrap());
        // 找到剩下的数据
        let temp_write = &data.clone()[start_usize..data.len()];
        // 找到文件名
        path_str = from_utf8(path_u8_tmp).unwrap();
        // println!("path_str {:}", path_str);
        //
        // println!("tmp_data_len {:?}", temp_write);
        write_bin(temp_write.to_vec(), path_str);

        return;
    } else if types == "g" {
        data = des::decrypt(&result, &to_key(&default_key));
        let tmp_data_len = data.get(0).unwrap();// 获取文件名长度从1开始找
        let start_usize = *tmp_data_len as usize + 1;
        //  找到文件名字
        let mut path_u8_tmp = &data[1..start_usize];
        path_str = from_utf8(path_u8_tmp).unwrap();
        println!("File Name = {:}", path_str);
    }
}