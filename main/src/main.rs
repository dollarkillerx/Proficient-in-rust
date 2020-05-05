use gui::{SelectBox,Screen,Button};
fn main() {
    let ui = Screen::new(vec![
        Box::new(Button{
            width: 23,
            height: 54,
            label: "Button1".to_string()
        }),
        Box::new(SelectBox{
            width: 12,
            height: 23,
            option: vec![
                "OP1".to_string(),
                "OP2".to_string(),
            ],
        }),
    ]);

    ui.run();
    println!("Hello, world!");
}
