use super::SupervisorError;
use uuid::Uuid;

#[derive(Debug)]
pub struct Supervisor;

impl Supervisor {
    pub async fn new(_uuid: Uuid) -> Result<Self, SupervisorError> {
        Ok(Self)
    }

    pub async fn report_game_ends(
        &mut self,
        _winners: &[usize],
        _timeout: bool,
        _error: bool,
    ) -> Result<(), SupervisorError> {
        Ok(())
    }

    pub async fn prepare_player_change(
        &mut self,
        _next_player: usize,
    ) -> Result<(), SupervisorError> {
        Ok(())
    }
}
