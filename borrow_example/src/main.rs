use std::collections::HashMap;

fn main() {
    let mut foo = String::from("Hello");
    
    //println!("{}", foo);
    let bar = &mut foo;
    bar.push_str(" World");
    
    //println!("{}", foo);
    println!("{}", bar);

    let baz = ["foo", "bar", "baz"];
    println!("Last baz: {}", baz[2]);

    let mut qux = vec!["foo", "bar"];
    println!("Last qux: {}", qux[qux.len() - 1]);
    qux.push("baz");
    println!("Last qux: {}", qux[qux.len() - 1]);
    qux.push("qux");
    qux.push("corge");

    let corge = ("foo", 30, true);
    let (corge0, corge1, corge2) = corge;

    for elem in &qux
    {
        println!("Element {}", elem);
    }

    println!("corge 0: {}", corge0);
    println!("corge 1: {}", corge1);
    println!("corge 2: {}", corge2);

    let mut grault = HashMap::new();

    grault.insert("foo", "bar");
    grault.insert("baz", "qux");
    grault.insert("corge", "grault");
    for (key, value) in &grault
    {
        println!("Key: {}, Value: {}", key, value);
    }
}
