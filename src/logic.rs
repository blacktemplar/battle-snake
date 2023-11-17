// Welcome to
// __________         __    __  .__                               __
// \______   \_____ _/  |__/  |_|  |   ____   ______ ____ _____  |  | __ ____
//  |    |  _/\__  \\   __\   __\  | _/ __ \ /  ___//    \\__  \ |  |/ // __ \
//  |    |   \ / __ \|  |  |  | |  |_\  ___/ \___ \|   |  \/ __ \|    <\  ___/
//  |________/(______/__|  |__| |____/\_____>______>___|__(______/__|__\\_____>
//
// This file can be a nice home for your Battlesnake logic and helper functions.
//
// To get you started we've included code to prevent your Battlesnake from moving backwards.
// For more info see docs.battlesnake.com

use log::info;
use rand::seq::SliceRandom;
use serde_json::{json, Value};
use std::collections::HashMap;

use crate::{Battlesnake, Board, Coord, Game};

// info is called when you create your Battlesnake on play.battlesnake.com
// and controls your Battlesnake's appearance
// TIP: If you open your Battlesnake URL in a browser you should see this data
pub fn info() -> Value {
    info!("INFO");

    return json!({
        "apiversion": "1",
        "author": "blacktemplar",
        "color": "#1d682b",
        "head": "rose",
        "tail": "flytrap",
    });
}

// start is called when your Battlesnake begins a game
pub fn start(_game: &Game, _turn: &u32, _board: &Board, _you: &Battlesnake) {
    info!("GAME START");
}

// end is called when your Battlesnake finishes a game
pub fn end(_game: &Game, _turn: &u32, _board: &Board, _you: &Battlesnake) {
    info!("GAME OVER");
}

// move is called on every turn and returns your next move
// Valid moves are "up", "down", "left", or "right"
// See https://docs.battlesnake.com/api/example-move for available data
pub fn get_move(_game: &Game, turn: &u32, board: &Board, you: &Battlesnake) -> Value {
    let my_head = &you.body[0]; // Coordinates of your head

    let safe_moves: Vec<_> = [
        ("up", Coord::new(my_head.x, my_head.y + 1)),
        ("down", Coord::new(my_head.x, my_head.y.wrapping_sub(1))),
        ("left", Coord::new(my_head.x.wrapping_sub(1), my_head.y)),
        ("right", Coord::new(my_head.x + 1, my_head.y)),
    ]
    .iter()
    .filter(|(_, coord)| is_field_safe(coord, board))
    .map(|(m, _)| *m)
    .collect();

    // Choose a random move from the safe ones
    let chosen = safe_moves
        .choose(&mut rand::thread_rng())
        .unwrap_or(&"left");

    // TODO: Step 4 - Move towards food instead of random, to regain health and survive longer
    // let food = &board.food;

    info!("MOVE {}: {}", turn, chosen);
    return json!({ "move": chosen });
}

fn is_field_safe(coord: &Coord, board: &Board) -> bool {
    coord.x < board.width
        && coord.y < board.height
        && board
            .snakes
            .iter()
            .flat_map(|s| s.body.iter().rev().skip(1))
            .all(|c| c != coord)
}
