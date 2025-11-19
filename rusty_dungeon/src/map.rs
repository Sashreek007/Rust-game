use crate::player::Player;

pub struct Map {
    width: i32,
    height: i32,
}

impl Map {
    pub fn new(width: i32, height: i32) -> Map {
        Map { width, height }
    }

    pub fn is_valid_move(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.width && y >= 0 && y < self.height
    }

    pub fn check_event(&self, player: &mut Player) -> String {
        if player.x == 2 && player.y == 2 {
            player.take_damage(20);
            return String::from("XXX You stepped on a TRAP! took 20 damage XXX");
        } else if player.x == 4 && player.y == 4 {
            return String::from("$$$ You found TREASURE! You Win! $$$");
        }

        String::from("The room is empty.")
    }
}
