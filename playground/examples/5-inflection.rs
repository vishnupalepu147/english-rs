use english::prelude::*;

fn main() {
    // You write dictionary forms; "goed" and "mouses" are unrepresentable.
    println!("{}", She.verb("go").then("home"));
    println!("{}", They.past("go").then("home"));
    println!("{}", It.perfect("begin"));
    println!("{}", Noun::new("mouse").plural().the());
    println!("{}", Noun::new("hour").a());

    // A name does not tell you someone's pronouns, so a Person defaults to They.
    let ada = Person::new("Ada");
    println!("{}", english!(ada writes code. They 're good at it));
}
