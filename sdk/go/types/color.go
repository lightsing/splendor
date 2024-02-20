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

// ColorVec represent the color combinations
type ColorVec [6]uint8

func NewColorVec() *ColorVec {
	return &ColorVec{}
}

func (cv *ColorVec) Add(other *ColorVec) {
	for i := 0; i < 6; i++ {
		cv[i] += other[i]
	}
}

func (cv *ColorVec) Sub(other *ColorVec) {
	for i := 0; i < 6; i++ {
		cv[i] -= other[i]
	}
}

func (cv *ColorVec) SaturatingSub(other *ColorVec) {
	for i := 0; i < 6; i++ {
		if cv[i] < other[i] {
			cv[i] = 0
		} else {
			cv[i] -= other[i]
		}
	}
}

func (cv *ColorVec) Total() uint8 {
	var total uint8
	for i := 0; i < 6; i++ {
		total += cv[i]
	}
	return total
}
