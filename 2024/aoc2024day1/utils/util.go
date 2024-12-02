package utils

import (
	"fmt"
	"os"
	"strings"
)

const dirpath = "2024/aoc2024day1/util.go"

func LoadInput(part int32, test bool, bench bool) ([]string, error) {
	var filename string

	if test {
		filename = fmt.Sprint("../input_part_", part, "_example.txt")
	} else if bench {
		filename = fmt.Sprint("../input_part_", part, ".txt")
	} else {
		filename = fmt.Sprint("input_part_", part, ".txt")
	}

	bytesRead, err := os.ReadFile(filename)
	if err != nil {
		return nil, err
	}

	fileContent := string(bytesRead)
	lines := strings.Split(fileContent, "\n")

	// Check if last line is empty
	if lines[len(lines)-1] == "" {
		lines = lines[:len(lines)-1]
	}

	return lines, nil
}
