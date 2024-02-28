package types

import (
	"encoding/json"

	log "github.com/sirupsen/logrus"
)

// Card represents a card in the game
type Card struct {
	// The tier of the card
	Tier Tier `json:"tier"`
	// The color of the card
	Color Color `json:"color"`
	// The points of the card
	Points uint8 `json:"points"`
	// The cost of the card
	Requires ColorVec `json:"requires"`
}

// The card pool
type CardPool struct {
	// The remaining cards in each tier
	Remaining [3]uint8 `json:"remaining"`
	// The revealed cards in each tier
	Revealed [3][]Card `json:"revealed"`
}

const (
	VISIBLE_RESERVED_CARD   string = "visible"
	INVISIBLE_RESERVED_CARD string = "invisible"
)

// CardView represents a card that can be visible or not
type CardView struct {
	Type string      `json:"type"`
	View interface{} `json:"view"`
}

// UnwrapCard unwraps the card from the view, panics if the card is not visible
func (c *CardView) UnwrapCard() Card {
	if c.Type == VISIBLE_RESERVED_CARD {
		cardMashal, _ := json.Marshal(c.View)
		var card Card
		json.Unmarshal(cardMashal, &card)
		return card
	}
	log.Panic("cannot unwrap invisible card")
	return Card{} // unreachable
}

// DevelopmentCards represents the development cards of a player
type DevelopmentCards struct {
	// The total points of the development cards
	Points uint8 `json:"points"`
	// The total bonus tokens of the development cards
	Bonus ColorVec `json:"bonus"`
	// The cards grouped by color
	Inner [6][]Card `json:"inner"`
}
