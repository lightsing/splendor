package types

type Player struct {
	Idx              uint8            `json:"idx"`
	Points           uint8            `json:"points"`
	Tokens           ColorVec         `json:"tokens"`
	DevelopmentCards DevelopmentCards `json:"development_cards"`
	ReservedCards    []CardView       `json:"reserved_cards"`
	Nobles           []Noble          `json:"nobles"`
}

// check if the player can buy the card, return the tokens needed if possible
func (p *Player) CanBuy(card *Card) (bool, *ColorVec) {
	effective_cost := card.Requires
	effective_cost.SaturatingSub(&p.DevelopmentCards.Bonus)
	diff := effective_cost
	diff.SaturatingSub(&p.Tokens)
	if diff.Total() > p.Tokens[YELLOW] {
		return false, nil
	}
	effective_cost.Sub(&diff)
	effective_cost[YELLOW] = diff.Total()
	return true, &effective_cost
}
