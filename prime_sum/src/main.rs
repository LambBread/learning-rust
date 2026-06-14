use std::io;

fn get_primes(num: i32) -> Vec<i32>
{
    let mut prime_vec : Vec<i32> = Vec::new();
    let mut is_comp_vec : Vec<bool> = Vec::new();
    for i in 0..=num
    {
        prime_vec.push(i);
        is_comp_vec.push(true);
    }
    for i in 2..=num.isqrt()
    {
        if is_comp_vec[i as usize]
        {
            let mut j = i * i;
            while j <= num
            {
                is_comp_vec[j as usize] = false;
                j += i;
            }
        }

        //for elem in &mut prime_vec
        //{
        //    if *elem % i == 0
         //   {
        //        *elem = 0;
        //    }
        //}
    }

    for i in 0..prime_vec.len()
    {
        if !is_comp_vec[i] || i == 1
        {
            prime_vec[i] = 0;
        }
    }

    
    return prime_vec
    
}

fn main() {
    println!("Enter a number:");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    let guess_asint : i32 = guess.trim().parse().expect("give string number");
    let primes = get_primes(guess_asint);
    let mut res = 0;
    for elem in &primes
    {
        res += elem;
    }
    println!("{}", res);

}
