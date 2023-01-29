use crate::user_input::handle_piece_move;
use crate::{coordinates::*, field::Field};
use bevy::{
    prelude::*,
    tasks::{AsyncComputeTaskPool, Task},
};
use futures_lite::future;
use reqwest::Client;

use crate::*;

const CHESS_API_ACTION: &str = "http://www.chessdb.cn/cdb.php?action=";

fn get_chess_api_querybest(board: String, query: &str) -> String {
    CHESS_API_ACTION.to_string() + query + "&board=" + &board + "&json=1"
}

#[derive(Component)]
struct BotMoveTask(Task<String>);

fn spawn_task(mut commands: Commands, state: ResMut<GameState>) {
    if state.bot_turn {
        let thread_pool = AsyncComputeTaskPool::get();
        let board_clone = state.board.clone();
        let task = thread_pool.spawn(async move {
            let client = Client::new();
            let res = send_request(&client, board_clone);
            res
        });
        commands.spawn(BotMoveTask(task));
        //println!("spawned task");
    }
}

fn extract_coordinates_from_move(string: String) -> (Coordinates, Coordinates) {
    println!("string: {:?}", string);
    let best_move = string
        .split(',')
        .nth(1)
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .replace("\"", "")
        .replace("}", "")
        .replace("\"", "");

    println!("best move: {:?}", best_move);
    let from = best_move.chars().nth(0).unwrap();
    let from_number = best_move.chars().nth(1).unwrap();
    let to = best_move.chars().nth(2).unwrap();
    let to_number = best_move.chars().nth(3).unwrap();

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

    //game_state.bot_turn = false;
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
            println!("result: {:?}", best_move);
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

// function that sends request to chessdb.cn
#[tokio::main]
async fn send_request(client: &reqwest::Client, board: Board) -> String {
    let chess_api_query: &str = "query";
    let mut board = board.board_to_fen(); //"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1";
    println!("board: {}", board);

    board += " b - - 0 1";

    let request_url = get_chess_api_querybest(board, chess_api_query);
    println!("{}", request_url);

    let response = client.get(&request_url).send().await;
    match response {
        Ok(res) => {
            let res = res.text().await;
            match res {
                Ok(res) => res,
                Err(e) => {
                    println!("Error: {}", e);
                    "".to_string()
                }
            }
        }
        Err(e) => {
            println!("Error: {}", e);
            "".to_string()
        }
    }
}

pub struct BotPlugin;

impl Plugin for BotPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(WhoseTurn::Bot).with_system(spawn_task));
        app.add_system_set(SystemSet::on_update(WhoseTurn::Bot).with_system(manage_task));
    }
}
