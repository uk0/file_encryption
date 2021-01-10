#![feature(num_as_ne_bytes)]
#![feature(array_methods)]
#![feature(iter_advance_by)]

use std::{fs, thread, env};
use std::io::{Write, BufReader, Read, BufRead};
use std::path::{Path, PathBuf};
use std::fs::File;
use std::ptr::null;
use std::ops::Index;
use std::str::from_utf8;
use uuid::Uuid;

use std::convert::TryFrom;

fn write_bin<P: AsRef<Path>>(vb: Vec<u8>, filename: P) -> std::io::Result<()> {
    let mut f = fs::File::create(filename)?;
    f.write(&vb)?;
    f.sync_all()?;
    Ok(())
}

use std::ptr;
use std::borrow::Borrow;

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

//F:/t/t/t/
//F:\\t\\t\\t\\

pub fn convert_path(path_str: &str) -> String {
    if cfg!(target_os = "windows") {
        String::from(path_str.replace("\\", "/"))
    } else {
        String::from(path_str)
    }
}


pub fn convert_spell() -> &'static str {
    return if cfg!(target_os = "windows") {
        "\\"
    } else {
        "/"
    };
}

pub fn _is_win_os_() -> bool {
    return convert_spell() == "\\";
}


pub fn generate_the_string(n: i32) -> String {
    let mut ans = String::new();
    if n % 2 == 0 {
        for num in (0..n - 1).rev() {
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


pub fn cover_usize_to_u8s(u: usize, ad: usize) -> [u8; 10] {
    // 99 99 99 99 99 99 // 6
    let mut str_one = u.to_string();
    // let mut data = Vec::<u8>::new();
    let mut data: Vec<u8> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    // println!("file size start  = {:}", str_one);
    let mut str2 = String::from("");
    let mut index = 0;
    let mut insert_index = 0;

    for (i, c) in str_one.chars().enumerate() {
        index = i + 1;
        if index % ad == 0 {
            str2.push(c);
            // // println!("{:}", str2);

            unsafe {
                let mut i: i8 = str2.parse().unwrap_or(0);
                std::mem::replace(&mut data[insert_index], i as u8);
            }
            insert_index = insert_index + 1;
            str2 = String::from("");
        } else {
            str2.push(c);
            if index == str_one.len() {
                unsafe {
                    let mut i: i8 = str2.parse().unwrap_or(0);
                    std::mem::replace(&mut data[insert_index], i as u8);
                }
                insert_index = insert_index + 1;
                // // println!("{:}", str2);
            }
        }
    }

    // println!("file key = {:?}", data);
    <[u8; 10]>::try_from(data).unwrap()
}


fn get_platform(types: i8) -> &'static str {
    match types {
        1 => "task_unix",
        2 => "task.exe",
        3 => "task_linux",
        _ => ""
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let base = &args[0];
    if args.len() > 1 {
        let types = &args[1];
        let key = &args[2];
        let filename = &args[3];
        let savepath = &args[4];
        let to_platform: i8 = (&args[5]).parse().unwrap();
        let platform = get_platform(to_platform);
        // println!("platform ! {:?}", platform);
        // println!("encryption start !");
        let mut result = fs::read(filename).unwrap();
        // println!("需要加密的文件长度 = {:?}", result.len());
        let mut default_key = key.as_bytes();
        let mut path_str = "";
        let mut hidd_path_str = "";
        let mut path = "";
        // 找到文件名
        if filename.find(convert_spell()).is_some() {
            let sp_lit = filename.split(convert_spell()).collect::<Vec<&str>>();
            hidd_path_str = sp_lit.get(sp_lit.len() - 1).unwrap();
        } else {
            hidd_path_str = filename
        }

        extern crate des;
        let mut data = Vec::<u8>::new();
        if types == "e" {
            // 隐藏文件名
            let merge_data = hidd_path_str.as_bytes();
            // 合并 两个数据
            unsafe { prepend_slice(&mut result, &merge_data) }
            result.insert(0, merge_data.len() as u8);
            data = des::encrypt(&result, &to_key(&default_key));

            // println!("加密数据长度{:}", data.len());
            let mut path = "";
            let mut t1 = String::from(Uuid::new_v4().to_string());
            if to_platform == 2 {
                t1.push_str(".exe");
            }
            path = t1.as_str();

            let mut base_clone = base.clone();
            // platform
            if _is_win_os_() && to_platform == 1 {
                //是windows 目标是 macos
                base_clone = str::replace(&base_clone, "task.exe", "task_unix");
            }

            if _is_win_os_() && to_platform == 2 {
                // 是 windows 目标 是 windows
                base_clone = str::replace(&base_clone, "task.exe", "task.exe");
            }

            if _is_win_os_() && to_platform == 3 {
                base_clone = str::replace(&base_clone, "task.exe", "task_linux");
            }

            if !_is_win_os_() && to_platform == 3 {
                // 不是 windows 是 macos 目标 是linux
                base_clone = str::replace(&base_clone, "task_unix", "task_linux");
            }

            if !_is_win_os_() && to_platform == 2 {
                //不是windows 是 macos 目标是 windows
                base_clone = str::replace(&base_clone, "task_unix", "task.exe");
            }

            if !_is_win_os_() && to_platform == 1 {
                //不是windows 是 macos 目标是 macos
                base_clone = str::replace(&base_clone, "task_unix", "task_unix");
            }


            let mut base_file = fs::read(base_clone).unwrap();
            let de_code_index = base_file.len();
            // 将内容写进去 二进制文件 尾巴
            base_file.write(data.as_slice());
            let mut insert_index = base_file.len();

            // println!("文件起始长度{:}", insert_index);

            // 在合并文件后最后一位存储加密文件的开始坐标
            for (i, idx) in cover_usize_to_u8s(de_code_index, 2).iter().enumerate() {
                base_file.insert(insert_index + i, *idx);
            }
            // 保存文件
            // println!("文件最终长度{:}", base_file.len());
            println!("encrypt out file {:}", path.clone());
            write_bin(base_file, convert_path(savepath) + convert_spell() + path);
        }
    } else {
        use std::io;
        use std::io::prelude::*;
        println!("{}", "Please enter password");
        let mut input = String::new();
        let stdin = io::stdin();
        stdin.lock().read_line(&mut input).unwrap();
        let mut pass = input.clone();
        let mut imput_clone = input.clone();

        // println!("{:?}", imput_clone);

        let len_withoutcrlf = imput_clone.trim_right().len();
        imput_clone.truncate(len_withoutcrlf);

        pass = imput_clone;

        // println!("{:?}", pass);

        let mut result_tmp = fs::read(base).unwrap();
        let mut index_file = String::from("");
        let mut index = 11;
        for (idx, i) in (1..index).rev().enumerate() {
            let tmp_data_len = result_tmp.get(result_tmp.len() - i).unwrap();// 获取文件名长度从1开始找
            if tmp_data_len.to_string() != "0" {
                index_file.push_str(tmp_data_len.to_string().as_str());
            }
        }
        let mut start: i64 = index_file.parse().unwrap_or(0);
        let mut result = &result_tmp[start as usize..result_tmp.len() - 10 as usize];
        // println!("result tmp_data_len {:}", result_tmp.len());
        // println!("file result len = {:?}", result.len());
        // println!("file start index = {:?}", start);
        // 从自身解决数据大小 并且解密
        let mut data = des::decrypt(&result.to_vec(), &to_key(pass.as_bytes()));
        // let mut data = des::decrypt(&result.to_vec(), &to_key("123".as_bytes()));
        // println!("decrypt file ");
        let tmp_data_len = data.get(0).unwrap();// 获取文件名长度从1开始找
        let start_usize = *tmp_data_len as usize + 1;
        //  找到文件名字
        let mut path_u8_tmp = &data[1..start_usize];
        println!("File Name = {:}", String::from_utf8(path_u8_tmp.to_vec()).unwrap());
        // 找到剩下的数据
        let temp_write = &data.clone()[start_usize..data.len()];
        // 找到文件名
        let path_str = from_utf8(path_u8_tmp).unwrap();
        println!("out file  {:}", path_str);
        //
        // // println!("tmp_data_len {:?}", temp_write);
        write_bin(temp_write.to_vec(), path_str);

        return;
    }
}