use rand::prelude::*;

fn main() {
    let mut rng = rand::rng();

    let mut nums: Vec<i32> = (1..10000).collect();
    nums.shuffle(&mut rng);
    println!("{:?}", nums);

    let mut i = 1;
    while i < nums.len()
    {
        let mut j = i;
        while j > 0 && nums[j-1] > nums[j]
        {
            let temp = nums[j-1];
            nums[j-1] = nums[j];
            nums[j] = temp;
            j -= 1;
        }
        i += 1;
    }
    println!("\n\n{:?}", nums);


}
