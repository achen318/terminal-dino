use rand::random;

const JUMP_TIME: u16 = 8;
const WIDTH: u16 = 100;

pub struct Game {
    pub dino_y: u16,

    pub jumping: bool,
    jump_time: u16,

    pub obstacles: Vec<u16>,

    score_cooldown: u16,
    pub score: u16,
}

impl Game {
    pub fn new() -> Self {
        Self {
            dino_y: 0,
            jumping: false,
            jump_time: 0,
            obstacles: vec![0; WIDTH as usize],
            score_cooldown: 0,
            score: 0,
        }
    }

    pub fn update(&mut self) -> bool {
        // Shift the obstacles left
        for i in 1..WIDTH - 3 {
            self.obstacles[i as usize] = self.obstacles[i as usize + 2];
            self.obstacles[i as usize + 2] = 0;
        }

        // Randomly generate a new obstacle height
        if self.obstacles[WIDTH as usize - 20..]
            .iter()
            .all(|&x| x == 0)
        {
            self.obstacles[WIDTH as usize - 2] = random::<u16>() % 3;
        }

        // Jump
        if self.jumping {
            self.jump();
        }

        // Add score every 4 ticks
        if self.score_cooldown == 0 {
            self.score += 1;
            self.score_cooldown = 4;
        } else {
            self.score_cooldown -= 1;
        }

        // Check for collision
        if self.collision() {
            println!("Game Over! Score: {}", self.score);
            return false;
        }

        true
    }

    fn jump(&mut self) {
        if self.jump_time >= JUMP_TIME / 2 {
            self.dino_y -= 1;
        } else {
            self.dino_y += 1;
        }

        self.jump_time += 1;

        if self.jump_time == JUMP_TIME {
            self.jump_time = 0;
            self.jumping = false;
        }
    }

    fn collision(&self) -> bool {
        self.obstacles[2] > 0 && self.dino_y < self.obstacles[2]
    }
}
