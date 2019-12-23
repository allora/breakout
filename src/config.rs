
use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ArenaConfig {
    pub width: f32,
    pub height: f32,
    pub paddlepos: f32,
}

impl Default for ArenaConfig {
    fn default() -> Self {
        ArenaConfig {
            width: 500.0,
            height: 800.0,
            paddlepos: 50.0,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BallConfig {
    pub radius: f32,
}

impl Default for BallConfig {
    fn default() -> Self {
        BallConfig {
            radius: 2.5,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PaddleConfig {
    pub velocity: f32,
    pub width: f32,
    pub height: f32,
}

impl Default for PaddleConfig {
    fn default() -> Self {
        PaddleConfig {
            velocity: 5.0,
            width: 10.0,
            height: 5.0,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BlockConfig {
    pub width: f32,
    pub height: f32,
    pub hits: u8,
}

impl Default for BlockConfig {
    fn default() -> Self {
        BlockConfig {
            width: 10.0,
            height: 5.0,
            hits: 0,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LevelConfig {
    pub layout: Vec<Vec<i32>>,
}

impl Default for LevelConfig {
    fn default() -> Self {
        LevelConfig {
            layout: vec![
                vec![0,0,0,0,0,0,0,0],
                vec![0,0,0,0,0,0,0,0],
                vec![0,0,0,0,0,0,0,0],
                vec![0,0,0,0,0,0,0,0],
                vec![0,0,0,0,0,0,0,0],
                vec![0,0,0,0,0,0,0,0],
            ]
        }
    }
}

// Config data
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct BreakoutConfig {
    pub arena: ArenaConfig,
    pub ball: BallConfig,
    pub paddle: PaddleConfig,
    pub block: BlockConfig,
    pub level: LevelConfig,
}