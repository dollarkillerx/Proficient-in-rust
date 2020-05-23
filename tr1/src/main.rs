trait Add<RHS,Output> {
    fn my_add(self,rhs: RHS) -> Output;
}

impl Add<i32,i32> for i32 {
    fn my_add(self, rhs: i32) -> i32 {
        self + rhs
    }
}

impl Add<u32,i32> for u32 {
    fn my_add(self, rhs: u32) -> i32 {
        (self + rhs) as i32
    }
}

trait Less<T> {
    fn my_less(self,par: T) -> T;
}

impl Less<i32> for i32 {
    fn my_less(self, par: i32) -> i32 {
        self - par
    }
}

//  PRO
trait LessPro<RHS=Self> {
    fn pro_less(self,rhs: RHS) -> RHS ;
}

impl LessPro for i32 {
    fn pro_less(self, rhs: Self) -> Self {
        self - rhs
    }
}
// pro 2
trait AddPro<RHS=Self> {
    type Output;
    fn pro_add(self,r: RHS) -> Self::Output;
}

impl AddPro for i32 {
    type Output = u32;

    fn pro_add(self, r: Self) -> Self::Output {
        (self + r) as Self::Output
    }
}

trait HelloWorld<T> {
    fn hello_world(&self,p: T);
}

impl HelloWorld<&str> for i32 {
    fn hello_world(&self,p: &str) {
        println!("{}",p);
    }
}

impl HelloWorld<i32> for i32 {
    fn hello_world(&self,p: i32) {
        println!("Hello World : {}",p);
    }
}

fn main() {
    let (a,b,c,d) = (3i32,5i32,7u32,90u32);
    println!("a + b : {}",a.my_add(b));
    println!("c + d : {}",d.my_add(c));

    let (e,f) = (8i32,88i32);
    println!("e - f: {}",e.my_less(f));

    let (g,h) = (7i32,98i32);
    println!("g less pro h : {}",g.pro_less(h));

    let (g,h) = (7i32,98i32);
    println!("g add pro h: {}",g.pro_add(h));

    let p = 8i32;
    p.hello_world("Hello World");
    p.hello_world(12);
}