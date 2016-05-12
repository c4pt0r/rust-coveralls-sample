fn foo() -> i32 {
    return 1;
}

fn main() {
    println!("Hello, world! {}", foo());
}

#[test]
fn test_foo() {
    assert!(foo() == 1);
}
