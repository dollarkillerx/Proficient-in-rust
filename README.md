# Rust
### 错误类型
- 可恢复错误
    - 在可恢复错误通常代表向用户报告错误和重试操作是合理的  未找到文件 rust中使用Result<T,E>来实现
- 不可恢复错误
    - 在不可恢复错误是bug的同义词  入尝试访问超过数组结尾的位置。rust中通过panic! 实现
    
``` 
fn main() {
    panic!("crash heil")
}

RUST_BACKTRACE=1 cargo run  打印错误堆栈信息
```

demo:
``` 
use std::fs::File;
fn main() {
    let f = File::open("src/main.rs");
    let r = match f {
        Err(e) => panic!("err: {:#?}",e),
        Ok(file) => file,
    };
}

// 简化
    let f = File::open("main.rs").expect("open file error"); // 添加自己的提示
    let f = File::open("main.rs").unwrap(); // 没有自己的提示版本
    // let f = match File::open("main.rs") {
    //     Ok(c) => c ,
    //     Err(e) => panic!("err: {:#?}",e),
    // };
```

#### 传播错误
``` 
    // pub fn read_file(self, file_path: String) -> Result<String,io::Error> {
    //     let mut f = match File::open(file_path) {
    //         Err(e) => return Err(e),
    //         Ok(f) => f,
    //     };
    //
    //     let mut data = String::new();
    //     match f.read_to_string(&mut data) {
    //         Err(e) => Err(e),
    //         Ok(_) => Ok(data),
    //     }
    // }

    // 简化版本
    pub fn read_file(self, file_path: String) -> Result<String,io::Error> {
        let mut f = File::open(file_path)?; // 如果出现问题就 返回

        let mut data = String::new();
        f.read_to_string(&mut data)?;
        Ok(data)
    }

    // 简化版本2
    pub fn read_file(self, file_path: String) -> Result<String,io::Error> {
        let mut result = String::new();
        File::open(file_path)?.read_to_string(&mut result)?;
        Ok(result)
    }
```

### 神魔时候用panic! ,神魔时候用Result
- 1. 实例 代码原型 测试 用 panic!  unwarp expect
- 2. 实际项目用Result

