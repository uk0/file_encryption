# file_encryption



#### info

```bash
# 将文件加密并且将文件名写在index = 0
# 支持 'https://github.com/substantic/rain' 
```


#### example


```bash

# 加密
./task e 11111111 0dxZNzzwEFq7PTZWWLoyLx.mp4
# 解密
./task d 11111111 67da3b95-aada-47b8-82c9-7452f2a9d5e4 



```



#### test

```bash

(base) ➜  file_encryption git:(main) ✗ md5sum test.pdf 
cbc1e3b73da2fb97206e707e5c3db35d  test.pdf
(base) ➜  file_encryption git:(main) ✗ md5sum test22222222.pdf 
cbc1e3b73da2fb97206e707e5c3db35d  test22222222.pdf

```