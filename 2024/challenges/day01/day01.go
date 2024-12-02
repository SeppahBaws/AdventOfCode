package challenges_01

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"sort"
	"strconv"
	"strings"
)

func Run() {
	// Read file
	file, err := os.Open("inputs/day01.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	left := []int{}
	right := []int{}

	// Read file line by line
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		numbers := getNumbers(scanner.Text())
		left = append(left, numbers[0])
		right = append(right, numbers[1])
	}

	fmt.Println("===[ Part 1 ]===")
	fmt.Printf("The total distance is: %d\n", part1(left, right))

	fmt.Println("===[ Part 2 ]===")
	fmt.Printf("The similarity score is: %d\n", part2(left, right))
}

func part1(left []int, right []int) int {
	// Sort both arrays
	sort.Ints(left)
	sort.Ints(right)

	if len(left) != len(right) {
		log.Fatal("left and right array aren't the same length!")
	}

	// Count distance
	totalDistance := 0
	for i := 0; i < len(left); i++ {
		l := left[i]
		r := right[i]

		if l < r {
			totalDistance += r - l
		} else {
			totalDistance += l - r
		}
	}

	return totalDistance
}

func part2(left []int, right []int) int {
	// Count ocurrences in right array
	freq := make(map[int]int)
	for _, num := range right {
		freq[num] = freq[num] + 1
	}

	// Calculate similarity score
	totalSimilarity := 0
	for _, num := range left {
		ocurrences, ok := freq[num]
		if ok {
			totalSimilarity += num * ocurrences
		}
	}

	return totalSimilarity
}

func getNumbers(input string) []int {
	parts := strings.Split(input, "   ")
	if len(parts) != 2 {
		panic("Expected 2 parts!")
	}

	left, err := strconv.Atoi(parts[0])
	if err != nil {
		log.Fatal(err)
	}

	right, err := strconv.Atoi(parts[1])
	if err != nil {
		log.Fatal(err)
	}

	return []int{left, right}
}
