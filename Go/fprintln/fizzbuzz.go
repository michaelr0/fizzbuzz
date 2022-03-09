package main

import "bufio"
import "fmt"
import "os"

func main() {
	buf := bufio.NewWriter(os.Stdout)

	defer buf.Flush()

	for i := 1; i <= 1000000; i++ {
		if i % 15 == 0 {
			fmt.Fprintln(buf, i, ":", "FizzBuzz")
		} else if i % 3 == 0 {
			fmt.Fprintln(buf, i, ":", "Fizz")
		} else if i % 5 == 0 {
			fmt.Fprintln(buf, i, ":", "Buzz")
		} else {
			fmt.Fprintln(buf, i, ":", i)
		}
	}
}
