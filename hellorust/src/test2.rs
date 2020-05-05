pub fn test2() {
    unsafe {
        dag_fn();
    }
}

unsafe fn dag_fn() {
    println!("ddg");
}