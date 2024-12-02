package main

import (
	"aoc2024day1/part1"
	"aoc2024day1/part2"
	"aoc2024day1/utils"
	"log"
)

func main() {
	part_1_input, err := utils.LoadInput(1, false, false)
	if err != nil {
		log.Fatal("Failed to laod input")
	}
	part_2_input, err := utils.LoadInput(2, false, false)
	if err != nil {
		log.Fatal("Failed to laod input")
	}

	part_1_answer := part1.ProcessPart1(part_1_input)
	log.Println("Part 1: ", part_1_answer)
	part_2_answer := part2.ProcessPart2(part_2_input)
	log.Println("Part 2: ", part_2_answer)
}
