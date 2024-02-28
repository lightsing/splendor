package types

import "encoding/json"

type playerActionType string
type takeTokenActionType string
type buyCardSourceType string

const (
	nopAction         playerActionType = "nop"
	takeTokenAction   playerActionType = "take_tokens"
	reserveCardAction playerActionType = "reserve_card"
	buyCardAction     playerActionType = "buy_card"

	takeThreeDifferentToken takeTokenActionType = "three_different"
	takeTwoSameToken        takeTokenActionType = "two_same"

	revealedCard buyCardSourceType = "revealed"
	reservedCard buyCardSourceType = "reserved"
)

type PlayerAction interface {
	GetType() playerActionType
	MarshalJSON() ([]byte, error)
}

type NopAction struct{}

func (a NopAction) GetType() playerActionType {
	return nopAction
}

func (a NopAction) MarshalJSON() ([]byte, error) {
	return json.Marshal(struct {
		Type playerActionType `json:"type"`
	}{
		Type: nopAction,
	})
}

type TakeTokenAction struct {
	Type   takeTokenActionType
	Tokens ColorVec
}

func (a TakeTokenAction) GetType() playerActionType {
	return takeTokenAction
}

func (a TakeTokenAction) MarshalJSON() ([]byte, error) {
	action := struct {
		Type   takeTokenActionType `json:"type"`
		Tokens ColorVec            `json:"tokens"`
	}{
		Type:   a.Type,
		Tokens: a.Tokens,
	}
	marshal := struct {
		Type   playerActionType `json:"type"`
		Action interface{}      `json:"action"`
	}{
		Type:   takeTokenAction,
		Action: action,
	}
	return json.Marshal(marshal)
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
	Tier Tier
	Idx  *uint8
}

func (a ReserveCardAction) GetType() playerActionType {
	return reserveCardAction
}

func (a ReserveCardAction) MarshalJSON() ([]byte, error) {
	action := struct {
		Tier Tier   `json:"tier"`
		Idx  *uint8 `json:"idx"`
	}{
		Tier: a.Tier,
		Idx:  a.Idx,
	}
	marshal := struct {
		Type   playerActionType `json:"type"`
		Action interface{}      `json:"action"`
	}{
		Type:   reserveCardAction,
		Action: action,
	}
	return json.Marshal(marshal)
}

// NewReserveCardFromRevealedAction creates a ReserveCardAction from the revealed cards
func NewReserveCardFromRevealedAction(tier Tier, idx uint8) ReserveCardAction {
	return ReserveCardAction{
		Tier: tier,
		Idx:  &idx,
	}
}

// NewReserveCardFromPoolAction creates a ReserveCardAction from the card pool
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

func (a BuyCardAction) MarshalJSON() ([]byte, error) {
	action := struct {
		Source buyCardActionSource `json:"source"`
		Uses   ColorVec            `json:"uses"`
	}{
		Source: a.Source,
		Uses:   a.Uses,
	}
	marshal := struct {
		Type   playerActionType `json:"type"`
		Action interface{}      `json:"action"`
	}{
		Type:   buyCardAction,
		Action: action,
	}
	return json.Marshal(marshal)
}

// NewBuyCardFromRevealedAction creates a BuyCardAction from the revealed cards
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

// NewBuyCardFromReservedAction creates a BuyCardAction from the reserved cards
func NewBuyCardFromReservedAction(idx uint8, uses ColorVec) BuyCardAction {
	return BuyCardAction{
		Source: buyCardActionSource{
			Type:     reservedCard,
			Location: idx,
		},
		Uses: uses,
	}
}
