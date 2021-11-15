# fizzbuzz
1, 2, Fizz, 4, Buzz, Fizz, 7, 8, Fizz, Buzz!

# Rules
* Must use for.
* Must run from 1 to 1000000 inclusive.
* Must use If/elseif/else.
* Must echo/print/output i: * (eg: 15: fizzbuzz)

## Go
hyperfine -w 3 'Go/fizzbuzz'
Benchmark 1: Go/fizzbuzz
  Time (mean ± σ):      2.305 s ±  0.010 s    [User: 0.306 s, System: 2.009 s]
  Range (min … max):    2.290 s …  2.319 s    10 runs

## PHP
hyperfine -w 3 'php PHP/fizzbuzz.php'
Benchmark 1: php PHP/fizzbuzz.php
  Time (mean ± σ):      2.246 s ±  0.012 s    [User: 0.252 s, System: 1.990 s]
  Range (min … max):    2.233 s …  2.265 s    10 runs

## Rust
hyperfine -w 3 'Rust/target/rrelease/fizzbuzz'
Benchmark 1: Rust/target/release/fizzbuzz
  Time (mean ± σ):      3.507 s ±  0.011 s    [User: 0.190 s, System: 3.313 s]
  Range (min … max):    3.492 s …  3.524 s    10 runs
