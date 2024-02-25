package types

import (
	"encoding/json"
)

type Color uint8

const (
	BLACK  Color = 0
	BLUE   Color = 1
	GREEN  Color = 2
	RED    Color = 3
	WHITE  Color = 4
	YELLOW Color = 5
)

var colorNames = map[Color]string{
	BLACK:  "black",
	BLUE:   "blue",
	GREEN:  "green",
	RED:    "red",
	WHITE:  "white",
	YELLOW: "yellow",
}

var colorValues = map[string]Color{
	"black":  BLACK,
	"blue":   BLUE,
	"green":  GREEN,
	"red":    RED,
	"white":  WHITE,
	"yellow": YELLOW,
}

func (c Color) MarshalJSON() ([]byte, error) {
	if name, ok := colorNames[c]; ok {
		return json.Marshal(name)
	}
	return nil, ErrInvalidColor
}

func (c *Color) UnmarshalJSON(data []byte) error {
	var s string
	if err := json.Unmarshal(data, &s); err != nil {
		return err
	}

	if value, ok := colorValues[s]; ok {
		*c = value
		return nil
	}
	return ErrInvalidColor
}

// ColorVec represent the color combinations.
//
// Note: when comparing two ColorVecs a, b, the order is defined as:
// - a < b iff \forall i, a[i] <= b[i] and \exists j, a[j] < b[j]
// - a <= b iff \forall i, a[i] <= b[i]
// - a > b iff \forall i, a[i] >= b[i] and \exists j, a[j] > b[j]
// - a >= b iff \forall i, a[i] >= b[i]
// - otherwise a and b are not comparable
//
// Also the equality is trivially defined as the equality of the vectors.
type ColorVec [6]uint8

func NewColorVec() *ColorVec {
	return &ColorVec{}
}

// Add adds other to cv
func (cv *ColorVec) Add(other *ColorVec) {
	for i := 0; i < 6; i++ {
		cv[i] += other[i]
	}
}

// Sub subtracts other from cv
func (cv *ColorVec) Sub(other *ColorVec) {
	for i := 0; i < 6; i++ {
		cv[i] -= other[i]
	}
}

// SaturatingSub subtracts other from cv, but never goes below 0
func (cv *ColorVec) SaturatingSub(other *ColorVec) {
	for i := 0; i < 6; i++ {
		if cv[i] < other[i] {
			cv[i] = 0
		} else {
			cv[i] -= other[i]
		}
	}
}

// LessThan returns true if cv < other,
// i.e. \forall i, cv[i] <= other[i] and \exists j, cv[j] < other[j]
func (cv *ColorVec) LessThan(other *ColorVec) bool {
	le, lt := true, false
	for i := 0; i < 6; i++ {
		// \forall i, cv[i] <= other[i]
		le = le && cv[i] <= other[i]
		// \exists j, cv[j] < other[j]
		lt = lt || cv[i] < other[i]
	}
	return le && lt
}

// LessThanOrEqual returns true if cv <= other,
// i.e. \forall i, cv[i] <= other[i]
func (cv *ColorVec) LessThanOrEqual(other *ColorVec) bool {
	for i := 0; i < 6; i++ {
		if cv[i] > other[i] {
			return false
		}
	}
	return true
}

// GreaterThan returns true if cv > other,
// i.e. \forall i, cv[i] > other[i] and \exists j, cv[j] > other[j]
func (cv *ColorVec) GreaterThan(other *ColorVec) bool {
	ge, gt := true, false
	for i := 0; i < 6; i++ {
		// \forall i, cv[i] > other[i]
		ge = ge && cv[i] >= other[i]
		// \exists j, cv[j] > other[j]
		gt = gt || cv[i] > other[i]
	}
	return ge && gt
}

// GreaterThanOrEqual returns true if cv >= other,
// i.e. \forall i, cv[i] >= other[i]
func (cv *ColorVec) GreaterThanOrEqual(other *ColorVec) bool {
	for i := 0; i < 6; i++ {
		if cv[i] < other[i] {
			return false
		}
	}
	return true
}

// Total returns the total number of tokens
func (cv *ColorVec) Total() uint8 {
	var total uint8
	for i := 0; i < 6; i++ {
		total += cv[i]
	}
	return total
}
