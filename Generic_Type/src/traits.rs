fn main() {
    // Understanding Traits

    let article = NewsArticle {
        headline: String::from("Rust is awesome!"),
        location: String::from("The Internet"),
        author: String::from("John Doe"),
        content: String::from("A lot of details about Rust..."),
    };

    println!("Summary: {}", article.summarize());
}

// Here we declare a trait with  "trait" keyword name and "Summary" trait.
pub trait Summary {
    fn summarize(&self) -> String;
}

// Here we implement the trait "Summary" for the struct "NewsArticle"
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
impl Summary for NewsArticle {

    // Function
    fn summarize(&self) -> String {
        format!("{} by {} ({})", self.headline, self.author, self.location)
    }
}