package challenges_02

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

func Run() {
	// Read file
	file, err := os.Open("inputs/day02.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	part1Safe := 0
	part2Safe := 0

	// Read file line by line
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		str := scanner.Text()
		numbers := parseLine(str)

		if checkIfSafe(numbers, false) {
			part1Safe += 1
		}
		if checkIfSafe(numbers, true) {
			part2Safe += 1
		}
	}

	fmt.Println("===[ Part 1 ]===")
	fmt.Printf("The total amount of safe levels is: %d\n", part1Safe)

	fmt.Println("===[ Part 2 ]===")
	fmt.Printf("The total amount of safe levels is: %d\n", part2Safe)
}

type Direction int

const (
	Up   Direction = 0
	Down Direction = 1
)

func checkIfSafe(input []int, errorDampening bool) bool {
	if len(input) < 1 {
		log.Fatalln("Invalid array length")
	}

	// numbers must always be moving.
	if input[0] == input[1] {
		return false
	}

	// detect if we're going up or down
	direction := Up
	if input[1] < input[0] {
		direction = Down
	}

	prev := input[0]
	for i := 1; i < len(input); i++ {
		curr := input[i]

		distance := curr - prev

		// If we're going down, the distance will be negative, so we need to negate.
		if direction == Down {
			distance *= -1
		}

		// Handle unsafe values
		if distance < 1 || distance > 3 {
			// If we have error dampening turned on, test again with all different permutations
			// If any of them is correct, we'll assume it's ok
			// This is by far not an optimal solution but I'm too tired to think of a better one.
			if errorDampening {
				for i := 0; i < len(input); i++ {
					// Remove the nth item from the input and return a clone
					test := append(input[:i], input[i+1:]...)

					if checkIfSafe(test, false) {
						return true
					}
				}
			}

			// If we still have an error by now, we need to bail out
			return false
		}

		prev = curr
	}

	return true
}

func parseLine(input string) []int {
	parts := strings.Split(input, " ")

	var numbers = []int{}

	for _, num := range parts {
		v, err := strconv.Atoi(num)

		if err != nil {
			log.Fatal(err)
		}

		numbers = append(numbers, v)
	}

	return numbers
}
