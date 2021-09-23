#[derive(Debug)]



struct User{
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}



fn main() {
let mut user1 = User {
    username: String::from("pavithra.be08"),
              email: String::from("pavithra.be08@gmail.com"),
              sign_in_count: 1,
              active: true,
};

struct Color(i32,i32,f64);
let black = Color(1,2,1.5);

//println!("tuple struct {:?}", black);

 let rect1 = Rectangle {
     height : 30,
     width : 50,
 };

//println!("area of rectangle {}",area(&rect1));
//println!(" struct rect {:#?}",rect1);

println!(" area of rectangle using method {}",rect1.area());
}
struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) ->u32 {
        self.width * self.height
    }
}

fn area(rectangle: &Rectangle) -> u32{
    rectangle.height * rectangle.width
}
