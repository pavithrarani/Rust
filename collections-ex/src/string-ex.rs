fn main()
{
    let mut s1= String::from("foo");
    let s2 ="bar";
    s1.push_str(s2);

    println!(" final string is {}", s1);
    println!(" string slice s2 {}", s2);


    for c in "&%^&**@#".chars(){
        println!("{}",c);
    }
}
