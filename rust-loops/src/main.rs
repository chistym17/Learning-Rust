fn main() {
    println!("Hello, world!");
    for x in 1..100{
        if x%2==0
        {
            println!("{}",x);
        }
    }

    let mut x = 0;
    while x < 10{
       x+=1;
       println!("inside loop x value is {}",x);
    }

    let mut count = 0;

    for num in 0..21 {
       if num % 2==0 {
          continue;
       }
       count+=1;
    }
    println! (" The count of odd values between 0 and 20 is: {} ",count);
    
}
