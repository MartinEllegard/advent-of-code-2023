package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestProcessPart2(t *testing.T) {
	input, err := LoadInput(2, true)
	if err != nil {
		t.Errorf("Failed to load input: %S", err)
	}

	result := processPart2(input)

	assert.Equal(t, 0, result)
}

func BenchmarkProcessPart2(t *testing.B) {
	input, err := LoadInput(2, false)
	if err != nil {
		t.Errorf("Failed to load input: %S", err)
	}

	for i := 0; i < t.N; i++ {
		processPart2(input)
	}
}
