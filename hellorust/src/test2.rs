pub fn test2() {
    let mut add = Cache::new(|x| x + 12);
    let c1 = add.add(12);
    let c2 = add.add(13);
    println!("C1: {} C2: {}",c1,c2);
}

struct Cache<T>
    where T: Fn(u32) -> u32
{
    add: T,
    value: Option<u32>,
}

impl <T> Cache <T>
    where T: Fn(u32) -> u32
{
    fn new(fa: T) -> Cache<T> {
        Cache{
            add:fa,
            value:None,
        }
    }

    fn add(&mut self, i: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.add)(i);
                self.value = Some(v);
                v
            },
        }
    }
}