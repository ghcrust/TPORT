pub fn add(x: i32, y: i32) -> i32 {
    x + y
}

pub fn hello() -> String {
    String::from("Hello world!")
}

#[test]
fn hello_returns_hello_world(){
    let want = "Hello world!".to_string();
    let got = hello();
    assert_eq!(want, got, "hello(): want {want} got {got}");
}
#[test]
fn never_passes(){
    panic!("oh no");
}
#[test]
fn add_adds() {
    let result = add(2,2);
    assert_eq!(result, 4, "add(2,2): want 4, got {result}");
}
