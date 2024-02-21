package main

import (
	"fmt"
	"os"

	"github.com/lightsing/splendor/sdk/go/pkg/actor"
	"github.com/lightsing/splendor/sdk/go/pkg/actor/naive_actors"
)

func main() {
	naiveActor, err := actor.NewWebsocketPlayerActor(&naive_actors.RandomActor{}, "", "")
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error creating actor: %v\n", err)
		return
	}
	defer func() {
		if r := recover(); r != nil {
			fmt.Fprintf(os.Stderr, "Gracefully exiting: %v\n", r)
		}
		if naiveActor != nil {
			naiveActor.Close()
		}
	}()

	naiveActor.Run()
}
