unsafe trait Foo {
    fn foo(&self);
}

struct Bar();

unsafe impl Foo for Bar {
    fn foo(&self) {
        println!("foo");
    }
}

pub fn test4() {
    let a = Bar();
    a.foo();
}