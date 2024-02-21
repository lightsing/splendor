package actor

import (
	"encoding/json"
	"errors"
	"os"

	"github.com/gorilla/websocket"
	"github.com/lightsing/splendor/sdk/go/pkg/types"
	log "github.com/sirupsen/logrus"
)

type PlayerActor interface {
	GetAction(*types.GameSnapshot) types.PlayerAction
	DropTokens(*types.GameSnapshot) types.ColorVec
	SelectNoble(*types.GameSnapshot) uint8
}

type WebsocketPlayerActor struct {
	client *websocket.Conn
	actor  PlayerActor
}

type actorReq struct {
	Type     string              `json:"type"`
	Snapshot *types.GameSnapshot `json:"snapshot"`
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
			log.Errorf("either rpc arg or RPC_URL env not set")
			return nil, errors.New("either rpc arg or RPC_URL env not set")
		}
	}
	log.Infof("WebSocket Server: %s", rpc)
	if len(secret) == 0 {
		path, ok := os.LookupEnv("CLIENT_SECRET")
		if !ok {
			log.Error("either secret arg or CLIENT_SECRET env not set")
			return nil, errors.New("either secret arg or CLIENT_SECRET env not set")
		}
		file, err := os.Open(path)
		if err != nil {
			log.Errorf("error opening secret file: %v", err)
			return nil, err
		}
		buf := make([]byte, 128)
		n, err := file.Read(buf)
		if err != nil {
			log.Errorf("error reading secret file: %v", err)
			return nil, err
		}
		secret = string(buf[:n])
		log.Infof("Secret: %s", secret)
	}

	conn, _, err := websocket.DefaultDialer.Dial(rpc, nil)
	if err != nil {
		log.Errorf("error dialing websocket server: %v", err)
		return nil, err
	}

	err = conn.WriteMessage(websocket.TextMessage, []byte(secret))
	if err != nil {
		log.Errorf("error sending secret to server: %v", err)
		return nil, err
	}

	return &WebsocketPlayerActor{
		client: conn,
		actor:  actor,
	}, nil
}

func (w *WebsocketPlayerActor) Close() {
	log.Infof("Shutting down websocket actor...")
	w.client.Close()
}

func (w *WebsocketPlayerActor) Run() (bool, error) {
	for {
		var (
			req    actorReq
			action interface{}
			err    error
		)

		err = w.client.ReadJSON(&req)
		if err != nil {
			if closeErr, ok := err.(*websocket.CloseError); ok && closeErr.Code == websocket.CloseNormalClosure {
				log.Infof("Game ended")
				return true, nil
			}
			log.Errorf("error reading request from server: %v", err)
			return false, err
		}
		log.Infof("Received request of type: %s", req.Type)
		switch req.Type {
		case "get_action":
			action = w.actor.GetAction(req.Snapshot)
			log.Infof("Took action: %#v", action)
		case "drop_tokens":
			action = w.actor.DropTokens(req.Snapshot)
			log.Infof("Dropped tokens: %v", action)
		case "select_noble":
			action = w.actor.SelectNoble(req.Snapshot)
			log.Infof("Selected noble: %d", action)
		}
		actionBytes, err := json.Marshal(action)
		if err != nil {
			log.Panicf("error marshalling action: %v", err)
		}
		err = w.client.WriteMessage(websocket.TextMessage, actionBytes)
		if err != nil {
			log.Errorf("error writing action to server: %v", err)
			return false, err
		}
	}
}
