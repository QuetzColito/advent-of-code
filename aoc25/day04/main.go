package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"time"
)

func accessible(rolls [][]bool, x int, y int) bool {
	free := 0
	for _, i := range [...]int{-1, 0, 1} {
		for _, j := range [...]int{-1, 0, 1} {
			xi := x + i
			yj := y + j

			if yj >= len(rolls) || yj < 0 || xi >= len(rolls[yj]) || xi < 0 || !rolls[yj][xi] {
				free++
			}
		}
	}
	return free >= 5
}

func deepcopy(src [][]bool) [][]bool {
	var trg [][]bool
	for _, line := range src {
		var inner []bool
		for _, x := range line {
			inner = append(inner, x)
		}
		trg = append(trg, inner)
	}
	return trg
}

func equal(a [][]bool, b [][]bool) bool {
	for y := range a {
		for x := range a[y] {
			if a[y][x] != b[y][x] {
				return false
			}
		}
	}
	return true
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
	var rolls [][]bool
	for scanner.Scan() {
		line := scanner.Text()
		var booline []bool
		for _, location := range line {
			booline = append(booline, location == '@')
		}
		rolls = append(rolls, booline)
	}

	for {
		old := deepcopy(rolls)
		for y, line := range rolls {
			for x := range line {
				if rolls[y][x] && accessible(rolls, x, y) {
					rolls[y][x] = false
					count += 1
				}
			}
		}
		if equal(old, rolls) {
			break
		}
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	elapsed := time.Since(start)
	log.Printf("took %s", elapsed)
	fmt.Printf("result: %d\n", count)
}
