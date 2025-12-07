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

	file, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	count := 0
	scanner := bufio.NewScanner(file)
	var cols [][]rune
	first := true
	for scanner.Scan() {
		if first {
			cols = make([][]rune, len(scanner.Text())+1)
		}
		first = false
		for i, rune := range []rune(scanner.Text()) {
			cols[i] = append(cols[i], rune)
		}
	}

	current_op := cols[0][len(cols[0])-1]
	var result int
	if current_op == '*' {
		result = 1
	} else {
		result = 0
	}
	for i, runes := range cols {
		is_empty := true
		for _, rune := range runes {
			if rune != ' ' {
				is_empty = false
			}
		}

		if is_empty {
			current_op = cols[i+1][len(cols[0])-1]
			count += result

			if current_op == '*' {
				result = 1
			} else {
				result = 0
			}
		} else {
			number := ""
			for _, rune := range runes {
				if rune != ' ' && rune != '*' && rune != '+' {
					number += string(rune)
				}
			}

			value, _ := strconv.Atoi(number)

			if string(current_op) == "*" {
				result *= value
			} else {
				result += value
			}
		}
	}

	count += result

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	elapsed := time.Since(start)
	log.Printf("took %s", elapsed)
	fmt.Printf("result: %d\n", count)
}
