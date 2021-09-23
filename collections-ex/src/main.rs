fn main(){
    println!("vaector");

    let mut v: Vec<i32> = Vec::new();

    let v1 = vec![1,2,3];

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let third: &i32 = &v[2];

    match v.get(2) {
        Some(third) => println!("the thid element is {}", third),
        None => println!("Thre is no third element"),
    }

    println!("vec contents {}",v[0]);


}
