pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    components: Vec<Box<dyn Draw>>, // trait 对象使用dyn 关键字描述
}

impl Screen {
    pub fn new(mg: Vec<Box<dyn Draw>>) -> Screen {
        Screen{
            components: mg,
        }
    }

    pub fn run(&self) {
        for i in self.components.iter() {
            i.draw();
        }
    }
}

// 为什么需要dyn描述呢？ 直接用泛形不香吗 使用范形是编译器会获得其类型 改变为静态 所以泛形在RUST中没有额外的开销
// but 这样会定死类型
// pub struct Screnn<T: Draw> {
//     pub components: Vec<T>,
// }

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("draw button, width = {}, height = {}, label = {}",
                 self.width, self.height, self.label);
    }
}

pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub option: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("draw selectBox, width = {}, height = {}, option = {:?}",
                 self.width, self.height, self.option);
    }
}




#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
