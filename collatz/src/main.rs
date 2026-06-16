use std::io;

fn main() {
    println!("Enter a number:");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    let mut guess_asint : i32 = guess.trim().parse().expect("give string number");

    loop
    {
        println!("{}", guess_asint);
        if guess_asint == 1
        {
            break;
        }
        if guess_asint % 2 == 0
        {
            guess_asint /= 2;
        }
        else
        {
            guess_asint *= 3;
            guess_asint += 1;
        }
    }
}
