package naive_actors

import (
	"math/rand"
	"time"

	"github.com/lightsing/splendor/sdk/go/pkg/types"
)

type RandomActor struct {
	rng *rand.Rand
}

func NewRandomActor() *RandomActor {
	return &RandomActor{
		rng: rand.New(rand.NewSource(time.Now().UnixNano())),
	}
}

func NewRandomActorSeeded(seed int64) *RandomActor {
	return &RandomActor{
		rng: rand.New(rand.NewSource(seed)),
	}
}

func (a *RandomActor) GetAction(snapshot types.GameSnapshot) types.PlayerAction {
	currentPlayer := snapshot.Players[snapshot.CurrentPlayer]
	possibleActions := make([]types.PlayerAction, 0, 4)

	// take 3 different tokens if possible
	{
		tokens := make([]types.Color, 0, 5)
		for i := 0; i < 5; i++ {
			if snapshot.Tokens[i] > 0 {
				tokens = append(tokens, types.Color(i))
			}
		}
		if len(tokens) > 0 {
			a.rng.Shuffle(len(tokens), func(i, j int) {
				tokens[i], tokens[j] = tokens[j], tokens[i]
			})
			takes := min(3, len(tokens))
			possibleActions = append(
				possibleActions,
				types.NewTakeThreeDifferentTokenAction(tokens[0:takes]),
			)
		}
	}

	// take 2 tokens of the same color if possible
	{
		tokens := make([]types.Color, 0, 5)
		for i := 0; i < 5; i++ {
			if snapshot.Tokens[i] > 3 {
				tokens = append(tokens, types.Color(i))
			}
		}
		if len(tokens) > 0 {
			possibleActions = append(
				possibleActions,
				types.NewTakeTwoSameTokenAction(
					tokens[a.rng.Intn(len(tokens))],
				),
			)

		}
	}

	// reserve a card if possible
	if len(currentPlayer.ReservedCards) < 3 {
		possibleCards := make([]types.ReserveCardAction, 0, 16)
		for tier := 0; tier < 3; tier++ {
			n := len(snapshot.CardPool.Revealed[tier])
			for i := 0; i < n; i++ {
				possibleCards = append(
					possibleCards,
					types.NewReserveCardFromRevealedAction(types.Tier(tier), uint8(i)),
				)
			}
			if snapshot.CardPool.Remaining[tier] > 0 {
				possibleCards = append(
					possibleCards,
					types.NewReserveCardFromPoolAction(types.Tier(tier)),
				)
			}
		}
		if len(possibleCards) > 0 {
			possibleActions = append(
				possibleActions,
				possibleCards[a.rng.Intn(len(possibleCards))],
			)
		}
	}

	// buy a card if possible
	{
		possibleCards := make([]types.BuyCardAction, 0, 3*4+3)
		for tier := 0; tier < 3; tier++ {
			n := len(snapshot.CardPool.Revealed[tier])
			for i := 0; i < n; i++ {
				card := &snapshot.CardPool.Revealed[tier][i]
				if ok, uses := currentPlayer.CanBuy(card); ok {
					possibleCards = append(
						possibleCards,
						types.NewBuyCardFromRevealedAction(types.Tier(tier), uint8(i), uses),
					)
				}
			}
		}
		for i := 0; i < len(currentPlayer.ReservedCards); i++ {
			card := currentPlayer.ReservedCards[i].UnwrapCard()
			if ok, uses := currentPlayer.CanBuy(&card); ok {
				possibleCards = append(
					possibleCards,
					types.NewBuyCardFromReservedAction(uint8(i), uses),
				)
			}
		}
		if len(possibleCards) > 0 {
			possibleActions = append(
				possibleActions,
				possibleCards[a.rng.Intn(len(possibleCards))],
			)
		}
	}

	if len(possibleActions) > 0 {
		return possibleActions[a.rng.Intn(len(possibleActions))]
	}
	return types.NopAction{}
}

func (a *RandomActor) DropTokens(snapshot types.GameSnapshot) types.ColorVec {
	currentPlayer := snapshot.Players[snapshot.CurrentPlayer]
	tokens := currentPlayer.Tokens
	if tokens.Total() <= 10 {
		panic("no tokens to drop")
	}
	toDrop := tokens.Total() - 10
	drops := types.ColorVec{}
	for toDrop > 0 {
		possibleDrops := make([]types.Color, 0, 6)
		for i := 0; i < 6; i++ {
			if tokens[i] > 0 {
				possibleDrops = append(possibleDrops, types.Color(i))
			}
		}
		color := possibleDrops[a.rng.Intn(len(possibleDrops))]
		drops[color]++
		tokens[color]--
		toDrop--
	}
	return drops
}

func (a *RandomActor) SelectNoble(snapshot types.GameSnapshot) uint8 {
	currentPlayer := snapshot.Players[snapshot.CurrentPlayer]
	possibleNobles := make([]uint8, 0, len(snapshot.Nobles))
	for i := 0; i < len(snapshot.Nobles); i++ {
		noble := snapshot.Nobles[i]
		if currentPlayer.DevelopmentCards.Bonus.GreaterThanOrEqual(&noble.Requires) {
			possibleNobles = append(possibleNobles, uint8(i))
		}
	}
	if len(possibleNobles) == 0 {
		panic("no nobles to select")
	}
	return possibleNobles[a.rng.Intn(len(possibleNobles))]
}
