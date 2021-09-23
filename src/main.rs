//Main program
fn main() {
    println!("Hello, world!");
    println!("{} days", 31);
    println!("{0}, this is {1}. {1}, this is {0}","Alice", "Bob");
    // named arguments
    println!("{subject} {verb} {object}", object="the lazy dog", subject="the quick brown fox", verb="jumps over");
    
    //special formatting after ':'
    println!("{} of {:b} people know binary, the other half doesn't", 1,3);
    println!("{number:>0width$}", number=1, width=5);
    println!("My name is {0},{1} {0}", "Pavi", "rani");



println!("Hello {:5}!", "x");
println!("Hello {:1$}!", "x", 5);
println!("Hello {1:0$}!", 5, "x");
println!("Hello {:width$}!", "x", width = 5);


//struct
//
#[allow(dead_code)]
struct Structure(i32);
let _str = Structure(1);

println!("Strcture contains {:?}", _str.0);


//macro

macro_rules! say_hello {
    () => {
        println!("hello!");
    };
}
    say_hello!()
}
