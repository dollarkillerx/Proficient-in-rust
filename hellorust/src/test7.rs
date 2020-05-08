#[macro_export]
macro_rules! my_vec {
    ($($x: expr),*) => { // expr 表达式 * 0个或则1个
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

// #[macro_export]
// macro_rules! error {
//     ($($x: expr),*) => {
//         {
//             println!($x);
//         }
//     };
// }

pub fn test7() {
    let c = my_vec![1,2,3,4,5];
    for i in c {
        println!("c: {}",i);
    }

    // error!("ccc");
}

