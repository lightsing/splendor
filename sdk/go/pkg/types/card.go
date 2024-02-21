package types

type Card struct {
	Tier     Tier     `json:"tier"`
	Color    Color    `json:"color"`
	Points   uint8    `json:"points"`
	Requires ColorVec `json:"requires"`
}

type CardPool struct {
	Remaining [3]uint8  `json:"remaining"`
	Revealed  [3][]Card `json:"revealed"`
}

const (
	VISIBLE_RESERVED_CARD   string = "visible"
	INVISIBLE_RESERVED_CARD string = "invisible"
)

type CardView struct {
	Type string      `json:"type"`
	View interface{} `json:"view"`
}

func (c *CardView) UnwrapCard() Card {
	if c.Type == VISIBLE_RESERVED_CARD {
		return c.View.(Card)
	}
	panic("cannot unwrap invisible card")
}

type DevelopmentCards struct {
	Points uint8     `json:"points"`
	Bonus  ColorVec  `json:"bonus"`
	Inner  [6][]Card `json:"inner"`
}
