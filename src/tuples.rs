pub fn run() {
  let person: (&str, &str, i8) = ("Yoshi", "Tokyo", 24);

  println!("{} is from {} and is {}", person.0, person.1, person.2);
}