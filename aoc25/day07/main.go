package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"slices"
	"strings"
	"time"
)

func main() {
	start := time.Now()

	file, err := os.Open("test.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	count := 0
	scanner := bufio.NewScanner(file)
	scanner.Scan()
	var splitters [][]int
	emitter := strings.Index(scanner.Text(), "S")
	ends := slices.Repeat([]int{1}, len(scanner.Text()))
	for scanner.Scan() {
		var row_splitters []int
		for i, rune := range []rune(scanner.Text()) {
			if string(rune) == "^" {
				row_splitters = append(row_splitters, i)
			}
		}
		splitters = append(splitters, row_splitters)
	}

	for y := len(splitters) - 1; y > 0; y-- {
		for _, splitter := range splitters[y] {
			ends[splitter] = ends[splitter+1] + ends[splitter-1]
		}
	}

	count = ends[emitter]

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	elapsed := time.Since(start)
	log.Printf("took %s", elapsed)
	fmt.Printf("result: %d\n", count)
}
