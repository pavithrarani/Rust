fn main()
{

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i+1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!(" six {:?}, none {:?}", six, none);

    //_ placeholder
     let some_u8 = 0u8;

     match some_u8 {
         1 => println!("one"),
         3 => println!("three"),
         _ => (),
     }
}
