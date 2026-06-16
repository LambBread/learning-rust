struct Foo
{
    foo: String,
    bar: u32
}

impl Foo
{
    fn baz()
    {
        println!("Function Foo::baz() called");
    }

    fn qux(&mut self)
    {
        self.bar += 123;
        println!("{} {}", self.foo, self.bar);
    }

    fn corge(&self) -> u32
    {
        self.bar
    }
}

enum Bar
{
    Foo(String),
    Bar(String)
}

fn main() {
    let mut foo = Foo {
        foo: String::from("bar"),
        bar: 123
    };

    let bar = Bar::Foo(String::from("Hello World"));
    let baz = Bar::Bar(String::from("dlroW olleH"));

    match bar {
        Bar::Foo(message) => println!("Foo: {}", message),
        Bar::Bar(message) => println!("Bar: {}", message)
    }

    Foo::baz();
    foo.qux();
    println!("{}", foo.corge());
    println!("Hello, world!");
}
