fn main()
{
let s = String::from("hello world");

let s1 = &s[0..5];
let s2 = &s[6..10];

println!(" slice: s1 s2 {} {}",s1,s2);
}
