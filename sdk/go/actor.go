package actor

import (
	"errors"
	"os"

	"github.com/gorilla/websocket"
	"github.com/lightsing/splendor/sdk/go/types"
)

type PlayerActor interface {
	GetAction(types.GameSnapshot) types.PlayerAction
	DropTokens(types.GameSnapshot) types.ColorVec
	SelectNoble(types.GameSnapshot) uint8
}

type WebsocketPlayerActor struct {
	client *websocket.Conn
	actor  PlayerActor
}

type actorReq struct {
	Type     string             `json:"type"`
	Snapshot types.GameSnapshot `json:"snapshot"`
}

func NewWebsocketPlayerActor(
	actor PlayerActor,
	rpc string,
	secret string,
) (*WebsocketPlayerActor, error) {
	if len(rpc) == 0 {
		var ok bool
		rpc, ok = os.LookupEnv("RPC_URL")
		if !ok {
			return nil, errors.New("either rpc arg or RPC_URL env not set")
		}
	}
	if len(secret) == 0 {
		path, ok := os.LookupEnv("CLIENT_SECRET")
		if !ok {
			return nil, errors.New("either secret arg or CLIENT_SECRET env not set")
		}
		file, err := os.Open(path)
		if err != nil {
			return nil, err
		}
		buf := make([]byte, 128)
		_, err = file.Read(buf)
		if err != nil {
			return nil, err
		}
		secret = string(buf)
	}

	conn, _, err := websocket.DefaultDialer.Dial(rpc, nil)
	if err != nil {
		return nil, err
	}

	err = conn.WriteMessage(websocket.TextMessage, []byte(secret))
	if err != nil {
		return nil, err
	}

	return &WebsocketPlayerActor{
		client: conn,
		actor:  actor,
	}, nil
}

func (w *WebsocketPlayerActor) Close() {
	w.client.Close()
}

func (w *WebsocketPlayerActor) Run() error {
	for {
		var (
			req    actorReq
			action interface{}
		)
		err := w.client.ReadJSON(&req)
		if err != nil {
			return err
		}
		switch req.Type {
		case "get_action":
			action = w.actor.GetAction(req.Snapshot)
		case "drop_tokens":
			action = w.actor.DropTokens(req.Snapshot)
		case "select_noble":
			action = w.actor.SelectNoble(req.Snapshot)
		}
		err = w.client.WriteJSON(action)
		if err != nil {
			return err
		}
	}
}
