package main

import (
	"bufio"
	"fmt"
	"log"
	"math"
	"os"
	"slices"
	"strconv"
	"strings"
	"time"
)

func to_binary(s string) (int, int) {
	x := 0
	for _, c := range s {
		if c == '#' {
			x = x*2 + 1
		} else {
			x = x * 2
		}
	}
	return x, len(s) - 1
}

func to_binary_button(s string, length int) int {
	x := 0
	for c := range strings.SplitSeq(s, ",") {
		value, _ := strconv.Atoi(c)
		x += int(math.Pow(2, float64(length-value)))
	}
	return x
}

func unwrap(s string) string {
	return s[1 : len(s)-1]
}

func main() {
	start := time.Now()

	file, err := os.Open("input.txt")
	// file, err := os.Open("test.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	count := 0
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := strings.Fields(scanner.Text())
		desired, length := to_binary(unwrap(line[0]))
		var buttons []int
		for _, button_string := range line[1 : len(line)-1] {
			buttons = append(buttons, to_binary_button(unwrap(button_string), length))
		}
		seen := []int{0}
		states := []int{0}
		i := 0
		for !slices.Contains(states, desired) {
			var new_states []int
			for _, state := range states {
				for _, button := range buttons {
					new_state := state ^ button
					if !slices.Contains(seen, new_state) {
						seen = append(seen, new_state)
						new_states = append(new_states, new_state)
					}
				}
			}
			i++
			states = new_states
		}
		count += i
		// fmt.Printf("%d\n", desired)
		// fmt.Printf("%v\n", buttons)
		// fmt.Printf("%d\n", i)
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	elapsed := time.Since(start)
	log.Printf("took %s", elapsed)
	fmt.Printf("result: %d\n", count)
}
