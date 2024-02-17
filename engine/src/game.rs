use crate::action::ActionExt;
use crate::cards::CardPool;
use crate::error::StepError;
use crate::nobles::Nobles;
use crate::player::PlayerContext;
use rand::RngCore;
use smallvec::SmallVec;
use splendor_core::{
    ActionRecord, CardPoolSnapshot, ColorVec, GameSnapshot, Noble, PlayerActor, PlayerSnapshot,
    Record, SelectNoblesAction, MAX_PLAYERS,
};
use std::array;

/// A struct to represent the game context.
pub struct GameContext {
    pub(crate) n_players: usize,
    pub(crate) last_round: bool,
    pub(crate) game_end: bool,
    pub(crate) current_round: usize,
    pub(crate) current_player: usize,

    pub(crate) tokens: ColorVec,
    pub(crate) card_pool: CardPool,
    pub(crate) nobles: Nobles,

    pub(crate) players: SmallVec<PlayerContext, MAX_PLAYERS>,
    pub(crate) player_actors: SmallVec<Box<dyn PlayerActor>, MAX_PLAYERS>,

    pub(crate) records: Vec<Record>,
}

impl GameContext {
    /// Create a new game context with thread_rng.
    pub fn random(player_actors: SmallVec<Box<dyn PlayerActor>, MAX_PLAYERS>) -> Self {
        GameContext::with_rng(&mut rand::thread_rng(), player_actors)
    }

    /// Create a new game context with a given random number generator.
    ///
    /// This can be used to create a game context with a specific seed for reproducibility.
    pub fn with_rng<R: RngCore>(
        rng: &mut R,
        player_actors: SmallVec<Box<dyn PlayerActor>, MAX_PLAYERS>,
    ) -> Self {
        let n_players = player_actors.len();
        let tokens = match n_players {
            2 => ColorVec::new(4, 4, 4, 4, 4, 5),
            3 => ColorVec::new(5, 5, 5, 5, 5, 5),
            4 => ColorVec::new(7, 7, 7, 7, 7, 5),
            _ => panic!("Invalid number of players"),
        };
        let card_pool = CardPool::with_rng(rng);
        let nobles = Nobles::with_rng(rng, n_players + 1);
        let players = SmallVec::from_buf(array::from_fn(PlayerContext::new));
        GameContext {
            n_players,
            current_round: 0,
            last_round: false,
            game_end: false,
            current_player: 0,
            tokens,
            card_pool,
            nobles,
            players,
            player_actors,
            records: Vec::new(),
        }
    }

    /// Step the game by one turn.
    pub fn step(&mut self) -> Result<Option<SmallVec<usize, MAX_PLAYERS>>, StepError> {
        let action = self.player_actors[self.current_player].get_action(GameSnapshot::from(&*self));
        action.is_valid(self)?;
        action.apply(self);
        self.records.push(Record::PlayerAction(ActionRecord::new(
            self.current_player,
            action,
        )));

        if self.players[self.current_player].tokens.total() > 10 {
            let drop_tokens =
                self.player_actors[self.current_player].drop_tokens(GameSnapshot::from(&*self));
            drop_tokens.is_valid(self)?;
            drop_tokens.apply(self);
            self.records.push(Record::DropTokens(ActionRecord::new(
                self.current_player,
                drop_tokens,
            )));
        }

        let noble_visits: SmallVec<(usize, Noble), { MAX_PLAYERS + 1 }> = self
            .nobles
            .iter()
            .copied()
            .enumerate()
            .filter(|(_, n)| {
                n.requires <= self.players[self.current_player].development_cards.bonus
            })
            .collect();
        if !noble_visits.is_empty() {
            let (action, noble) = if noble_visits.len() > 1 {
                let select_noble = self.player_actors[self.current_player]
                    .select_noble(GameSnapshot::from(&*self));
                select_noble.is_valid(self)?;
                (select_noble, self.nobles.get(select_noble.0))
            } else {
                (SelectNoblesAction(noble_visits[0].0), noble_visits[0].1)
            };
            debug_assert!(action.is_valid(self).is_ok());
            action.apply(self);
            self.records.push(Record::VisitNoble(ActionRecord::new(
                self.current_player,
                noble,
            )));
        }

        if self.players[self.current_player].points() >= 15 {
            self.last_round = true;
        }

        if self.last_round && self.current_player == self.n_players - 1 {
            self.game_end = true;
            return Ok(Some(self.get_winner()));
        }

        self.current_player = (self.current_player + 1) % self.n_players;
        if self.current_player == 0 {
            self.current_round += 1;
        }
        Ok(None)
    }

    fn get_winner(&self) -> SmallVec<usize, MAX_PLAYERS> {
        // player with the most points wins
        let player_points = self
            .players
            .iter()
            .map(|p| p.points())
            .collect::<SmallVec<u8, MAX_PLAYERS>>();
        let max_points = player_points.iter().max().copied().unwrap();
        let winner_candidates = player_points
            .iter()
            .enumerate()
            .filter(|(_, &p)| p == max_points)
            .map(|(i, _)| i)
            .collect::<SmallVec<usize, MAX_PLAYERS>>();
        if winner_candidates.len() > 1 {
            // player with the fewest development cards wins
            let development_cards = winner_candidates
                .iter()
                .map(|&i| (i, self.players[i].development_cards.iter().count()))
                .collect::<SmallVec<(usize, usize), MAX_PLAYERS>>();
            let min_development_cards = development_cards
                .iter()
                .map(|(_, cnt)| *cnt)
                .collect::<SmallVec<usize, MAX_PLAYERS>>()
                .into_iter()
                .min()
                .unwrap();
            development_cards
                .iter()
                .filter(|(_, cnt)| *cnt == min_development_cards)
                .map(|(i, _)| *i)
                .collect()
        } else {
            winner_candidates
        }
    }
}

impl GameContext {
    /// Get the number of players in the game.
    pub fn n_players(&self) -> usize {
        self.n_players
    }

    /// Get is the game in the last round.
    pub fn last_round(&self) -> bool {
        self.last_round
    }

    /// Get is the game ended.
    pub fn game_end(&self) -> bool {
        self.game_end
    }

    /// Get the current round.
    pub fn current_round(&self) -> usize {
        self.current_round
    }

    /// Get the current player.
    pub fn current_player(&self) -> usize {
        self.current_player
    }

    /// Get the tokens available in the game.
    pub fn tokens(&self) -> ColorVec {
        self.tokens
    }

    /// Create a snapshot of the game.
    pub fn snapshot(&self) -> GameSnapshot {
        GameSnapshot::from(self)
    }
}

impl From<&GameContext> for GameSnapshot {
    fn from(ctx: &GameContext) -> Self {
        Self {
            last_round: ctx.last_round,
            current_round: ctx.current_round,
            current_player: ctx.current_player,
            tokens: ctx.tokens,
            card_pool: (&ctx.card_pool).into(),
            nobles: ctx.nobles.0.clone(),
            players: ctx
                .players
                .iter()
                .map(|p| p.into())
                .collect::<SmallVec<_, MAX_PLAYERS>>(),
        }
    }
}

impl From<&CardPool> for CardPoolSnapshot {
    fn from(pool: &CardPool) -> Self {
        Self {
            remaining: pool.remaining(),
            revealed: pool.revealed.clone(),
        }
    }
}

impl From<&PlayerContext> for PlayerSnapshot {
    fn from(player: &PlayerContext) -> Self {
        Self {
            idx: player.idx,
            points: player.points(),
            tokens: player.tokens,
            development_cards: player.development_cards.clone(),
            reserved_cards: player.reserved_cards.iter().map(|c| (*c).into()).collect(),
            nobles: player.nobles.clone(),
        }
    }
}
