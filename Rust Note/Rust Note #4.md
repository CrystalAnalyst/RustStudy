### Rust Note #4 Rust的错误处理初识

1. ?宏

   1. 作用与match处理Result枚举一摸一样

      ```rust
      let mut f = match f {
          // 打开文件成功，将file句柄赋值给f
          Ok(file) => file,
          // 打开文件失败，将错误返回(向上传播)
          Err(e) => return Err(e),
      };
      
      use std::fs::File;
      use std::io;
      use std::io::Read;
      
      fn read_username_from_file() -> Result<String, io::Error> {
          let mut f = File::open("hello.txt")?;
          let mut s = String::new();
          f.read_to_string(&mut s)?;
          Ok(s)
      }
      ```

   2. 事实上？会比match => 更胜一筹

      1. ？可以自动进行类型提升（把一个错误类型转换为另一个而错误类型）
      2. ？通过自动调用std::From特征中的from方法，完成隐式类型转换。
      3. 这种转换的好用之处在于：
         1. 可以用一个大而全的ReturnError来覆盖所有错误类型。
         2. 对于各种具体情况，只需要为各种子错误类型实现这种转换即可。