package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"time"
)

func main() {
	start := time.Now()

	file, err := os.Open("01.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	dial := 50
	count := 0
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		amount, err := strconv.Atoi(line[1:])
		if err != nil {
			log.Fatal(err)
		}
		var direction int
		if line[0] == 'L' {
			direction = -1
		} else {
			direction = 1
		}
		for amount > 0 {
			amount -= 1
			dial = (dial + direction) % 100
			if dial == 0 {
				count += 1
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
