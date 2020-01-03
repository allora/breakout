#[derive(Default)]
pub struct PauseState {
    pub paused: bool,
}

#[derive(Default)]
pub struct ScoreBoard {
    pub current_score: i32,
}

#[derive(Default)]
pub struct LevelInfo {
    pub num_blocks_remaining: i32,
    pub num_lives_remaining: i32,
}
