pub struct Player {
    pub x: i32,
    pub y: i32,
    pub hp: u32,
}

impl Player {
    pub fn new(x: i32, y: i32) -> Player {
        Player { x, y, hp: 100 }
    }

    pub fn move_by(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }

    pub fn take_damage(&mut self, amount: u32) {
        if amount > self.hp {
            self.hp = 0;
        } else {
            self.hp -= amount;
        }
    }
    pub fn is_alive(&self) -> bool {
        self.hp > 0
    }
}
