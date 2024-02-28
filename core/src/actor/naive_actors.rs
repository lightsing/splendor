use super::*;
use crate::{
    BuyCardAction, BuyCardSource, Color, ColorVec, ReserveCardAction, TakeTokenAction, Tier,
};
use rand::seq::SliceRandom;
use rand::RngCore;
use smallvec::SmallVec;
use std::fmt;

/// A random player actor for testing.
#[derive(Default, Copy, Clone)]
pub struct RandomActor<R> {
    rng: R,
}

impl<R: RngCore + Send + Sync> RandomActor<R> {
    /// Create a new random actor with a given random number generator.
    pub fn new(rng: R) -> Self {
        RandomActor { rng }
    }
}

impl<R> Debug for RandomActor<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RandomActor").finish()
    }
}

#[async_trait::async_trait]
impl<R: RngCore + Send + Sync> PlayerActor for RandomActor<R> {
    async fn get_action(&mut self, snapshot: GameSnapshot) -> Result<PlayerAction, ActorError> {
        let current_player = &snapshot.players[snapshot.current_player];
        let mut possible_actions = SmallVec::<_, 4>::new();
        {
            // take 3 different tokens if possible
            let mut tokens = snapshot
                .tokens
                .iter()
                .take(5)
                .enumerate()
                .filter(|(_, n)| *n > 0)
                .collect::<SmallVec<_, 5>>();
            tokens.shuffle(&mut self.rng);
            if tokens.len() >= 1 {
                let mut colors = ColorVec::empty();
                for (i, _) in tokens.iter().take(3) {
                    colors.add(Color::try_from(*i).unwrap(), 1);
                }
                possible_actions.push(PlayerAction::TakeTokens(TakeTokenAction::ThreeDifferent(
                    colors,
                )));
            }
        }
        {
            // take 2 tokens of the same color if possible
            let tokens = snapshot
                .tokens
                .iter()
                .take(5)
                .enumerate()
                .filter(|(_, n)| *n > 3)
                .collect::<Vec<_>>();
            if let Some((i, _)) = tokens.first() {
                let mut colors = ColorVec::empty();
                colors.set(Color::try_from(*i).unwrap(), 2);
                possible_actions.push(PlayerAction::TakeTokens(TakeTokenAction::TwoSame(colors)));
            }
        }
        {
            // reserve a card if possible
            if current_player.reserved_cards.len() < 3 {
                let possible_card = snapshot
                    .card_pool
                    .revealed
                    .iter()
                    .flat_map(|cards| {
                        cards.iter().enumerate().map(|(idx, c)| ReserveCardAction {
                            tier: c.tier,
                            idx: Some(idx),
                        })
                    })
                    .chain(
                        snapshot
                            .card_pool
                            .remaining
                            .iter()
                            .enumerate()
                            .filter(|(_, n)| **n > 0)
                            .map(|(tier, _)| ReserveCardAction {
                                tier: Tier::try_from(tier).unwrap(),
                                idx: None,
                            }),
                    )
                    .collect::<SmallVec<_, { 4 * 4 }>>();
                if let Some(action) = possible_card.choose(&mut self.rng) {
                    possible_actions.push(PlayerAction::ReserveCard(*action));
                }
            }
        }
        {
            // buy a card if possible
            let possible_cards = snapshot
                .card_pool
                .revealed
                .iter()
                .flat_map(|cards| {
                    cards
                        .iter()
                        .enumerate()
                        .map(|(idx, c)| (c, BuyCardSource::Revealed { tier: c.tier, idx }))
                })
                .chain(
                    current_player
                        .reserved_cards
                        .iter()
                        .enumerate()
                        .filter_map(|(idx, c)| Some((c.unwrap(), BuyCardSource::Reserved(idx)))),
                )
                .filter_map(|(card, source)| {
                    let effective_cost = card
                        .requires
                        .saturating_sub(&current_player.development_cards.bonus);
                    let diff = effective_cost.saturating_sub(&current_player.tokens);
                    if diff.total() > current_player.tokens.get(Color::Yellow) {
                        return None;
                    }
                    let mut uses = effective_cost - diff;
                    uses.set(Color::Yellow, diff.total());
                    Some(BuyCardAction { source, uses })
                })
                .collect::<SmallVec<_, { 4 * 3 }>>();
            if let Some(action) = possible_cards.choose(&mut self.rng) {
                possible_actions.push(PlayerAction::BuyCard(*action));
            }
        }
        if let Some(action) = possible_actions.choose(&mut self.rng) {
            return Ok(*action);
        }

        // panic!("no possible action: {:#?}", snapshot);
        return Ok(PlayerAction::Nop);
    }

    async fn drop_tokens(
        &mut self,
        snapshot: GameSnapshot,
    ) -> Result<DropTokensAction, ActorError> {
        let current_player = &snapshot.players[snapshot.current_player];
        let mut tokens = current_player.tokens;
        let mut to_drop = tokens.total() - 10;
        let mut drops = ColorVec::empty();
        while to_drop > 0 {
            let possible_drops = tokens
                .iter()
                .enumerate()
                .filter(|(_, n)| *n > 0)
                .collect::<SmallVec<_, 6>>();
            let (color, _) = possible_drops.choose(&mut self.rng).unwrap();
            drops.add(Color::try_from(*color).unwrap(), 1);
            tokens.sub(Color::try_from(*color).unwrap(), 1);
            to_drop -= 1;
        }
        Ok(DropTokensAction(drops))
    }

    async fn select_noble(
        &mut self,
        snapshot: GameSnapshot,
    ) -> Result<SelectNoblesAction, ActorError> {
        let current_player = &snapshot.players[snapshot.current_player];
        let possible_nobles = snapshot
            .nobles
            .iter()
            .enumerate()
            .filter(|(_, n)| current_player.development_cards.bonus.le(&n.requires))
            .collect::<SmallVec<_, 5>>();
        let (idx, _) = possible_nobles.choose(&mut self.rng).unwrap();
        Ok(SelectNoblesAction(*idx))
    }
}
