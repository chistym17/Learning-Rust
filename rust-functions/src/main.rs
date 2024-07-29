fn main() {
    println!("Hello, world!");
    println!("{}",sum(10));
    let mine:String = String::from("TutorialsPoint");
    println!("{}",mine);
    array();


}

fn sum(sum:i32)->i32{  //function with parameters+return type
return 120+sum;

}

fn get(string:String){
    println!("{}",string);
}

fn array()
{
    let arr:[i32;4]=[4,7,8,9];
    println!("{:?}",arr);
}