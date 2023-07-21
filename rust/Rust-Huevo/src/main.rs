use std::{thread, time};
fn main() {
    let frame1 = r#"
      /\_/\
     / @ @ \
    ( >    <)
     '>>-<<'
      / o \
    "#;
    let frame2 = r#"
      /\_/\
     / o o \
    ( >    <)
     '>>-<<'
      / o \
    "#;
    let millis: u64 = 500;

    loop {
        print!("{}", frame1);
        thread::sleep(time::Duration::from_millis(millis));
        clear.terminal();
        print!("{}", frame2);
        thread::sleep(time::Duration::from_millis(millis));
        clear_terminal();
    }
    println!("Hello, world!");
}

fn clear_terminal() {
    print!(\x18[2J\x18[1;1H");
}
