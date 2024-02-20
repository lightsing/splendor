from random import Random

from splendor_actor import PlayerActor, WebsocketPlayerActor
from splendor_actor.types import ColorVec, Tier
from splendor_actor.types.snapshot import GameSnapshot
from splendor_actor.types.actions import PlayerAction, TakeTokenAction, ReserveCardAction, BuyCardAction, SelectNoblesAction, NopAction, DropTokensAction

class RandomActor(PlayerActor):
    def __init__(self, seed):
        self.rng = Random(seed)

    def get_action(self, snapshot: GameSnapshot) -> PlayerAction:
        current_player = snapshot.players[snapshot.current_player]
        possible_actions = []

        # take 3 different tokens if possible
        tokens = [color for color, count in snapshot.tokens if count > 0][:-1]
        self.rng.shuffle(tokens)
        if tokens:
            possible_actions.append(TakeTokenAction.three_different(tokens[:3]))
        
        # take 2 same tokens if possible
        tokens = [color for color, count in snapshot.tokens if count > 3][:-1]
        if tokens:
            possible_actions.append(TakeTokenAction.two_same(self.rng.choice(tokens)))
        
        # reserve a card if possible
        if len(current_player.reserved_cards) < 3:
            possible_cards = []
            for cards in snapshot.card_pool.revealed:
                for idx, card in enumerate(cards):
                    possible_cards.append(ReserveCardAction.from_revealed(card.tier, idx))
            for idx, cnt in enumerate(snapshot.card_pool.remaining):
                if cnt > 0:
                    possible_cards.append(ReserveCardAction.from_pool(Tier(idx)))
            if possible_cards:
                possible_actions.append(self.rng.choice(possible_cards))

        # buy a card if possible
        possible_cards = []
        for cards in snapshot.card_pool.revealed:
            for idx, card in enumerate(cards):
                if uses := current_player.can_buy(card):
                    possible_cards.append(BuyCardAction.from_revealed(card.tier, idx, uses))
        for idx, card in enumerate(current_player.reserved_cards):
            if uses := current_player.can_buy(card.view):
                possible_cards.append(BuyCardAction.from_reserved(idx, uses))
        if possible_cards:
            possible_actions.append(self.rng.choice(possible_cards))
        
        if possible_actions:
            return self.rng.choice(possible_actions)
        
        return NopAction()
    
    def drop_tokens(self, snapshot: GameSnapshot) -> DropTokensAction:
        current_player = snapshot.players[snapshot.current_player]
        tokens = current_player.tokens
        to_drop = current_player.tokens.total() - 10
        drops = ColorVec.empty()
        while to_drop > 0:
            possible_drops = [color for color, count in current_player.tokens if count > 0]
            color = self.rng.choice(possible_drops)
            drops[color] += 1
            tokens[color] -= 1
            to_drop -= 1
        return DropTokensAction(drops)
    
    def select_noble(self, snapshot: GameSnapshot) -> SelectNoblesAction:
        current_player = snapshot.players[snapshot.current_player]
        possible_nobles = []
        for idx, noble in enumerate(snapshot.nobles):
            if current_player.development_cards.bonus >= noble.requires:
                possible_nobles.append(idx)
        idx = self.rng.choice(possible_nobles)
        return SelectNoblesAction(idx)


if __name__ == "__main__":
    WebsocketPlayerActor(RandomActor(None)).run()