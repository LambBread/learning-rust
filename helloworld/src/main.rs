fn main() {
    const PI : f64 = 3.1415926535898;
    let mut foo: &str = "bar";
    println!("PI={}", PI);
    // let foo = "bar";
    println!("Hello, {} world!", foo);
    // foo = 32;
    foo = "baz";
    println!("Hello, {} world!", foo);
}
