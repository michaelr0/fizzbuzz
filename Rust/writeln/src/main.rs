use std::io::{self, BufWriter, Write};

fn main() {
    let mut stdout = BufWriter::new(io::stdout());

    for i in 1..=1000000 {
        if i % 15 == 0 {
            writeln!(stdout, "{}: {}", i, "FizzBuzz").unwrap();
        } else if i % 3 == 0 {
            writeln!(stdout, "{}: {}", i, "Fizz").unwrap();
        } else if i % 5 == 0 {
            writeln!(stdout, "{}: {}", i, "Buzz").unwrap();
        } else {
            writeln!(stdout, "{}: {}", i, i).unwrap();
        }
    }
}
