mod lib;
use lib::{Tweet,Summary,NewsArticle,Display};
fn main() {

trait_test();
life_cycle1();
}
pub fn notify<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}
pub fn notify1(item: impl Summary + Display){

}
fn trait_test(){
   let tweet = Tweet {
    username: String::from("horse_ebooks"),
    content: String::from("of course, as you probably already know, people"),
    reply: false,
    retweet: false,
};

println!("1 new tweet: {}", tweet.summarize());
let news_article=NewsArticle{
    author: String::from("zhangsan"),
    content: String::from("hello course"),
    headline: String::from("----"),
    location: String::from("sichuan")
};
//println!("article:{}",news_article.summarize() );
notify(news_article);

}
//he longest string is long string is long
fn life_cycle1(){
 let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = largest2(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}
//尝试在 string2 离开作用域之后使用 result
fn life_cycle2(){
 let string1 = String::from("long string is long");
 let mut result="";

    {
        let string2 = String::from("xyz");
        result = largest2(string1.as_str(), string2.as_str());
        
    }
    println!("The longest string is {}", result);
}

fn largest2<'a>(x:&'a str,y:&'a str)->&'a str{
    if x.len()>y.len(){
        x
    }
    else{
        y
    }
}