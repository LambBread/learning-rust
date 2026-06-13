fn main() {
    let mut count : i8 = 99;
    while count >= 1
    {
        println!("{} bottle{} of beer on the wall,", count, if count == 1 {""} else {"s"});
        println!("{} bottle{} of beer.", count, if count == 1 {""} else {"s"});
        println!("Take one down, pass it around,");
        count -= 1;
        if count <= 0
        {
            break;
        }
        else
        {
            println!("{} bottle{} of beer on the wall.\n", count, if count == 1 {""} else {"s"});
        }
    }
    println!("No more bottles of beer on the wall.\n");
    println!("No more bottles of beer on the wall,");
    println!("No more bottles of beer.");
    println!("Go to the store and buy some more,");
    println!("99 bottles of beer on the wall...");
    // println!("Hello, world!");
}
