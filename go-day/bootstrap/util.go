package main

import (
	"fmt"
	"os"
	"strings"
)

const dirpath = "AABBCCDDEE"

func LoadInput(part int32, test bool) ([]string, error) {
	var filename string

	if test {
		filename = fmt.Sprint("input_part_", part, "_example.txt")
	} else {
		filename = fmt.Sprint("input_part_", part, ".txt")
	}

	bytesRead, err := os.ReadFile(filename)
	if err != nil {
		return nil, err
	}

	fileContent := string(bytesRead)
	lines := strings.Split(fileContent, "\n")

	if lines[len(lines)-1] == "" {
		lines = lines[:len(lines)-2]
	}

	return lines, nil
}
