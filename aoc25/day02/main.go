package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
	"time"
)

func divisors(x int) []int {
	var result []int
	for i := range x {
		if i > 1 && x%i == 0 {
			result = append(result, i)
		}
	}
	return result
}

func main() {
	start := time.Now()

	file, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	scanner.Scan()
	line := scanner.Text()
	count := 0

	for _, ids := range strings.Split(line, ",") {
		bounds := strings.Split(ids, "-")
		lower, _ := strconv.Atoi(bounds[0])
		upper, _ := strconv.Atoi(bounds[1])

		for ; lower <= upper; lower++ {
			id := strconv.Itoa(lower)
			// fmt.Printf("target: %s\n", id)
			// fmt.Printf("divs: %v\n", divisors(len(id)))
			for _, width := range divisors(len(id)) {
				pattern := id[:width]
				found := true

				for x := 1; x < len(id)/width; x++ {
					if pattern != id[x*width:(x+1)*width] {
						found = false
					}
				}

				if found {
					count += lower
					// fmt.Printf("!!! Found: %d\n", lower)
					break
				}
			}
		}
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	elapsed := time.Since(start)
	log.Printf("took %s", elapsed)
	fmt.Printf("result: %d\n", count)
}
