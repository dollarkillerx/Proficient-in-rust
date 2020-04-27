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
