package types

type GameSnapshot struct {
	LastRound     bool     `json:"last_round"`
	CurrentRound  uint8    `json:"current_round"`
	CurrentPlayer uint8    `json:"current_player"`
	Tokens        ColorVec `json:"tokens"`
	CardPool      CardPool `json:"card_pool"`
	Nobles        []Noble  `json:"nobles"`
	Players       []Player `json:"players"`
}
