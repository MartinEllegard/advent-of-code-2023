package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestProcessPart1(t *testing.T) {
	input, err := LoadInput(1, true)
	if err != nil {
		t.Errorf("Failed to load input: %S", err)
	}

	result := processPart1(input)

	assert.Equal(t, 0, result)
}

func BenchmarkProcessPart1(t *testing.B) {
	input, err := LoadInput(1, false)
	if err != nil {
		t.Errorf("Failed to load input: %S", err)
	}

	for i := 0; i < t.N; i++ {
		processPart1(input)
	}
}
