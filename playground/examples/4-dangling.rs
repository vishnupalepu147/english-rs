use english::prelude::*;

fn main() {
    let dog = Noun::new("dog").the();
    let it = dog.pronoun(); // `it` borrows the dog
    drop(dog); // the dog leaves the story
    println!("{}", english!(it barks));
}
