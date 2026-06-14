fn custom_fib(num: i32) -> f64
{
    let mut f0 : f64 = 0.0;
    let mut f1 : f64 = 1.0;

    if num == 0
    {
        return f0;
    }

    if num == 1
    {
        return f1;
    }
    
    let mut new_num = num - 1;

    while new_num > 0
    {
        let old_f1 = f1;
        f1 = f0 + f1;
        f0 = old_f1;
        new_num -= 1;
    }
    return f1;
}

fn main() {
    for i in 1..=1300
    {
        println!("{}", custom_fib(i));
    }
}
