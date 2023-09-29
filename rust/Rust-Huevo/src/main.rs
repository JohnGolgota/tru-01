use std::{thread, time};
fn main() {
    let frame1 = r#"
        /\_/\
       / o o \
       \¨ ^ ¨/
        /   \   \
       /|_|_|\__/
    "#;
    let frame2 = r#"
        /\_/\
       / = = \
       \¨ ^ ¨/
        /   \    _
       /|_|_|\__/
    "#;
    let frame3 = r#"
        /\_/\
       / o o \
       \¨ ^ ¨/
        /   \    _
       /|_|_|\__/
    "#;

    let frame4 = r#"
        /\_/\
       / = = \
       \¨ w ¨/
        /   \   \
       /|_|_|\__/
"#;
    let frame5 = r#"
        /\_/\
       / o o \
       \¨ w ¨/
        /   \    _
       /|_|_|\__/
"#;

    let msg = "miau";
    let millis: u64 = 500;

    loop {
        print!("{}", frame1);
        println!("\n{} .", msg);
        thread::sleep(time::Duration::from_millis(millis));
        clear_terminal();
        print!("{}", frame2);
        println!("\n{} ..", msg);
        thread::sleep(time::Duration::from_millis(millis));
        clear_terminal();
        print!("{}", frame3);
        println!("\n{} ...", msg);
        thread::sleep(time::Duration::from_millis(millis));
        clear_terminal();
        print!("{}", frame4);
        println!("\n{} ..", msg);
        thread::sleep(time::Duration::from_millis(millis));
        clear_terminal();
        print!("{}", frame5);
        println!("\n{} .", msg);
        thread::sleep(time::Duration::from_millis(millis));
        clear_terminal();
    }
    // println!("Hello, world!");
}

fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
}