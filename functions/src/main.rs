fn main() {
    println!("Hello, world!");
    let sum = add(5,10);
    println!("sum is {}",sum);
}


fn add(x:i32, y:i32) -> i32 {
    x+y
}
