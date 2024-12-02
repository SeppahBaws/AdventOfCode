package main

import (
	"fmt"
	"log"
	"os"

	challenges_01 "AdventOfCode2024/challenges/day01"
	challenges_02 "AdventOfCode2024/challenges/day02"
)

func main() {
	if len(os.Args) == 1 {
		fmt.Println("Usage: adventofcode2024 <challenge> [part]")
	}

	challengeName := os.Args[1]

	challenges := map[string]func(){
		"01": challenges_01.Run,
		"02": challenges_02.Run,
	}

	challenge, ok := challenges[challengeName]
	if !ok {
		log.Fatalf("Missing challenge '%s'", challengeName)
	}

	runChallenge(challengeName, challenge)
}

func runChallenge(name string, challenge func()) {
	fmt.Println("Welcome to Advent Of Code 2024!")
	fmt.Printf("----- %s -----\n", name)

	challenge()
}
