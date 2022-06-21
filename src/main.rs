use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main()
{
    println!("Guess the number!");
    
    let secret_number = rand::thread_rng().gen_range(1..100);
    // println!("The secret number is : {}", secret_number);
    
    let mut r:i32 = 0;

    loop
    {
        println!("Please input your guess:");

        let mut guess:String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        println!("You guessed {}", guess);
        let guess:u32 = match guess.trim().parse()
        {
            Ok(num) => num,
            Err(_) => continue
        };

        match guess.cmp(&secret_number)
        {
            Ordering::Less => println!("Too low"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => 
                {
                    println!("victory!!");
                    println!("Number of chances taken are: {}", r);
                    println!("Exiting in 5 seconds....");
                    std::thread::sleep(std::time::Duration::from_secs(5));
                    break;
                }
        }
        r += 1;
    }
}
