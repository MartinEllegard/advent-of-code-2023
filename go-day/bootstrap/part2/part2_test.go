package part2

import (
	"AABBCCDD/utils"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestProcessPart2(t *testing.T) {
	input, err := utils.LoadInput(2, true, false)
	if err != nil {
		t.Errorf("Failed to load input: %S", err)
	}

	result := ProcessPart2(input)

	assert.Equal(t, 0, result)
}

func BenchmarkProcessPart2(t *testing.B) {
	input, err := utils.LoadInput(2, false, true)
	if err != nil {
		t.Errorf("Failed to load input: %S", err)
	}

	for i := 0; i < t.N; i++ {
		ProcessPart2(input)
	}
}
