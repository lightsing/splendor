import json

from abc import ABC, abstractmethod
from os import environ
from typing import Optional

from websockets.sync.client import connect, ClientConnection
from websockets.exceptions import ConnectionClosedOK

from .types.snapshot import GameSnapshot
from .types.actions import PlayerAction, DropTokensAction, SelectNoblesAction


class PlayerActor(ABC):
    @abstractmethod
    def get_action(self, snapshot: GameSnapshot) -> PlayerAction: ...

    @abstractmethod
    def drop_tokens(self, snapshot: GameSnapshot) -> DropTokensAction: ...

    @abstractmethod
    def select_noble(self, snapshot: GameSnapshot) -> SelectNoblesAction: ...


class WebsocketPlayerActor:

    ws_client: ClientConnection
    actor: PlayerActor

    def __init__(
        self,
        actor: PlayerActor,
        rpc: Optional[str] = None,
        secret: Optional[str] = None,
    ):
        if rpc is None:
            rpc = environ["RPC_URL"]
        if secret is None:
            secret = open(environ["CLIENT_SECRET"]).read()

        ws_client = connect(rpc)
        ws_client.send(secret)

        self.ws_client = ws_client
        self.actor = actor

    def run(self):
        try:
            while True:
                req = self.ws_client.recv()
                req = json.loads(req)
                req_type = req["type"]
                snapshot = GameSnapshot(**req["snapshot"])
                if req_type == "get_action":
                    action = self.actor.get_action(snapshot)
                elif req_type == "drop_tokens":
                    action = self.actor.drop_tokens(snapshot)
                elif req_type == "select_noble":
                    action = self.actor.select_noble(snapshot)
                else:
                    raise ValueError(f"Invalid request type: {req_type}")
                self.ws_client.send(json.dumps(action.to_json()))
        except (ConnectionClosedOK, KeyboardInterrupt):
            pass
        finally:
            self.ws_client.close()
