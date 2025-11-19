mod map;
mod player;

use map::Map;
use player::Player;
use std::io;
fn main() {
    let mut player = Player::new(0, 0);

    let map = Map::new(5, 5);

    println!("Welcome to the Rusty Dungeon!");
    println!("Map Size: 5x5. Find the treasure at (4,4)!");

    while player.is_alive() {
        println!("\n---------------------------");
        println!(
            "Status: HP: {} | Pos: ({}, {})",
            player.hp, player.x, player.y
        );
        println!("Input (W/A/S/D to move, Q to quit):");

        // Handling Input
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let command = input.trim().to_uppercase();

        // Calculate potential new coordinates
        let (dx, dy) = match command.as_str() {
            "W" => (0, -1), // Up
            "S" => (0, 1),  // Down
            "A" => (-1, 0), // Left
            "D" => (1, 0),  // Right
            "Q" => {
                println!("Quitting game...");
                break;
            }
            _ => {
                println!("Invalid command!");
                (0, 0)
            }
        };

        let target_x = player.x + dx;
        let target_y = player.y + dy;

        if map.is_valid_move(target_x, target_y) {
            player.move_by(dx, dy);

            let message = map.check_event(&mut player);
            println!("{}", message);

            if message.contains("TREASURE") {
                break;
            }
        } else {
            println!("You hit a wall! Cannot move there.");
        }
    }

    if !player.is_alive() {
        println!("GAME OVER! You died.");
    }
}

