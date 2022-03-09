# FizzBuzz!
1, 2, Fizz, 4, Buzz, Fizz, 7, 8, Fizz, Buzz!

This repo aims to showcase FizzBuzz in various languages and "benchmark" them as well.

# Rules
* Must use for.
* Must run from 1 to 1000000 inclusive.
* Must use If/elseif/else.
* Must echo/print/output i: * (eg: 15: fizzbuzz)

## Go
### PrintLn
```
hyperfine -w 3 'Go/println/fizzbuzz'
Benchmark 1: Go/fizzbuzz
  Time (mean ± σ):      2.714 s ±  0.011 s    [User: 0.537 s, System: 2.204 s]
  Range (min … max):    2.697 s …  2.733 s    10 runs
```

### FprintLn
```
hyperfine -w 3 'Go/fprintln/fizzbuzz'
Benchmark 1: Go/fizzbuzz
  Time (mean ± σ):     140.7 ms ±   2.9 ms    [User: 124.0 ms, System: 19.9 ms]
  Range (min … max):   133.5 ms … 143.9 ms    20 runs
```

## PHP
```
hyperfine -w 3 'php PHP/fizzbuzz.php'
Benchmark 1: php PHP/fizzbuzz.php
  Time (mean ± σ):      2.246 s ±  0.012 s    [User: 0.252 s, System: 1.990 s]
  Range (min … max):    2.233 s …  2.265 s    10 runs
```

## Rust
### PrintLn
```
hyperfine -w 3 'Rust/println/target/release/fizzbuzz'
Benchmark 1: Rust/println/target/release/fizzbuzz
  Time (mean ± σ):      3.494 s ±  0.007 s    [User: 0.190 s, System: 3.301 s]
  Range (min … max):    3.485 s …  3.507 s    10 runs
```

### WriteLn
```
hyperfine -w 3 'Rust/writeln/target/release/fizzbuzz'
Benchmark 1: Rust/writeln/target/release/fizzbuzz
  Time (mean ± σ):      45.7 ms ±   0.9 ms    [User: 32.1 ms, System: 12.9 ms]
  Range (min … max):    41.8 ms …  47.2 ms    61 runs
```
