fn main(){
    let mut a = 15;
    let mut b = 20;

    a=a+b;
    b=a-b;
    a=a-b;
    println!("number after swapping {} and {}",a,b);
}