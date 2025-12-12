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

type Box struct {
	x int
	y int
	z int
}

type Connection struct {
	i    int
	j    int
	a    *Box
	b    *Box
	dist int
}

func (c Connection) show() {
	fmt.Printf("%d %d %d - %d %d %d (%d)\n", c.a.x, c.a.y, c.a.z, c.b.x, c.b.y, c.b.z, c.dist)
}

func distance(a, b Box) int {
	dx := a.x - b.x
	dy := a.y - b.y
	dz := a.z - b.z
	return dx*dx + dy*dy + dz*dz
}

func show(cs []Connection) {
	fmt.Println("[")
	for i := range len(cs) {
		cs[i].show()
	}
	fmt.Println("...")
	fmt.Println("]")
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
	var boxes []Box
	for scanner.Scan() {
		coords := strings.SplitN(scanner.Text(), ",", 3)
		x, _ := strconv.Atoi(coords[0])
		y, _ := strconv.Atoi(coords[1])
		z, _ := strconv.Atoi(coords[2])
		boxes = append(boxes, Box{x: x, y: y, z: z})
	}

	var connections []Connection
	for i, a := range boxes {
		for j := i + 1; j < len(boxes); j++ {
			conn := Connection{a: &a, b: &boxes[j], i: i, j: j, dist: distance(a, boxes[j])}
			connections = append(connections, conn)
		}
	}

	slices.SortFunc(connections, func(a Connection, b Connection) int { return a.dist - b.dist })

	show(connections[:10])

	circuits := map[int]int{}
	for i := range len(boxes) {
		circuits[i] = -1
	}
	for _, c := range connections[:1000] {
		if circuits[c.i] < 0 && circuits[c.j] < 0 {
			circuits[c.i] = c.i
			circuits[c.j] = c.j
		} else if circuits[c.i] < 0 && circuits[c.j] >= 0 {
			circuits[c.i] = circuits[c.j]
		} else if circuits[c.j] < 0 && circuits[c.i] >= 0 {
			circuits[c.j] = circuits[c.i]
		} else if circuits[c.j] >= 0 && circuits[c.i] >= 0 && circuits[c.i] != circuits[c.j] {
			to_merge := circuits[c.j]
			for x := range len(boxes) {
				if circuits[x] == to_merge {
					circuits[x] = circuits[c.i]
				}
			}
		}
	}

	sizes := make([]int, len(boxes)+1)
	for _, v := range circuits {
		if v >= 0 {
			sizes[v]++
		}
	}
	slices.Sort(sizes)

	count = (sizes[len(sizes)-1] + 1) * (sizes[len(sizes)-2] + 1) * (sizes[len(sizes)-3] + 1)

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	elapsed := time.Since(start)
	log.Printf("took %s", elapsed)
	fmt.Printf("result: %d\n", count)
}
