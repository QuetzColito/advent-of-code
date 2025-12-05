package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"slices"
	"strconv"
	"strings"
	"time"
)

func merge(ranges [][2]int) ([][2]int, bool) {
	var merged [][2]int
	var consumed []int
	had_merge := false
	for i, fresh := range ranges {
		if slices.Contains(consumed, i) {
			continue
		}

		lower := fresh[0]
		upper := fresh[1]

		for j, other := range ranges {
			if j <= i || slices.Contains(consumed, j) {
				continue
			}

			lower_o := other[0]
			upper_o := other[1]

			if (lower_o <= upper && lower_o >= lower) || (upper_o >= lower && upper_o <= upper) || (lower_o < lower && upper_o > upper) {
				// fmt.Printf("%v consumes %v\n", fresh, other)
				upper = max(upper, upper_o)
				lower = min(lower, lower_o)
				consumed = append(consumed, j)
				had_merge = true
			}
		}
		merged = append(merged, [...]int{lower, upper})
	}
	return merged, had_merge
}

func show(ranges [][2]int) {
	fmt.Print("[\n")
	for _, fresh := range ranges {
		fmt.Printf("  %v\n", fresh)
	}
	fmt.Print("]\n")
}

func main() {
	start := time.Now()

	file, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	count := 0
	scanner := bufio.NewScanner(file)
	var ranges [][2]int
	for scanner.Scan() {
		if len(strings.TrimSpace(scanner.Text())) == 0 {
			break
		}
		bounds := strings.SplitN(scanner.Text(), "-", 2)
		lower, _ := strconv.Atoi(bounds[0])
		upper, _ := strconv.Atoi(bounds[1])
		ranges = append(ranges, [...]int{lower, upper})
	}

	// Part 1
	// for scanner.Scan() {
	// 	id, _ := strconv.Atoi(scanner.Text())
	// 	for _, allowed := range ranges {
	// 		if id >= allowed[0] && id <= allowed[1] {
	// 			count++
	// 			break
	// 		}
	// 	}
	// }

	// show(ranges)
	for {
		merged, had_merge := merge(ranges)
		// show(merged)
		if !had_merge {
			break
		}
		ranges = merged
	}

	for _, fresh := range ranges {
		count += fresh[1] - fresh[0] + 1
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	elapsed := time.Since(start)
	log.Printf("took %s", elapsed)
	fmt.Printf("result: %d\n", count)
}
