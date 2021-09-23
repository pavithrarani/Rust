pub use crate::summary::summarize;
pub trait summary {
    fn summarize(&self) -> String;
}


struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub Author: String,
    pub content: String,
}

impl summary for NewsArticle {
    fn summarize(&self) -> String {
        format!(" {} by {} {}", self.headline, self.Author, self.location)
    }
}

struct Tweet {
    pub username: String,
    pub content: String,
    pub reply:bool,
    pub retweet:bool,
}

impl summary for Tweet {
    fn summarize (&self) -> String {
        format!("{} :{}",self.username, self.content)
    }
}


fn main()
{
  let tweet = Tweet { 
      username : String::from("horse_ebooks"),
      content : String::from("of course you can"),
      reply:false,
      retweet:false,
  };

  println!(" new tweet{}", tweet.summarize());
}
