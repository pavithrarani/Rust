# Rust
Moving from C++ to RUST.

Skills required:
1. Object oriented programming & concepts
2. Work under linux or windows(your choice)
3. using GitBash to execute rust programs.
4. Following https://doc.rust-lang.org/book/ch06-02-match.html

Learnings:
1. Cargo build
2. Mutable & immutable variables
3. Reading inputs( storage for input : String::new()), io::stdin() .read_line(&mut variable1)
4. error handling (.expect)
5. Write output( println!(" {}", variable1)
6. use of crates (rand::Rng) similar to importing packages(java)/including libraies(c++)/include namespace(C#)
7. rand::thread_rng().gen_range(1.101);
8. match
9. parse the string input to integer
10. variable shadowing( let x =5; let x = x+1; let x = x*2; output: 12)
11. DataTypes 
    (Scalar:Integer :signed -> i8,i16,i32,i64,i128,isize  unsigned -u8,u16,u32,u64,u128, usize FloatingPoint:f32,f64..,
     bool,char,unicode support)
     (Compound: Tuple( let tup:(i32,f64) =(4,6.7); Array: let a= [1,2,3,4]
13. compile time check on integer overflow
14. Functions ( fn another_function(param1: type) =>ReturnType { body}
15. comments ( only //)
16. control flow ( if condition { body } ), multiple else if,else supported
17. conditional operator --> let no = if condition { value1} else { value2 }
18. loop { body }, while condition { body } , for ele in array.iter() { body }
19. Ownership in Rust[**TODO**] 
20. References and Borrowing [**Relook**]
21. The SLice type ( let x = String::from("hello world"); let s1 = &x[0..5];
22. structure( Tuple Struct( sturct color(i32,i32,f64)) [ Example recheck] Have error printing struct variable **#[derive(Debug)] {:?} -->dint work**)
23. Method ( impl StructureName { fn functionName(&self) { body}} Calling: structInstance.functionName())
24. Enums (declare enum, use enum as parameter to function, moving from stuct to enum (enum IpAddr { v4(String, v6(String)}
25. Option Enum (enum Option<T> { Some(T), None, } [**Relook**]

  <<Yet to explore remaining part of rust>>
  
