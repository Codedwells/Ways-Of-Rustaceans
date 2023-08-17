use std::fmt::{Display, Debug};



pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{},by {}", self.headline, self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// Implimenting a trait
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}, says {}", self.username, self.content)
    }
}

// Defining a trait
pub trait Summary {
    fn summarize(&self) -> String;
}

/// Ran by caller
pub fn init() {
    let article = NewsArticle {
        author: "Jayson Monreo".to_owned(),
        headline: String::from("A way to win the tests!"),
        content: String::from("You can always win if you follow this"),
    };
    println!("{}", article.summarize());
    notify(&article);

    let tweet = Tweet {
        username: String::from("@moondev"),
        content: String::from("Today I will visit the moon and come back!"),
        reply: false,
        retweet: false,
    };
    println!("{}", tweet.summarize());
}

/// ## Traits as parameters
/// It is syntactical sugar for Trait bounds
/// # Examples
///``` 
/// pub fn notify<T:Summary>(item:&T){
///    println!("Breaking news! {}",item.summarize())
/// }
///
///```
/// This syntax becomes more useful when working with more complexity
/// ```
/// pub fn notify (item1: &impl Summary,item2:&impl Summary){
///    // ...
/// }
///
///```
/// If we want the items types to be of a single type we can do this
///```
/// pub fn notify<T:Summary>(item1:&T,item2:&T){
///     // ...
/// }
/// ```
pub fn notify (item: &impl Summary){
    println!("Breaking news! {}",item.summarize())
}

// where keyword
/// A better way to do this would be to use where
/// # Examples
///
///```
/// fn some_function<T,U> (t:&T,u:&U)-> bool
///    where T:Display + Clone,
///          U:Clone + Debug
/// {
///    // some code   
/// } 
///```
fn _some_function<T:Display + Clone,U: Clone + Debug>(_t:&T,_u:&U)->bool{
    // this hinders readability
    false
}

