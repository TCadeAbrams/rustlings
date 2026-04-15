fn main() {
    let cat: (&str, f64) = ("Furry McFurson", 3.5);

    // DONE: Destructure the `cat` tuple in one statement so that the println works.
    let name = cat.0;
    let age = cat.1;

    println!("{} is {} years old", name, age);
}
