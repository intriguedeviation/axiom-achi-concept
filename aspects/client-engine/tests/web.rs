//! WebAssembly smoke tests for exported client-engine behavior.

#![cfg(target_arch = "wasm32")]

use achi::{define_player, start_game, BoardPhase, PlayerSide};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn wasm_exports_start_a_game() {
    let red = define_player(PlayerSide::Red, "Red").expect("red player can be defined");
    let black = define_player(PlayerSide::Black, "Black").expect("black player can be defined");
    let grid = start_game(&red, &black).expect("game can be started");

    assert_eq!(grid.phase(), BoardPhase::Placement);
    assert_eq!(grid.active_player(), PlayerSide::Red);
}
