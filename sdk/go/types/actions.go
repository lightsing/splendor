package types

import "encoding/json"

type playerActionType string
type takeTokenActionType string
type buyCardSourceType string

const (
	nopAction         playerActionType = "nop"
	takeTokenAction   playerActionType = "take_token"
	reserveCardAction playerActionType = "reserve_card"
	buyCardAction     playerActionType = "buy_card"

	takeThreeDifferentToken takeTokenActionType = "three_different"
	takeTwoSameToken        takeTokenActionType = "two_same"

	revealedCard buyCardSourceType = "revealed"
	reservedCard buyCardSourceType = "reserved"
)

type PlayerAction interface {
	GetType() playerActionType
}

func MarshalPlayerAction(a PlayerAction) ([]byte, error) {
	marshal := struct {
		Type   playerActionType `json:"type"`
		Action interface{}      `json:"action"`
	}{
		Type:   a.GetType(),
		Action: a,
	}
	return json.Marshal(marshal)
}

type NopAction struct{}

func (a NopAction) GetType() playerActionType {
	return nopAction
}

type TakeTokenAction struct {
	Type   takeTokenActionType `json:"type"`
	Tokens ColorVec            `json:"tokens"`
}

func (a TakeTokenAction) GetType() playerActionType {
	return takeTokenAction
}

// create a TakeTokenAction with at most three different tokens
func NewTakeThreeDifferentTokenAction(tokens []Color) TakeTokenAction {
	vec := ColorVec{}
	for _, c := range tokens {
		vec[c] = 1
	}
	return TakeTokenAction{
		Type:   takeThreeDifferentToken,
		Tokens: vec,
	}
}

// create a TakeTokenAction with two same tokens
func NewTakeTwoSameTokenAction(token Color) TakeTokenAction {
	vec := ColorVec{}
	vec[token] = 2
	return TakeTokenAction{
		Type:   takeTwoSameToken,
		Tokens: vec,
	}
}

type ReserveCardAction struct {
	Tier Tier   `json:"tier"`
	Idx  *uint8 `json:"idx"`
}

func (a ReserveCardAction) GetType() playerActionType {
	return reserveCardAction
}

// create a ReserveCardAction from the revealed cards
func NewReserveCardFromRevealedAction(tier Tier, idx uint8) ReserveCardAction {
	return ReserveCardAction{
		Tier: tier,
		Idx:  &idx,
	}
}

// create a ReserveCardAction from the card pool
func NewReserveCardFromPoolAction(tier Tier) ReserveCardAction {
	return ReserveCardAction{
		Tier: tier,
		Idx:  nil,
	}
}

type buyCardActionSource struct {
	Type     buyCardSourceType `json:"type"`
	Location interface{}       `json:"location"`
}

type BuyCardAction struct {
	Source buyCardActionSource `json:"source"`
	Uses   ColorVec            `json:"uses"`
}

func (a BuyCardAction) GetType() playerActionType {
	return buyCardAction
}

// create a BuyCardAction from the revealed cards
func NewBuyCardFromRevealedAction(tier Tier, idx uint8, uses ColorVec) BuyCardAction {
	return BuyCardAction{
		Source: buyCardActionSource{
			Type: revealedCard,
			Location: struct {
				Tier Tier  `json:"tier"`
				Idx  uint8 `json:"idx"`
			}{
				Tier: tier,
				Idx:  idx,
			},
		},
		Uses: uses,
	}
}

// create a BuyCardAction from the reserved cards
func NewBuyCardFromReservedAction(idx uint8, uses ColorVec) BuyCardAction {
	return BuyCardAction{
		Source: buyCardActionSource{
			Type:     reservedCard,
			Location: idx,
		},
		Uses: uses,
	}
}
