# 测试
``` 
pub mod animial;

// mod animial;
// pub use animial::Dog;

#[cfg(test)]
mod test {
    use crate::animial::*;

    #[test]
    fn use_cat() {
        cat::hello();
    }

    #[test]
    fn use_dog() {
        dog::hello();
    }

}
```


run test
``` 
cargo test
```