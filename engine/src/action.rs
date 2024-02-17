use crate::game::GameContext;
use splendor_core::{
    BuyCardAction, Color, DropTokensAction, PlayerAction, ReserveCardAction, ReservedCard,
    SelectNoblesAction, TakeTokenAction,
};

macro_rules! require {
    ($cond:expr) => {
        if !$cond {
            return false;
        } else {
            true
        }
    };
}

pub trait ActionExt {
    fn is_valid(&self, ctx: &GameContext) -> bool;
    fn apply(&self, ctx: &mut GameContext);
}

impl ActionExt for DropTokensAction {
    fn is_valid(&self, ctx: &GameContext) -> bool {
        // Check if the player has enough tokens to drop.
        require!(ctx.players[ctx.current_player].tokens.total() > 10);
        require!(self.0 <= ctx.players[ctx.current_player].tokens)
    }

    fn apply(&self, ctx: &mut GameContext) {
        ctx.tokens -= self.0;
    }
}

impl ActionExt for SelectNoblesAction {
    fn is_valid(&self, ctx: &GameContext) -> bool {
        // Check if nobles are available.
        require!(self.0 <= ctx.nobles.len());
        // Check if the player has met the noble requirements.
        let noble = &ctx.nobles.get(self.0);
        let player = &ctx.players[ctx.current_player];
        require!(noble.requires <= player.development_cards.bonus)
    }

    fn apply(&self, ctx: &mut GameContext) {
        let noble = ctx.nobles.remove(self.0);
        let player = &mut ctx.players[ctx.current_player];
        player.nobles.push(noble);
    }
}

impl ActionExt for PlayerAction {
    fn is_valid(&self, ctx: &GameContext) -> bool {
        match self {
            PlayerAction::TakeTokens(action) => action.is_valid(ctx),
            PlayerAction::BuyCard(action) => action.is_valid(ctx),
            PlayerAction::ReserveCard(action) => action.is_valid(ctx),
            PlayerAction::NoOp => true,
        }
    }

    fn apply(&self, ctx: &mut GameContext) {
        match self {
            PlayerAction::TakeTokens(action) => action.apply(ctx),
            PlayerAction::BuyCard(action) => action.apply(ctx),
            PlayerAction::ReserveCard(action) => action.apply(ctx),
            PlayerAction::NoOp => {}
        }
    }
}

impl ActionExt for TakeTokenAction {
    fn is_valid(&self, ctx: &GameContext) -> bool {
        require!(self
            .tokens()
            .iter()
            .zip(ctx.tokens.iter())
            .all(|(cnt, available)| cnt <= available));
        require!(match self {
            TakeTokenAction::ThreeDifferent(tokens) => {
                require!(self.tokens().iter().all(|cnt| cnt <= 1));
                require!(tokens.iter().filter(|cnt| *cnt > 0).count() <= 3)
            }
            TakeTokenAction::TwoSame(tokens) => {
                require!(self.tokens().iter().all(|cnt| cnt <= 2));
                require!(tokens.iter().filter(|cnt| *cnt > 0).count() == 1)
            }
        })
    }

    fn apply(&self, ctx: &mut GameContext) {
        let tokens = self.tokens();
        ctx.tokens -= tokens;
        ctx.players[ctx.current_player].tokens += tokens;
    }
}

impl ActionExt for BuyCardAction {
    fn is_valid(&self, ctx: &GameContext) -> bool {
        // Check if the card is available.
        let card = ctx.card_pool.peek(self.tier, self.idx);
        require!(card.is_some());
        let card = card.unwrap();
        // Check if the player has enough tokens.
        let player = &ctx.players[ctx.current_player];
        require!(player.tokens >= self.uses);
        // Check use of tokens matches the card.
        let available = player.development_cards.bonus + self.uses;
        let diff = available
            .iter()
            .zip(card.requires.iter())
            .map(|(a, b)| a as i8 - b as i8)
            .filter(|&x| x < 0)
            .map(|x| x.unsigned_abs())
            .sum::<u8>();
        require!(diff == self.uses.get(Color::Yellow))
    }

    fn apply(&self, ctx: &mut GameContext) {
        let card = ctx.card_pool.take(self.tier, self.idx);
        let player = &mut ctx.players[ctx.current_player];
        player.development_cards.add(card);
        player.tokens -= self.uses;
    }
}

impl ActionExt for ReserveCardAction {
    fn is_valid(&self, ctx: &GameContext) -> bool {
        // Check if the card is available.
        match self.idx.into_option() {
            None => require!(ctx.card_pool.remaining()[self.tier as usize] > 0),
            Some(idx) => require!(ctx.card_pool.revealed()[self.tier as usize] > idx),
        };
        // Check if the player has less than 3 reserved cards.
        let player = &ctx.players[ctx.current_player];
        require!(player.reserved_cards.len() < 3)
    }

    fn apply(&self, ctx: &mut GameContext) {
        let card = match self.idx.into_option() {
            None => {
                let card = ctx.card_pool.take_from_pool(self.tier);
                ReservedCard::new(card, true)
            }
            Some(idx) => ctx.card_pool.take(self.tier, idx).into(),
        };
        let player = &mut ctx.players[ctx.current_player];
        player.reserved_cards.push(card);
    }
}
