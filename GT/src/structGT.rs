struct point <T>
{

    x:T,
    y:T,
}

impl<T> point<T> {
    fn function1(&self) -> &T {
        &self.x
    }
}
struct Distance <T> {
    x:T,
    y:T,
}
impl Distance<f32> {
    fn function2(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main()
{
    let point1 = point{ x:10,y:20};
    println!("x & y {} {}", point1.x, point1.y);
    println!(" invoke structure method {}", point1.function1());

    let point2 = Distance { x:2.1, y: 3.4};
    println!(" distance is {}", point2.function2());


    }
