use std::cmp::PartialOrd;
fn largest<T : ParitalOrd> (input : &[T]) -> T {
    let mut largest = input[0];

    for &item in input {
        if largest < item {
            largest = item;
        }
    }
    largest
}

fn main() {
    let no_list = vec![10,45,23,56,12,67];
    let res = largest(&no_list);

    println!(" result is {}", res);

    let char_list = vec!['a','M', 'r','f'];
    let res1 = largest(&char_list);
    println!(" char largest is {}", res1);
    
    }
