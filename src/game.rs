const JUMP_TIME: usize = 4;

pub struct Game {
    pub dino_y: usize,

    jumping: bool,
    jump_time: usize,

    pub obstacles: Vec<usize>,

    pub score: usize,
    pub high_score: usize,

    pub game_over: bool,
}

impl Game {
    pub fn new() -> Self {
        Self {
            dino_y: 0,
            jumping: false,
            jump_time: 0,
            obstacles: Vec::new(),
            score: 0,
            high_score: 0,
            game_over: false,
        }
    }

    pub fn update(&mut self) {
        for obstacle in self.obstacles.iter_mut() {
            *obstacle -= 1;
        }

        if self.jumping {
            self.jump();
        }

        self.score += 1;

        if self.game_over {
            self.end_game();
        }
    }

    fn jump(&mut self) {
        if self.jump_time > JUMP_TIME/2 {
            self.dino_y -= 1;
        } else{
            self.dino_y += 1;
        }

        self.jump_time += 1;

        if self.jump_time == JUMP_TIME {
            self.jump_time = 0;
            self.jumping = false;
        }
    }

    fn end_game(&mut self) {
        if self.score > self.high_score {
            self.high_score = self.score;
        }

        self.score = 0;
    }
}
