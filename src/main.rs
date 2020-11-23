mod tools;

use std::{fs, thread, env};
use std::io::{Write, BufReader, Read};
use std::path::{Path, PathBuf};
use core::time;
use rand::{OsRng, RngCore};
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


fn main() {
    let args: Vec<String> = env::args().collect();

    let types = &args[1];
    let filename = &args[2];


    println!("encryption start !");

    let mut result = fs::read(filename).unwrap();


    let mut tmp_bytes = Vec::<u8>::new();

    println!(" 文件大小  = {:?}", result.len());
    let key = [0x13, 0x34, 0x57, 0x79, 0x9B, 0xBC, 0xDF, 0xF1];
    // key
    //  需要保持序列化
    let mut path_str = "";
    let mut hidd_path_str = "";
    let mut path = "";


    // 找到文件名
    if filename.find("/").is_some() {
        let sp_lit = filename.split("/").collect::<Vec<&str>>();
        hidd_path_str = sp_lit.get(sp_lit.len() - 1).unwrap();
    }



    extern crate des;
    // 加密或者解密后的data
    let mut data = Vec::<u8>::new();


    if types == "encrypt" {
        // 把长度找到 放到第一位
        let merge_data = hidd_path_str.as_bytes();
        // let merge_data = "test.pdf".as_bytes();
        unsafe { prepend_slice(&mut result, &merge_data) }

        // println!("{:?}",merge_data.len() as u8);
        result.insert(0, merge_data.len() as u8);
        data = des::encrypt(&result, &key);
        // println!("{:?}", data.len());
        //文件结果

        let t1  = Uuid::new_v4().to_string();
        let path = t1.as_str();



        println!("encrypt");
        // 在尾巴插入文件名
        write_bin(data, path);
    } else if types == "decrypt"  {
        data = des::decrypt(&result, &key);

        println!("decrypt");

        let tmp_data_len = data.get(0).unwrap();// 获取文件名长度从1开始找

        let start_usize = *tmp_data_len as usize + 1;
        //  找到文件名字
        let mut path_u8_tmp = &data[1..start_usize];

        println!("{:?}", path_u8_tmp);

        // 找到剩下的数据
        let temp_write = &data.clone()[start_usize..data.len()];


        // 找到文件名
        path_str = from_utf8(path_u8_tmp).unwrap();

        // println!("path_str {:}", path_str);
        //
        // println!("tmp_data_len {:?}", temp_write);

        write_bin(temp_write.to_vec(), path_str);

        return

    }else if types == "getname"{
        data = des::decrypt(&result, &key);
        let tmp_data_len = data.get(0).unwrap();// 获取文件名长度从1开始找
        let start_usize = *tmp_data_len as usize + 1;
        //  找到文件名字
        let mut path_u8_tmp = &data[1..start_usize];
        path_str = from_utf8(path_u8_tmp).unwrap();
        println!("file name = {:}", path_str);
    }
}
