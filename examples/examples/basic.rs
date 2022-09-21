use winstr::*;

fn main() {
    let standard = "Testing\0123";
    let borrowed = bstr!("Testing\0123");
    let owned = BString::from("Testing\0123");

    // breakpoint here for testing

    dbg!(standard);
    dbg!(borrowed);
    dbg!(owned);
}
