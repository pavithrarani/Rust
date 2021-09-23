fn main() {
   
    let list1 = vec![10,45,23,89,17,45];
    let result =largest(&list1);
    println!("result is {}", result);
}

fn largest(list : &[i32]) -> i32 {

    let mut bigno :i32 = list[0];

    for &item in list {
        if bigno <item {
            bigno =item;
        }
    }
            bigno
}
