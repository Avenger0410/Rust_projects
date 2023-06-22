use std::io;
fn main(){

    let mut input = String::new();
    println!("Enter the 1st No.");
    io::stdin().read_line(&mut input).expect("Error");
    let a:i32 = input.trim().parse().expect("Error");
    //println!("{}" , input);

    let mut input2 = String::new();
    println!("Enter the 2nd No.");
    io::stdin().read_line(&mut input2).expect("Error");
    let b:i32 = input2.trim().parse().expect("Error");
    //println!("{}" , input2);

    let  sum= add_inputs( a , b);
    println!("Sum= {}" , sum);

    let mult= mult_inputs(a,b);
    println!("Multiple of these are {}", mult);

    let divide= divide_inputs(a as f32,b as f32);
    println!("Dividing these gives: {}", divide);

    let remainder= remainder_inputs(a,b);
    println!("Remainder Left: {}", remainder);
    
}

fn add_inputs(a :i32 , b:i32) -> i32  {
    //format!("{}{}" ,input,input2);
    a+b
    }
    
fn mult_inputs(a :i32 , b:i32) -> i32 {

    a*b
}

fn divide_inputs(a :f32 , b:f32) -> f32{

    a/b
}

fn remainder_inputs(a: i32, b: i32) -> i32 {

    a%b
} 