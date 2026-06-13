fn custom_fib(num: i32) -> i64
{
    let mut f0 : i64 = 0;
    let mut f1 : i64 = 1;

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
    for i in 1..=20
    {
        println!("{}", custom_fib(i));
    }
}
