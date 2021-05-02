pub fn run() {
  let name = "Yoshi";
  let mut age = 24;
  println!("My name is {} and I am {}", name, age);
  age = 25;
  println!("My name is {} and I am {}", name, age);

  const ID: i32 = 001;
  println!("ID:{}", ID);

  let (my_name, my_age) = ("Yoshi", 24);
  println!("{} is {}", my_name, my_age);
}