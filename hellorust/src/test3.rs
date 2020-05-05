static mut HELLO_WORLD: &str = "Hello World";
// static 变量拥有固定的内存地址 (基地址)
// 常量则允许在任何被用到的时候复制其数据
// 静态变量是可以改变的 但是不安全的 unsafe
pub fn test3() {

    unsafe {
        HELLO_WORLD = "Stc";
        println!("{}",HELLO_WORLD);
    }

}