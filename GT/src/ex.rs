use crate::summary::summarize;

fn main(){
let tweet = Tweet{ 
    username: "pavithra",
    content : "nice work",
    reply: false,
    retweet: false,
    };
    
    println!(" summarize {}", tweet.summarize());
    }

