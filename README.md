## file_encryption

### 初学 Rust 时候写的，可能对学习有一些用处。


#### info

```bash
# 将文件加密并且将文件名写在index = 0
# 支持 'https://github.com/substantic/rain' 
```


#### example


```bash
#lib_self_release.rs


 ./gui/bin/task_unix + type +' ' +Key + ' '+ SelectFile+ ' ' + savedir +' ' +platform
 
 ./gui/bin/task_unix e test123 /Users/firshme/testfile  /Users/firshme/Desktop/work/file_encryption/  1


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



![img](osx.png)


#### windows


![img](windows.png)

