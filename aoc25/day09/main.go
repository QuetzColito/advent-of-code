package main

import (
	"bufio"
	"fmt"
	"log"
	"math"
	"os"
	"strconv"
	"strings"
	"time"
)

type Tile struct {
	x, y int
}

func area(a Tile, b Tile) int {
	return int(math.Abs(float64(a.x-b.x+1))) * int(math.Abs(float64(a.y-b.y+1)))
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
	var tiles []Tile
	for scanner.Scan() {
		coords := strings.SplitN(scanner.Text(), ",", 2)
		x, _ := strconv.Atoi(coords[0])
		y, _ := strconv.Atoi(coords[1])
		tiles = append(tiles, Tile{x: x, y: y})
	}

	for _, a := range tiles {
		for _, b := range tiles {
			new_area := area(a, b)
			// fmt.Printf("%d,%d * %d,%d area: %d\n", a.x, a.y, b.x, b.y, new_area)
			if count < new_area {
				count = new_area
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
