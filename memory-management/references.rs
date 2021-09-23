fn main() {

let s1 = String::from("hello");

let len = cal_len(&s1);
println!("len {}",len);
 

let mut c = String::from("Pavithra");
let len1 =change(&mut c);
println!("len1 {}", len1);


//dangling references

let ref_to_nothing = dangle();
}

fn dangle() -> String {
let s = String::from("pavi");

s
}

fn cal_len(str: &String) -> usize  {
 str.len()


}


fn change(name: &mut String) ->usize {
    name.push_str("Rani");
    name.len()
}
