package types

import (
	"encoding/json"
	"testing"

	"github.com/stretchr/testify/assert"
)

const snapshotString string = "{\"last_round\":false,\"current_round\":0,\"current_player\":0,\"tokens\":[7,7,7,7,7,5],\"card_pool\":{\"remaining\":[36,26,16],\"revealed\":[[{\"tier\":0,\"bonus\":\"white\",\"points\":0,\"requires\":[1,1,0,0,3,0]},{\"tier\":0,\"bonus\":\"red\",\"points\":0,\"requires\":[1,1,1,0,1,0]},{\"tier\":0,\"bonus\":\"green\",\"points\":0,\"requires\":[2,1,0,2,0,0]},{\"tier\":0,\"bonus\":\"green\",\"points\":0,\"requires\":[0,1,0,0,2,0]}],[{\"tier\":1,\"bonus\":\"black\",\"points\":2,\"requires\":[0,0,0,0,5,0]},{\"tier\":1,\"bonus\":\"green\",\"points\":1,\"requires\":[0,0,2,3,3,0]},{\"tier\":1,\"bonus\":\"white\",\"points\":2,\"requires\":[2,0,1,4,0,0]},{\"tier\":1,\"bonus\":\"red\",\"points\":1,\"requires\":[3,0,0,2,2,0]}],[{\"tier\":2,\"bonus\":\"white\",\"points\":3,\"requires\":[3,3,3,5,0,0]},{\"tier\":2,\"bonus\":\"red\",\"points\":5,\"requires\":[0,0,7,3,0,0]},{\"tier\":2,\"bonus\":\"green\",\"points\":3,\"requires\":[3,3,0,3,5,0]},{\"tier\":2,\"bonus\":\"green\",\"points\":4,\"requires\":[0,6,3,0,3,0]}]]},\"nobles\":[{\"requires\":[3,0,3,3,0,0]},{\"requires\":[0,3,3,0,3,0]},{\"requires\":[0,4,0,0,4,0]},{\"requires\":[0,4,4,0,0,0]},{\"requires\":[0,3,3,3,0,0]}],\"players\":[{\"idx\":0,\"points\":0,\"tokens\":[0,0,0,0,0,0],\"development_cards\":{\"points\":0,\"bonus\":[0,0,0,0,0,0],\"inner\":[[],[],[],[],[]]},\"reserved_cards\":[],\"nobles\":[]},{\"idx\":1,\"points\":0,\"tokens\":[0,0,0,0,0,0],\"development_cards\":{\"points\":0,\"bonus\":[0,0,0,0,0,0],\"inner\":[[],[],[],[],[]]},\"reserved_cards\":[],\"nobles\":[]},{\"idx\":2,\"points\":0,\"tokens\":[0,0,0,0,0,0],\"development_cards\":{\"points\":0,\"bonus\":[0,0,0,0,0,0],\"inner\":[[],[],[],[],[]]},\"reserved_cards\":[],\"nobles\":[]},{\"idx\":3,\"points\":0,\"tokens\":[0,0,0,0,0,0],\"development_cards\":{\"points\":0,\"bonus\":[0,0,0,0,0,0],\"inner\":[[],[],[],[],[]]},\"reserved_cards\":[],\"nobles\":[]}]}"

func TestSnapshot(t *testing.T) {
	var snapshot GameSnapshot
	err := json.Unmarshal([]byte(snapshotString), &snapshot)
	if err != nil {
		t.Error(err)
	}
	assert.Equal(t, snapshot.LastRound, false)
	assert.Equal(t, snapshot.CurrentRound, uint8(0))
	assert.Equal(t, snapshot.CurrentPlayer, uint8(0))
	assert.Equal(t, snapshot.Tokens, ColorVec{7, 7, 7, 7, 7, 5})
	assert.Equal(t, snapshot.CardPool.Remaining, [3]uint8{36, 26, 16})
	assert.Equal(t, snapshot.CardPool.Revealed[0][0].Tier, Tier(0))
	assert.Equal(t, snapshot.CardPool.Revealed[0][0].Color, BLACK)
	assert.Equal(t, snapshot.CardPool.Revealed[0][0].Points, uint8(0))
	assert.Equal(t, snapshot.CardPool.Revealed[0][0].Requires, ColorVec{1, 1, 0, 0, 3, 0})
	assert.Equal(t, snapshot.Nobles[0].Requires, ColorVec{3, 0, 3, 3, 0, 0})
	assert.Equal(t, snapshot.Nobles[1].Requires, ColorVec{0, 3, 3, 0, 3, 0})
	assert.Equal(t, snapshot.Nobles[2].Requires, ColorVec{0, 4, 0, 0, 4, 0})
}
