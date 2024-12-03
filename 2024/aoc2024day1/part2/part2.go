package part2

import (
	"slices"
	"strconv"
	"strings"
)

func ProcessPart2(lines []string) int {
	result := 0

	line_len := len(lines)

	group_a_locations := make([]int, line_len)
	group_b_locations := make([]int, line_len)

	for i := 0; i < line_len; i++ {
		line := lines[i]
		lines_values := strings.Split(line, "   ")

		a, err := strconv.Atoi(lines_values[0])
		b, err := strconv.Atoi(lines_values[1])
		if err != nil {
			continue
		}

		// Add values to the slices
		group_a_locations[i] = a
		group_b_locations[i] = b
	}

	for _, location_a := range group_a_locations {
		occurence := 0
		for _, location_b := range group_b_locations {
			if location_a == location_b {
				occurence += 1
			}
		}
		result += location_a * occurence
	}

	return result
}

func ProcessPart2v2(lines []string) int {
	result := 0

	line_len := len(lines)

	group_a_locations := make([]int, line_len)
	group_b_locations := make([]int, line_len)

	for i := 0; i < line_len; i++ {
		line := lines[i]
		lines_values := strings.Split(line, "   ")

		a, err := strconv.Atoi(lines_values[0])
		b, err := strconv.Atoi(lines_values[1])
		if err != nil {
			continue
		}

		// Add values to the slices
		group_a_locations[i] = a
		group_b_locations[i] = b
	}

	slices.Sort(group_a_locations)
	slices.Sort(group_b_locations)
	lastNumber := 0
	seeker := 0
	occurence := 0
	for i := 0; i < line_len; i++ {
		if lastNumber != 0 && lastNumber == group_a_locations[i] {
			result += group_a_locations[i] * occurence
			continue
		} else {
			occurence = 0
		}
		for y := seeker; y < line_len; y++ {
			if group_a_locations[i] == group_b_locations[y] {
				occurence += 1
			} else if group_a_locations[i] < group_b_locations[y] {
				break
			} else {
				continue
			}
		}
		result += group_a_locations[i] * occurence
		lastNumber = group_a_locations[i]
	}

	return result
}
