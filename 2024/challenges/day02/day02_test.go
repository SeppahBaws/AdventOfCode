package challenges_02

import "testing"

type TestCase struct {
	values []int
	mustBe bool
}

func TestPart1(t *testing.T) {
	cases := []TestCase{
		{
			values: []int{7, 6, 4, 2, 1},
			mustBe: true,
		},
		{
			values: []int{1, 2, 7, 8, 9},
			mustBe: false,
		},
		{
			values: []int{9, 7, 6, 2, 1},
			mustBe: false,
		},
		{
			values: []int{1, 3, 2, 4, 5},
			mustBe: false,
		},
		{
			values: []int{8, 6, 4, 4, 1},
			mustBe: false,
		},
		{
			values: []int{1, 3, 6, 7, 9},
			mustBe: true,
		},
	}

	for idx, c := range cases {
		isSafe := checkIfSafe(c.values, false)

		if isSafe != c.mustBe {
			t.Fatalf("Test case %v (%v): got %v, but expected %v", idx, c.values, isSafe, true)
		}
	}
}

func TestPart2(t *testing.T) {
	cases := []TestCase{
		{
			values: []int{7, 6, 4, 2, 1},
			mustBe: true,
		},
		{
			values: []int{1, 2, 7, 8, 9},
			mustBe: false,
		},
		{
			values: []int{9, 7, 6, 2, 1},
			mustBe: false,
		},
		{
			values: []int{1, 3, 2, 4, 5},
			mustBe: true,
		},
		{
			values: []int{8, 6, 4, 4, 1},
			mustBe: true,
		},
		{
			values: []int{1, 3, 6, 7, 9},
			mustBe: true,
		},
		{
			values: []int{19, 21, 22, 25, 25, 28},
			mustBe: true,
		},
		{
			values: []int{49, 51, 52, 55, 55, 58, 58},
			mustBe: false,
		},
	}

	for idx, c := range cases {
		isSafe := checkIfSafe(c.values, true)

		if isSafe != c.mustBe {
			t.Fatalf("Test case %v (%v): got %v, but expected %v", idx, c.values, isSafe, true)
		}
	}
}

func TestPart2EdgeCase(t *testing.T) {
	isSafe := checkIfSafe([]int{20, 16, 14, 12, 10, 8, 7, 6}, true)

	if !isSafe {
		t.Fatal("This edge case is actually correct!")
	}
}
