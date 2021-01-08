# file_encryption



#### info

```bash
# 将文件加密并且将文件名写在index = 0
# 支持 'https://github.com/substantic/rain' 
```


#### example


```bash
# lib.rs
# 加密
./task e 11111111 0dxZNzzwEFq7PTZWWLoyLx.mp4
# 解密
./task d 11111111 67da3b95-aada-47b8-82c9-7452f2a9d5e4 

#lib_self_release.rs

# 加密
./task e 11111111 0dxZNzzwEFq7PTZWWLoyLx.mp4

# 解密
./67da3b95-aada-47b8-82c9-7452f2a9d5e4 
Please enter password
asdasd
result tmp_data_len 559218
file result len = 6512
file start index = 552696
decrypt file 
File Name = win_gui.png

```



#### test

```bash

(base) ➜  file_encryption git:(main) ✗ md5sum test.pdf 
cbc1e3b73da2fb97206e707e5c3db35d  test.pdf
(base) ➜  file_encryption git:(main) ✗ md5sum test22222222.pdf 
cbc1e3b73da2fb97206e707e5c3db35d  test22222222.pdf

```

#### build 


```bash


sudo port install x86_64-w64-mingw32-gcc

rustup target add x86_64-pc-windows-gnu

#                                                /toolchains/xxxxxxxx-apple-darwin/    
cp /opt/local/x86_64-w64-mingw32/lib/* ~/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/x86_64-pc-windows-gnu/lib/


cargo build --target x86_64-pc-windows-gnu


```



#### macos



![img](WX20210108-151253@2x.png)


#### windows


![img](win_gui.png)




##### Test


```bash
ae66ba9377dc5841ee6ec0e911027477  win_gui.png


```