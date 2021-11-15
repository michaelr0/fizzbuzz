fn main() {
    for i in 1..=1000000 {
        if i % 15 == 0 {
            println!("{}: {}", i, "FizzBuzz");
        } else if i % 3 == 0 {
            println!("{}: {}", i, "Fizz");
        } else if i % 5 == 0 {
            println!("{}: {}", i, "Buzz");
        } else {
            println!("{}: {}", i, i);
        }
    }
}
