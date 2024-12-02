package main

import "log"

func main() {
	part_1_input, err := LoadInput(1, false)
	if err != nil {
		log.Fatal("Failed to laod input")
	}
	part_2_input, err := LoadInput(2, false)
	if err != nil {
		log.Fatal("Failed to laod input")
	}

	part_1_answer := processPart1(part_1_input)
	log.Println("Part 1: ", part_1_answer)
	part_2_answer := processPart2(part_2_input)
	log.Println("Part 2: ", part_2_answer)
}
