use crate::user_input::handle_piece_move;
use crate::{coordinates::*, field::Field};
use bevy::{
    prelude::*,
    tasks::{AsyncComputeTaskPool, Task},
};
use futures_lite::future;
use std::io::Read;
use std::{io::Write, process::Command};

use crate::*;

#[derive(Component)]
struct BotMoveTask(Task<String>);

fn spawn_task(mut commands: Commands, state: ResMut<GameState>) {
    if state.bot_turn && state.winner == None {
        let thread_pool = AsyncComputeTaskPool::get();
        let board_clone = state.board.clone();
        let task = thread_pool.spawn(async move {
            let position = board_clone.to_fen();
            let res = get_best_move_from_stockfish(&position);
            res
        });
        commands.spawn(BotMoveTask(task));
    }
}

fn extract_coordinates_from_move(string: String) -> (Coordinates, Coordinates) {
    println!("Move: {}", string);
    if string.len() != 4 {
        println!("Invalid move string: {}", string);
        panic!("Invalid move string");
    }
    let from = string.chars().nth(0).unwrap();
    let from_number = string.chars().nth(1).unwrap();
    let to = string.chars().nth(2).unwrap();
    let to_number = string.chars().nth(3).unwrap();

    let from_first = ((from as u8 - 48) as char).to_digit(10).unwrap() as i32;
    let from_second = from_number.to_digit(10).unwrap() as i32;
    let to_first = ((to as u8 - 48) as char).to_digit(10).unwrap() as i32;
    let to_second = to_number.to_digit(10).unwrap() as i32;

    let from = Coordinates {
        x: from_first,
        y: from_second,
    };
    let to = Coordinates {
        x: to_first,
        y: to_second,
    };

    (from, to)
}

fn move_piece(
    commands: &mut Commands,
    piece_query: &mut Query<(&mut Handle<Image>, &mut Transform, &mut Piece)>,
    field_query: &mut Query<(&mut Sprite, &mut Field)>,
    from: Coordinates,
    to: Coordinates,
    game_state: &mut ResMut<GameState>,
    game_textures: &Res<GameTextures>,
    state: &mut ResMut<State<GlobalState>>,
    whose_turn: &mut ResMut<State<WhoseTurn>>,
) {
    let old_field_id = game_state.board.get_field_entity(from);
    let old_field_query_item = field_query.get_mut(old_field_id.unwrap());
    let old_field = old_field_query_item.unwrap().1;
    let piece_entity = old_field.piece.clone().unwrap().entity.unwrap();
    // print board
    println!("{}", game_state.board.to_fen());

    handle_piece_move(
        commands,
        game_state,
        piece_query,
        field_query,
        piece_entity,
        to,
        state,
        game_textures,
        whose_turn,
    );

    println!("{}", game_state.board.to_fen());
}

fn manage_task(
    mut commands: Commands,
    mut tasks: Query<(Entity, &mut BotMoveTask)>,
    mut piece_query: Query<(&mut Handle<Image>, &mut Transform, &mut Piece)>,
    mut field_query: Query<(&mut Sprite, &mut Field)>,
    mut game_state: ResMut<GameState>,
    game_textures: Res<GameTextures>,
    mut global_state: ResMut<State<GlobalState>>,
    mut whose_turn: ResMut<State<WhoseTurn>>,
) {
    for (entity, mut task) in &mut tasks {
        if let Some(result) = future::block_on(future::poll_once(&mut task.0)) {
            let best_move = extract_coordinates_from_move(result);
            //println!("best_move: {:?}", best_move);
            move_piece(
                &mut commands,
                &mut piece_query,
                &mut field_query,
                best_move.0,
                best_move.1,
                &mut game_state,
                &game_textures,
                &mut global_state,
                &mut whose_turn,
            );

            commands.entity(entity).despawn();
        }
    }
}

fn get_best_move_from_stockfish(position: &str) -> String {
    let mut process = Command::new("stockfish")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("failed to execute stockfish");

    let input = format!("position fen {}\ngo movetime 500\n", position);
    //println!("input: \n{}", input);

    let stockfish_stdin = process.stdin.as_mut().expect("failed to open stdin");

    stockfish_stdin
        .write_all(input.as_bytes())
        .expect("failed to write to stdin");

    std::thread::sleep(std::time::Duration::from_secs(1));
    //process.kill().expect("failed to kill process");
    // send quit to stdin
    stockfish_stdin
        .write_all(b"quit\n")
        .expect("failed to write to stdin");

    let stockfish_stdout = process.stdout.as_mut().expect("failed to open stdout");

    let mut output = String::new();
    stockfish_stdout
        .read_to_string(&mut output)
        .expect("failed to read stdout");

    let mut best_move = "";
    for line in output.lines() {
        //println!("line: {}", line);
        if line.starts_with("bestmove") {
            best_move = line.split_whitespace().nth(1).unwrap();
            break;
        }
    }

    best_move.to_string()
}

/*fn get_best_move_from_stockfish(position: &str) -> String {
    let mut process = Command::new("stockfish")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("failed to execute stockfish");

    let input = format!("position fen {}\ngo movetime 1000\n", position);

    let stdin = process.stdin.as_mut().expect("failed to open stdin");
    stdin
        .write_all(input.as_bytes())
        .expect("failed to write to stdin");

    let output = process.wait_with_output().expect("failed to read stdout");
    let output = String::from_utf8(output.stdout).expect("failed to parse stdout");

    let mut best_move = "";
    for line in output.lines() {
        if line.starts_with("bestmove") {
            best_move = line.split_whitespace().nth(1).unwrap();
            break;
        }
    }
    if best_move == "" {
        for line in output.lines() {
            println!("{}", line);
            if line.starts_with("info depth") {
                best_move = line.split_whitespace().nth(10).unwrap();
                break;
            }
        }
    }
    best_move.to_string()
}*/

pub struct BotPlugin;

impl Plugin for BotPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(WhoseTurn::Bot).with_system(spawn_task));
        app.add_system_set(SystemSet::on_update(WhoseTurn::Bot).with_system(manage_task));
    }
}
