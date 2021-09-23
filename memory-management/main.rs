fn main() {
    let mut s = String::from("hello");
    s.push_str(",world!");

    println!(" string is {}",s);
    let (s2,len) = calculate_length(s);
    println!(" string & lenght {} {}", s2, len);


}

fn calculate_length(name: String)  -> (String,usize) {
    let length = name.len();
    (name, length)
}
