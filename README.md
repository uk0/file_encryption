# file_encryption



#### info

```bash
# 将文件加密并且将文件名写在index = 0
# 支持 'https://github.com/substantic/rain' 
```


#### example


```bash
cargo run getname $filepath # 查看文件名

cargo run decrypt $filepath #解密

cargo run encrypt $filepath #加密

```



#### test

```bash

(base) ➜  file_encryption git:(main) ✗ md5sum test.pdf 
cbc1e3b73da2fb97206e707e5c3db35d  test.pdf
(base) ➜  file_encryption git:(main) ✗ md5sum test22222222.pdf 
cbc1e3b73da2fb97206e707e5c3db35d  test22222222.pdf

```