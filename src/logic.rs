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
use serde_json::{json, Value};

use crate::{AppConfig, Battlesnake, Board, Coord, Game};

// info is called when you create your Battlesnake on play.battlesnake.com
// and controls your Battlesnake's appearance
// TIP: If you open your Battlesnake URL in a browser you should see this data
pub fn info(config: &AppConfig) -> Value {
    info!("INFO");

    return json!({
        "apiversion": "1",
        "author": "blacktemplar",
        "color": config.color, //"#1d682b",
        "head": config.head, //"rose",
        "tail": config.tail, //"flytrap",
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
    .into_iter()
    .filter(|(_, coord)| is_field_safe(coord, board))
    .collect();

    // Choose best move
    let chosen = choose(safe_moves, board).unwrap_or("left");

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

fn choose(moves: Vec<(&'static str, Coord)>, board: &Board) -> Option<&'static str> {
    // Choose a random move
    /*moves
    .choose(&mut rand::thread_rng())
    .map(|(n, _)| *n)
    .unwrap_or("left")*/

    println!("Possible moves: {:?}", moves);

    // choose move closest to any food
    moves
        .into_iter()
        .min_by_key(|(_, c)| distance_to_next_food(c, board))
        .map(|(n, _)| n)
}

fn distance_to_next_food(coord: &Coord, board: &Board) -> u32 {
    board
        .food
        .iter()
        .map(|f| coord.dist(f))
        .min()
        .unwrap_or(u32::MAX)
}
