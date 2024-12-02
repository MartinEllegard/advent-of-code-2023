package part1

import (
	"slices"
	"strconv"
	"strings"
)

func ProcessPart1(lines []string) int {
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

	for i := 0; i < line_len; i++ {
		if group_a_locations[i] > group_b_locations[i] {
			result += group_a_locations[i] - group_b_locations[i]
		} else {
			result += group_b_locations[i] - group_a_locations[i]
		}
	}

	return result
}
