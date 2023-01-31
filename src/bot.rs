use crate::coordinates::*;
use crate::user_input::handle_piece_move;
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

fn spawn_task(mut commands: Commands, game_state: ResMut<GameState>) {
    if game_state.bot_turn {
        let thread_pool = AsyncComputeTaskPool::get();
        let position = game_state.board.to_fen();
        let task = thread_pool.spawn(async move { get_best_move_from_stockfish(&position) });
        commands.spawn(BotMoveTask(task));
    }
}

fn extract_coordinates_from_move(string: String) -> (Coordinates, Coordinates) {
    if string.len() != 4 {
        println!("Invalid move string: {}", string);
        panic!("Invalid move string received from Stockfish");
    }
    let from = string.chars().next().expect("Invalid move string");
    let from_number = string.chars().nth(1).expect("Invalid move string");
    let to = string.chars().nth(2).expect("Invalid move string");
    let to_number = string.chars().nth(3).expect("Invalid move string");

    let from_first = ((from as u8 - 48) as char)
        .to_digit(10)
        .expect("Invalid move string") as i32;
    let from_second = from_number.to_digit(10).expect("Invalid move string") as i32;
    let to_first = ((to as u8 - 48) as char)
        .to_digit(10)
        .expect("Invalid move string") as i32;
    let to_second = to_number.to_digit(10).expect("Invalid move string") as i32;

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
    from_to: (Coordinates, Coordinates),
    game_state: &mut ResMut<GameState>,
    game_textures: &Res<GameTextures>,
    state: &mut ResMut<State<GlobalState>>,
    whose_turn: &mut ResMut<State<WhoseTurn>>,
) {
    let from = from_to.0;
    let old_field = game_state
        .board
        .get_field(from)
        .expect("Stockfish returned invalid move");
    let piece = old_field
        .piece
        .as_ref()
        .expect("Stockfish returned invalid move");
    let piece_entity = piece.entity.expect("Stockfish returned invalid move");

    handle_piece_move(
        commands,
        game_state,
        piece_query,
        piece_entity,
        from_to.1,
        state,
        game_textures,
        whose_turn,
    );
}

fn manage_task(
    mut commands: Commands,
    mut tasks: Query<(Entity, &mut BotMoveTask)>,
    mut piece_query: Query<(&mut Handle<Image>, &mut Transform, &mut Piece)>,
    mut game_state: ResMut<GameState>,
    game_textures: Res<GameTextures>,
    mut global_state: ResMut<State<GlobalState>>,
    mut whose_turn: ResMut<State<WhoseTurn>>,
) {
    for (entity, mut task) in &mut tasks {
        if let Some(result) = future::block_on(future::poll_once(&mut task.0)) {
            let best_move = extract_coordinates_from_move(result);
            move_piece(
                &mut commands,
                &mut piece_query,
                best_move,
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

    let difficulty_input = "setoption name Skill Level value 0\n";
    let stockfish_stdin = process.stdin.as_mut().expect("failed to open stdin");
    stockfish_stdin
        .write_all(difficulty_input.as_bytes())
        .expect("failed to write to stdin");

    let input = format!("position fen {}\ngo movetime 500\n", position);

    stockfish_stdin
        .write_all(input.as_bytes())
        .expect("failed to write to stdin");

    std::thread::sleep(std::time::Duration::from_secs(1));

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
        if line.starts_with("bestmove") {
            best_move = line.split_whitespace().nth(1).expect("Parse error");
            break;
        }
    }

    best_move.to_string()
}

fn clear_tasks(mut commands: Commands, tasks: Query<Entity, With<BotMoveTask>>) {
    for entity in tasks.iter() {
        commands.entity(entity).despawn();
    }
}

pub struct BotPlugin;

impl Plugin for BotPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(WhoseTurn::Bot).with_system(spawn_task));
        app.add_system_set(SystemSet::on_update(WhoseTurn::Bot).with_system(manage_task));
        app.add_system_set(SystemSet::on_exit(WhoseTurn::Bot).with_system(clear_tasks));
    }
}
