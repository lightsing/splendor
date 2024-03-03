use crate::action::ActionExt;
use crate::cards::CardPool;
use crate::error::StepError;
use crate::nobles::Nobles;
use crate::player::PlayerContext;
use rand::RngCore;
use smallvec::{smallvec, SmallVec};
use splendor_core::{
    ActionRecord, CardPoolSnapshot, Color, ColorVec, GameSnapshot, Noble, PlayerActor, Record,
    SelectNoblesAction, Tier, MAX_PLAYERS,
};
use std::array;

/// A struct to represent the game context.
#[derive(Debug)]
pub struct GameContext {
    pub(crate) n_players: usize,
    pub(crate) last_round: bool,
    pub(crate) game_end: bool,
    pub(crate) nop_count: usize,
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
            nop_count: 0,
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
    pub async fn step(&mut self) -> Result<Option<SmallVec<usize, MAX_PLAYERS>>, StepError> {
        let snapshot = self.snapshot();
        let action = self.player_actors[self.current_player]
            .get_action(snapshot)
            .await?;
        action.is_valid(self)?;
        info!("Player#{} action: {:?}", self.current_player, action);
        action.apply(self);
        self.records.push(Record::PlayerAction(ActionRecord::new(
            self.current_player,
            action,
        )));
        if action.is_nop() {
            self.nop_count += 1;
        }

        if self.players[self.current_player].tokens.total() > 10 {
            info!("Player#{} needs to drop tokens", self.current_player);
            let snapshot = self.snapshot();
            let drop_tokens = self.player_actors[self.current_player]
                .drop_tokens(snapshot)
                .await?;
            drop_tokens.is_valid(self)?;
            info!(
                "Player#{} dropped tokens: {:?}",
                self.current_player, drop_tokens
            );
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
                n.requires
                    .le(&self.players[self.current_player].development_cards.bonus)
            })
            .collect();
        if !noble_visits.is_empty() {
            info!(
                "Player#{}({:?}) can visit nobles: {:?}",
                self.current_player,
                self.players[self.current_player].development_cards.bonus,
                noble_visits
            );
            let (action, noble) = if noble_visits.len() > 1 {
                let snapshot = self.snapshot();
                let select_noble = self.player_actors[self.current_player]
                    .select_noble(snapshot)
                    .await?;
                select_noble.is_valid(self)?;
                (select_noble, self.nobles.get(select_noble.0))
            } else {
                (SelectNoblesAction(noble_visits[0].0), noble_visits[0].1)
            };
            debug_assert!(action.is_valid(self).is_ok());
            info!("Player#{} visited noble: {:?}", self.current_player, noble);
            action.apply(self);
            self.records.push(Record::VisitNoble(ActionRecord::new(
                self.current_player,
                noble,
            )));
        }

        info!(
            "Player#{} ended turn, current points: {}",
            self.current_player,
            self.players[self.current_player].points()
        );
        if self.players[self.current_player].points() >= 15 {
            info!(
                "Player#{} reached 15 points, this is the last turn",
                self.current_player
            );
            self.last_round = true;
        }

        if self.last_round && self.current_player == self.n_players - 1 {
            info!("Game ended");
            self.game_end = true;
            return Ok(Some(self.get_winner()));
        }

        self.current_player = (self.current_player + 1) % self.n_players;
        if self.current_player == 0 {
            trace!("Round {} ended", self.current_round);
            if self.nop_count == self.n_players {
                self.pretty_print();
                error!("All players did nothing, game stuck, {:#?}", self);
                return Ok(Some(smallvec![]));
            }
            self.nop_count = 0;
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
        let winner = if winner_candidates.len() > 1 {
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
        };

        info!("Winner(s): {:?}", winner);

        winner
    }

    fn pretty_print(&self) {
        println!("current round: {}", self.current_round);
        println!("current player: {}", self.current_player);
        print!("tokens remaining:");
        for (color, cnt) in self.tokens.iter().enumerate() {
            print!("    {} {}", Color::try_from(color).unwrap().emoji(), cnt);
        }
        println!();
        println!("card pool remaining: {:?}", self.card_pool.remaining());

        for (tier, cards) in self.card_pool.revealed.iter().enumerate() {
            print!("{}", Tier::try_from(tier).unwrap().emoji());
            for card in cards {
                print!("    ");
                print!("{}", card.bonus.emoji());
                print!(" {:?}", card.points);
                for cnt in card.requires.iter().take(5) {
                    print!(" {:?}", cnt);
                }
            }
            println!();
        }

        for player in self.players.iter() {
            println!("player#{}", player.idx);
            println!("points: {}", player.points());
            println!("tokens:");
            for (color, cnt) in player.tokens.iter().enumerate() {
                print!("    {} {}", Color::try_from(color).unwrap().emoji(), cnt);
            }
            println!();
            println!("development cards:");
            for (color, cards) in player.development_cards.inner.iter().enumerate() {
                print!("{}", Color::try_from(color).unwrap().emoji());
                for card in cards {
                    print!("    ");
                    print!("{}", card.tier.emoji());
                    print!(" {:?}", card.points);
                }
                println!();
            }
            println!("reserved cards:");
            for card in player.reserved_cards.iter() {
                print!("    ");
                print!("{}", card.card.bonus.emoji());
                print!(" {:?}", card.card.points);
                for cnt in card.card.requires.iter().take(5) {
                    print!(" {:?}", cnt);
                }
                println!();
            }
            println!();
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
                .map(|p| p.snapshot(ctx.current_player))
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
