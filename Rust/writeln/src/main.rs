use std::io::{self, Write};

fn main() {
    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout);

    for i in 1..=1000000 {
        if i % 15 == 0 {
            let _ = writeln!(handle, "{}: {}", i, "FizzBuzz");
        } else if i % 3 == 0 {
            let _ = writeln!(handle, "{}: {}", i, "Fizz");
        } else if i % 5 == 0 {
            let _ = writeln!(handle, "{}: {}", i, "Buzz");
        } else {
            let _ = writeln!(handle, "{}: {}", i, i);
        }
    }
}
