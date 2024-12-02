package part1

import (
	"AABBCCDD/utils"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestProcessPart1(t *testing.T) {
	input, err := utils.LoadInput(1, true, false)
	if err != nil {
		t.Errorf("Failed to load input: %S", err)
	}

	result := ProcessPart1(input)

	assert.Equal(t, 0, result)
}

func BenchmarkProcessPart1(t *testing.B) {
	input, err := utils.LoadInput(1, false, true)
	if err != nil {
		t.Errorf("Failed to load input: %S", err)
	}

	for i := 0; i < t.N; i++ {
		ProcessPart1(input)
	}
}
