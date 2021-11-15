<?php

for ($i = 1; $i <= 1000000; ++$i) {
    if (0 === $i % 15) {
        echo "{$i}: FizzBuzz".PHP_EOL;
    } elseif (0 === $i % 3) {
        echo "{$i}: Fizz".PHP_EOL;
    } elseif (0 === $i % 5) {
        echo "{$i}: Buzz".PHP_EOL;
    } else {
        echo "{$i}: {$i}".PHP_EOL;
    }
}
