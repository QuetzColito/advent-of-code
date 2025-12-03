package main

import (
	"bufio"
	"fmt"
	"log"
	"math"
	"os"
	"time"
)

func main() {
	start := time.Now()

	file, err := os.Open("03_test.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	count := 0
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		runes := []rune(scanner.Text())

		var charge [12]int
		for index, char := range runes {
			current := int(char - '0')
			used := false

			for i := 0; i < len(charge); i++ {
				if used {
					charge[i] = 0
				} else if len(runes)-index >= len(charge)-i && current > charge[i] {
					charge[i] = current
					used = true
				}
			}
		}

		for index, value := range charge {
			count += int(math.Pow10(11-index)) * value
		}
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	elapsed := time.Since(start)
	log.Printf("took %s", elapsed)
	fmt.Printf("result: %d\n", count)
}
