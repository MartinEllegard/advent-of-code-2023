package part2

import (
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
