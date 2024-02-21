use crate::error::InvalidActionError;
use crate::game::GameContext;
use splendor_core::{
    BuyCardAction, BuyCardSource, Color, DropTokensAction, PlayerAction, ReserveCardAction,
    ReservedCard, SelectNoblesAction, TakeTokenAction,
};
use std::fmt::Debug;

pub trait ActionExt: Debug {
    fn is_valid(&self, ctx: &GameContext) -> Result<(), InvalidActionError>;
    fn apply(&self, ctx: &mut GameContext);

    fn require(
        &self,
        ctx: &GameContext,
        cond: bool,
        reason: &'static str,
    ) -> Result<(), InvalidActionError> {
        if !cond {
            error!(
                "Player#{} took an invalid action: {self:?}, reason {reason}",
                ctx.current_player
            );
            return Err(InvalidActionError {
                player: ctx.current_player,
                reason,
            });
        }
        Ok(())
    }
}

impl ActionExt for DropTokensAction {
    fn is_valid(&self, ctx: &GameContext) -> Result<(), InvalidActionError> {
        // Check if the player has enough tokens to drop.
        self.require(
            ctx,
            ctx.players[ctx.current_player].tokens.total() > 10,
            "no need to drop",
        )?;
        self.require(
            ctx,
            self.0.le(&ctx.players[ctx.current_player].tokens),
            "not enough tokens to drop",
        )
    }

    fn apply(&self, ctx: &mut GameContext) {
        ctx.players[ctx.current_player].tokens -= self.0;
        ctx.tokens += self.0;
    }
}

impl ActionExt for SelectNoblesAction {
    fn is_valid(&self, ctx: &GameContext) -> Result<(), InvalidActionError> {
        // Check if nobles are available.
        self.require(ctx, self.0 <= ctx.nobles.len(), "noble index out of range")?;
        // Check if the player has met the noble requirements.
        let noble = &ctx.nobles.get(self.0);
        let player = &ctx.players[ctx.current_player];
        self.require(
            ctx,
            noble.requires.le(&player.development_cards.bonus),
            "noble requirements not met",
        )
    }

    fn apply(&self, ctx: &mut GameContext) {
        let noble = ctx.nobles.remove(self.0);
        let player = &mut ctx.players[ctx.current_player];
        player.nobles.push(noble);
    }
}

impl ActionExt for PlayerAction {
    fn is_valid(&self, ctx: &GameContext) -> Result<(), InvalidActionError> {
        match self {
            PlayerAction::TakeTokens(action) => action.is_valid(ctx),
            PlayerAction::BuyCard(action) => action.is_valid(ctx),
            PlayerAction::ReserveCard(action) => action.is_valid(ctx),
            PlayerAction::Nop => Ok(()),
        }
    }

    fn apply(&self, ctx: &mut GameContext) {
        match self {
            PlayerAction::TakeTokens(action) => action.apply(ctx),
            PlayerAction::BuyCard(action) => action.apply(ctx),
            PlayerAction::ReserveCard(action) => action.apply(ctx),
            PlayerAction::Nop => {}
        }
    }
}

impl ActionExt for TakeTokenAction {
    fn is_valid(&self, ctx: &GameContext) -> Result<(), InvalidActionError> {
        self.require(
            ctx,
            self.tokens().get(Color::Yellow) == 0,
            "cannot take yellow tokens",
        )?;
        self.require(
            ctx,
            self.tokens().le(&ctx.tokens),
            "not enough tokens available",
        )?;
        match self {
            TakeTokenAction::ThreeDifferent(tokens) => {
                self.require(
                    ctx,
                    self.tokens().iter().all(|cnt| cnt <= 1),
                    "cannot take more than one token",
                )?;
                self.require(
                    ctx,
                    tokens.iter().filter(|cnt| *cnt > 0).count() <= 3,
                    "cannot take more than 3 tokens",
                )
            }
            TakeTokenAction::TwoSame(tokens) => {
                self.require(
                    ctx,
                    self.tokens().iter().all(|cnt| cnt <= 2),
                    "cannot take more than two tokens",
                )?;
                self.require(
                    ctx,
                    tokens.iter().filter(|cnt| *cnt > 0).count() == 1,
                    "must take tokens of the same color",
                )
            }
        }
    }

    fn apply(&self, ctx: &mut GameContext) {
        let tokens = self.tokens();
        ctx.tokens -= tokens;
        ctx.players[ctx.current_player].tokens += tokens;
        trace!(
            "Player#{} now has tokens: {:?}, remaining tokens: {:?}",
            ctx.current_player,
            ctx.players[ctx.current_player].tokens,
            ctx.tokens
        );
    }
}

impl ActionExt for BuyCardAction {
    fn is_valid(&self, ctx: &GameContext) -> Result<(), InvalidActionError> {
        let player = &ctx.players[ctx.current_player];
        // Check if the card is available.
        let card = match self.source {
            BuyCardSource::Revealed { tier, idx } => ctx.card_pool.peek(tier, idx),
            BuyCardSource::Reserved(idx) => player.reserved_cards.get(idx).map(|c| &c.card),
        };
        self.require(ctx, card.is_some(), "card index out of range")?;
        let card = card.unwrap();
        // Check if the player has enough tokens.
        self.require(
            ctx,
            player.tokens.ge(&self.uses),
            "not enough tokens to use",
        )?;
        // Check use of tokens matches the card.
        let available = player.development_cards.bonus + self.uses;
        let diff = available
            .iter()
            .zip(card.requires.iter())
            .map(|(a, b)| a as i8 - b as i8)
            .filter(|&x| x < 0)
            .map(|x| x.unsigned_abs())
            .sum::<u8>();
        self.require(
            ctx,
            diff == self.uses.get(Color::Yellow),
            "invalid token use",
        )
    }

    fn apply(&self, ctx: &mut GameContext) {
        let player = &mut ctx.players[ctx.current_player];
        let card = match self.source {
            BuyCardSource::Revealed { tier, idx } => ctx.card_pool.take(tier, idx),
            BuyCardSource::Reserved(idx) => player.reserved_cards.remove(idx).card,
        };
        player.development_cards.add(card);
        player.tokens -= self.uses;
        ctx.tokens += self.uses;
    }
}

impl ActionExt for ReserveCardAction {
    fn is_valid(&self, ctx: &GameContext) -> Result<(), InvalidActionError> {
        // Check if the card is available.
        match self.idx {
            None => self.require(
                ctx,
                ctx.card_pool.remaining()[self.tier as usize] > 0,
                "no cards available in pool",
            ),
            Some(idx) => self.require(
                ctx,
                ctx.card_pool.revealed()[self.tier as usize] > idx,
                "card index out of range",
            ),
        }?;
        // Check if the player has less than 3 reserved cards.
        let player = &ctx.players[ctx.current_player];
        self.require(
            ctx,
            player.reserved_cards.len() < 3,
            "cannot reserve more than 3 cards",
        )
    }

    fn apply(&self, ctx: &mut GameContext) {
        let card = match self.idx {
            None => {
                let card = ctx.card_pool.take_from_pool(self.tier);
                ReservedCard::new(card, true)
            }
            Some(idx) => ctx.card_pool.take(self.tier, idx).into(),
        };
        let player = &mut ctx.players[ctx.current_player];
        player.reserved_cards.push(card);
        if ctx.tokens.get(Color::Yellow) > 0 {
            ctx.tokens.sub(Color::Yellow, 1);
            player.tokens.add(Color::Yellow, 1);
        }
    }
}
