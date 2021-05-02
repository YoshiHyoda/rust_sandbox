pub fn run() {
  println!("Hello from the print.rs file");

  println!("{} is from {}", "Yoshi", "Tokyo");

  println!("{0} is from {1} and {0} likes to {2}", "Yoshi", "Tokyo", "code");

  println!("{name} likes to play {activity}", name = "John", activity = "Baseball");

  println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

  println!("{:?}", (12, true, "hello"));

  println!("10 + 10 = {}", 10 + 10);
}