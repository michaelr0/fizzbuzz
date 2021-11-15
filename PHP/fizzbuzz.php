<?php

for ($i = 0; $i <= 1000000; ++$i) {
    if (0 === $i % 15) {
        echo 'FizzBuzz'.PHP_EOL;
    } elseif (0 === $i % 3) {
        echo 'Fizz'.PHP_EOL;
    } elseif (0 === $i % 5) {
        echo 'Buzz'.PHP_EOL;
    } else {
        echo $i.PHP_EOL;
    }
}
