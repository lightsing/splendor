package main

import (
	"github.com/lightsing/splendor/sdk/go/pkg/actor"
	"github.com/lightsing/splendor/sdk/go/pkg/actor/naive_actors"
)

func main() {
	naiveActor, err := actor.NewWebsocketPlayerActor(naive_actors.NewRandomActor(), "", "")
	if err != nil {
		panic(err)
	}

	naiveActor.Run()
}
