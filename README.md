#  包管理
- use 关键字用来将路径引入作用域
- pub 关键字使项变为公有
- as 关键字用于将项引入作用域时进行重命名
- 使用 glob 运算符将模块的所有内容引入作用域

// 这些代码定义了名为 sound 的模块，其包含名为 guitar 的函数。
``` 
mod sound {
    fn guitar() {
        // 函数体
    }
}

fn main() {

}
```
- 绝对路径（absolute path）从 crate 根开始，以 crate 名或者字面值 crate 开头。
- 相对路径（relative path）从当前模块开始，以 self、super 或当前模块的标识符开头。
``` 
mod sound {
    mod instrument {
        fn clarinet() {
            // 函数体
        }
    }
}

fn main() {
    // 绝对路径
    crate::sound::instrument::clarinet();

    // Relative path
    sound::instrument::clarinet();
}
```

### 作用域（scope）
- Packages 
- Crates 是一个模块的树形结构，它形成了库或二进制项目。
- Modules
- path

### 包
- src/main.rs 
- src/lib.rs
- 包可以带有多个二进制 crate，需将其文件置于 src/bin 目录
> 如果包同时包含 src/main.rs 和 src/lib.rs，那么它带有两个 crate：一个库和一个二进制项目，同名。如果只有其中之一，则包将只有一个库或者二进制 crate。包可以带有多个二进制 crate，需将其文件置于 src/bin 目录；每个文件将是一个单独的二进制 crate。


### Base1
// 所有项（函数、方法、结构体、枚举、模块和常量）默认是私有的。
// 使用 super 开始相对路径
``` 
mod cat {
    pub fn hello_world(tag: &String) {
        print(&format!("Hello {}",tag));
    }

    fn print(v: &String) {
        println!("{}",v);
    }

    pub mod xi_xi {
        pub fn hello_world(tag: &String) {
            super::print(tag); // 调用父级别
        }
    }
}

fn test1() {
    crate::cat::hello_world(&String::from("PPC"));  // 相对路径
    cat::hello_world(&String::from("HhH"));         // 绝对路径

    cat::xi_xi::hello_world(&String::from("那 那 那"));
}
```
