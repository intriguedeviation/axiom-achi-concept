//! Client-side Achi domain engine generated from the repository specification.
//!
//! This crate keeps the specification's names visible at the API boundary:
//! players, tokens, grids, placements, and the declared domain behaviors are
//! represented as Rust types and exported to JavaScript with `wasm-bindgen`.
//! Rule failures are returned as stable string error codes that match the
//! declared exception names in `specs/specification.yml`.

mod utils;

use std::convert::TryFrom;
use std::fmt;
use std::sync::atomic::{AtomicU32, Ordering};
use wasm_bindgen::prelude::*;

/// Number of tokens owned by each player entity.
const TOKEN_COUNT_PER_PLAYER: usize = 4;
/// Number of legal token positions in the grid entity.
const BOARD_POSITION_COUNT: usize = 9;
/// Number of winning three-position alignments in the grid entity.
const VICTORY_ALIGNMENT_COUNT: usize = 8;

static NEXT_PLAYER_ID: AtomicU32 = AtomicU32::new(1);

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// The side assigned to a player.
pub enum PlayerSide {
    /// The red player side.
    Red = 0,
    /// The black player side.
    Black = 1,
}

impl PlayerSide {
    fn opponent(self) -> Self {
        match self {
            Self::Red => Self::Black,
            Self::Black => Self::Red,
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// The current rule phase for a grid.
pub enum BoardPhase {
    /// Tokens are being placed onto empty grid positions.
    Placement = 0,
    /// Existing tokens are being moved between adjacent positions.
    Movement = 1,
}

/// A validated position on the nine-position Achi grid.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct TokenPosition(u8);

impl TokenPosition {
    /// Returns the scalar position value declared by the domain primitive.
    pub fn value(self) -> u8 {
        self.0
    }
}

impl TryFrom<u8> for TokenPosition {
    type Error = DomainError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value < BOARD_POSITION_COUNT as u8 {
            Ok(Self(value))
        } else {
            Err(DomainError::InvalidTokenPosition)
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
/// A player-owned token with an optional grid position.
pub struct Token {
    position: Option<TokenPosition>,
}

impl Token {
    fn new() -> Self {
        Self { position: None }
    }

    /// Returns true when this token has been placed onto the grid.
    pub fn is_position_set(&self) -> bool {
        self.position.is_some()
    }

    /// Returns the token's current grid position value, when set.
    pub fn position_value(&self) -> Option<u8> {
        self.position.map(TokenPosition::value)
    }
}

#[wasm_bindgen]
impl Token {
    #[wasm_bindgen(js_name = isPositionSet)]
    pub fn is_position_set_js(&self) -> bool {
        self.is_position_set()
    }

    #[wasm_bindgen(js_name = position)]
    pub fn position_js(&self) -> Option<u8> {
        self.position_value()
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
/// A domain player with a side, label, and four owned tokens.
pub struct Player {
    id: u32,
    side: PlayerSide,
    label: String,
    tokens: Vec<Token>,
}

impl Player {
    /// Returns the player's side.
    pub fn side_value(&self) -> PlayerSide {
        self.side
    }

    /// Returns the validated player label.
    pub fn label_value(&self) -> &str {
        &self.label
    }

    /// Returns the number of tokens owned by the player.
    pub fn token_count(&self) -> usize {
        self.tokens.len()
    }

    fn token(&self, index: usize) -> Result<&Token, DomainError> {
        self.tokens
            .get(index)
            .ok_or(DomainError::TokenNotOwnedByPlayer)
    }

    fn token_mut(&mut self, index: usize) -> Result<&mut Token, DomainError> {
        self.tokens
            .get_mut(index)
            .ok_or(DomainError::TokenNotOwnedByPlayer)
    }

    fn token_positions(&self) -> impl Iterator<Item = TokenPosition> + '_ {
        self.tokens.iter().filter_map(|token| token.position)
    }

    fn all_tokens_set(&self) -> bool {
        self.tokens.iter().all(Token::is_position_set)
    }

    fn any_token_unset(&self) -> bool {
        self.tokens.iter().any(|token| !token.is_position_set())
    }
}

#[wasm_bindgen]
impl Player {
    #[wasm_bindgen(getter)]
    pub fn side(&self) -> PlayerSide {
        self.side
    }

    #[wasm_bindgen(getter)]
    pub fn label(&self) -> String {
        self.label.clone()
    }

    #[wasm_bindgen(js_name = tokenCount)]
    pub fn token_count_js(&self) -> usize {
        self.token_count()
    }

    #[wasm_bindgen(js_name = tokenPosition)]
    pub fn token_position_js(&self, index: usize) -> Result<Option<u8>, JsValue> {
        self.token(index)
            .map(Token::position_value)
            .map_err(DomainError::into)
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, PartialEq)]
/// A placement record produced when a token position changes.
pub struct Placement {
    turn: u8,
    mode: BoardPhase,
    player_side: PlayerSide,
    token_index: usize,
    position: TokenPosition,
}

impl Placement {
    fn new(turn: u8, player_side: PlayerSide, token_index: usize, position: TokenPosition) -> Self {
        Self {
            turn,
            mode: BoardPhase::Placement,
            player_side,
            token_index,
            position,
        }
    }
}

#[wasm_bindgen]
impl Placement {
    #[wasm_bindgen(getter)]
    pub fn turn(&self) -> u8 {
        self.turn
    }

    #[wasm_bindgen(getter)]
    pub fn mode(&self) -> BoardPhase {
        self.mode
    }

    #[wasm_bindgen(js_name = playerSide)]
    pub fn player_side(&self) -> PlayerSide {
        self.player_side
    }

    #[wasm_bindgen(js_name = tokenIndex)]
    pub fn token_index(&self) -> usize {
        self.token_index
    }

    #[wasm_bindgen(getter)]
    pub fn position(&self) -> u8 {
        self.position.value()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Rust-native result for the `victoryAchieved` behavior.
pub struct Victory {
    /// Whether any player currently has a victory alignment.
    pub result: bool,
    /// The winning player side when `result` is true.
    pub player: Option<PlayerSide>,
}

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, PartialEq)]
/// WebAssembly-compatible result for the `victoryAchieved` behavior.
pub struct VictoryResult {
    result: bool,
    player: i8,
}

#[wasm_bindgen]
impl VictoryResult {
    #[wasm_bindgen(getter)]
    pub fn result(&self) -> bool {
        self.result
    }

    #[wasm_bindgen(getter)]
    pub fn player(&self) -> i8 {
        self.player
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
/// The Achi grid aggregate.
///
/// The grid owns the active phase, active player, legal positions, adjacency
/// pairs, victory alignments, player snapshots, and placement history needed to
/// enforce the specification-defined behaviors.
pub struct Grid {
    positions: Vec<TokenPosition>,
    phase: BoardPhase,
    active_player: PlayerSide,
    adjacents: Vec<(TokenPosition, TokenPosition)>,
    victory_alignments: Vec<[TokenPosition; 3]>,
    red_player: Player,
    black_player: Player,
    placements: Vec<Placement>,
}

impl Grid {
    /// Returns the current grid phase.
    pub fn phase_value(&self) -> BoardPhase {
        self.phase
    }

    /// Returns the side whose turn is currently active.
    pub fn active_player_value(&self) -> PlayerSide {
        self.active_player
    }

    /// Returns the red player associated with this grid.
    pub fn red_player(&self) -> &Player {
        &self.red_player
    }

    /// Returns the black player associated with this grid.
    pub fn black_player(&self) -> &Player {
        &self.black_player
    }

    /// Returns the placement history recorded by placement and movement.
    pub fn placements(&self) -> &[Placement] {
        &self.placements
    }

    fn player(&self, side: PlayerSide) -> &Player {
        match side {
            PlayerSide::Red => &self.red_player,
            PlayerSide::Black => &self.black_player,
        }
    }

    fn player_mut(&mut self, side: PlayerSide) -> &mut Player {
        match side {
            PlayerSide::Red => &mut self.red_player,
            PlayerSide::Black => &mut self.black_player,
        }
    }

    fn player_from_input(&self, player: &Player) -> Result<PlayerSide, DomainError> {
        match player.side {
            PlayerSide::Red if self.red_player.id == player.id => Ok(PlayerSide::Red),
            PlayerSide::Black if self.black_player.id == player.id => Ok(PlayerSide::Black),
            _ => Err(DomainError::TokenNotOwnedByPlayer),
        }
    }

    fn is_occupied(&self, position: TokenPosition) -> Option<PlayerSide> {
        if self.red_player.token_positions().any(|p| p == position) {
            Some(PlayerSide::Red)
        } else if self.black_player.token_positions().any(|p| p == position) {
            Some(PlayerSide::Black)
        } else {
            None
        }
    }

    fn is_adjacent(&self, from: TokenPosition, to: TokenPosition) -> bool {
        self.adjacents
            .iter()
            .any(|(a, b)| (*a == from && *b == to) || (*a == to && *b == from))
    }

    fn append_placement(
        &mut self,
        side: PlayerSide,
        token_index: usize,
        position: TokenPosition,
    ) -> Placement {
        let turn = self.placements.len() as u8;
        let placement = Placement::new(turn, side, token_index, position);
        self.placements.push(placement.clone());
        placement
    }

    fn validate_place_token(
        &self,
        side: PlayerSide,
        position: TokenPosition,
    ) -> Result<(), DomainError> {
        if self.active_player != side {
            return Err(DomainError::WrongPlayerTurn);
        }

        match self.is_occupied(position) {
            Some(PlayerSide::Red) => Err(DomainError::IllegalRedTokenPlacement),
            Some(PlayerSide::Black) => Err(DomainError::IllegalBlackTokenPlacement),
            None => Ok(()),
        }
    }

    fn validate_move_token(
        &self,
        side: PlayerSide,
        token_index: usize,
        position: TokenPosition,
    ) -> Result<(), DomainError> {
        if self.active_player != side {
            return Err(DomainError::WrongPlayerTurn);
        }

        let current_position = self
            .player(side)
            .token(token_index)?
            .position
            .ok_or(DomainError::MovementNotAdjacent)?;

        if current_position == position {
            return Err(DomainError::SamePositionNotAllowed);
        }

        match self.is_occupied(position) {
            Some(PlayerSide::Red) => return Err(DomainError::IllegalRedTokenMovement),
            Some(PlayerSide::Black) => return Err(DomainError::IllegalBlackTokenMovement),
            None => {}
        }

        if !self.is_adjacent(current_position, position) {
            return Err(DomainError::MovementNotAdjacent);
        }

        Ok(())
    }

    /// Implements the `placeToken` behavior.
    ///
    /// The behavior requires the active player's turn and an unoccupied target
    /// position, then records the placement, sets the token position, and
    /// advances the active player.
    pub fn place_token(
        &mut self,
        player: &Player,
        token_index: usize,
        position: u8,
    ) -> Result<Placement, DomainError> {
        let side = self.player_from_input(player)?;
        let position = TokenPosition::try_from(position)?;
        self.validate_place_token(side, position)?;

        self.player_mut(side).token_mut(token_index)?.position = Some(position);
        let placement = self.append_placement(side, token_index, position);
        self.active_player = self.active_player.opponent();
        Ok(placement)
    }

    /// Implements the `moveToken` behavior.
    ///
    /// The behavior requires the active player's turn, an unoccupied target,
    /// a different position, and adjacency from the token's current position.
    pub fn move_token(
        &mut self,
        player: &Player,
        token_index: usize,
        position: u8,
    ) -> Result<Placement, DomainError> {
        let side = self.player_from_input(player)?;
        let position = TokenPosition::try_from(position)?;
        self.validate_move_token(side, token_index, position)?;

        self.player_mut(side).token_mut(token_index)?.position = Some(position);
        let placement = self.append_placement(side, token_index, position);
        self.active_player = self.active_player.opponent();
        Ok(placement)
    }

    /// Implements the `victoryAchieved` behavior.
    ///
    /// The behavior requires all red and black tokens to have positions before
    /// evaluating the grid's declared victory alignments.
    pub fn victory_achieved(&self) -> Result<Victory, DomainError> {
        if self.red_player.any_token_unset() {
            return Err(DomainError::RedPlayerUnsetTokens);
        }
        if self.black_player.any_token_unset() {
            return Err(DomainError::BlackPlayerUnsetTokens);
        }

        if self.has_victory(PlayerSide::Red) {
            Ok(Victory {
                result: true,
                player: Some(PlayerSide::Red),
            })
        } else if self.has_victory(PlayerSide::Black) {
            Ok(Victory {
                result: true,
                player: Some(PlayerSide::Black),
            })
        } else {
            Ok(Victory {
                result: false,
                player: None,
            })
        }
    }

    /// Implements the `changeGridPhase` behavior.
    ///
    /// The behavior transitions the same grid from placement to movement after
    /// all player tokens have been positioned.
    pub fn change_grid_phase(&mut self) -> Result<&Grid, DomainError> {
        if self.phase != BoardPhase::Placement {
            return Err(DomainError::WrongGridPhase);
        }
        if !self.red_player.all_tokens_set() {
            return Err(DomainError::RedPlayerUnsetTokens);
        }
        if !self.black_player.all_tokens_set() {
            return Err(DomainError::BlackPlayerUnsetTokens);
        }

        self.phase = BoardPhase::Movement;
        Ok(self)
    }

    fn has_victory(&self, side: PlayerSide) -> bool {
        let positions: Vec<TokenPosition> = self.player(side).token_positions().collect();
        self.victory_alignments.iter().any(|alignment| {
            alignment
                .iter()
                .all(|position| positions.contains(position))
        })
    }
}

#[wasm_bindgen]
impl Grid {
    #[wasm_bindgen(getter)]
    pub fn phase(&self) -> BoardPhase {
        self.phase
    }

    #[wasm_bindgen(js_name = activePlayer)]
    pub fn active_player(&self) -> PlayerSide {
        self.active_player
    }

    #[wasm_bindgen(js_name = positionCount)]
    pub fn position_count(&self) -> usize {
        self.positions.len()
    }

    #[wasm_bindgen(js_name = victoryAlignmentCount)]
    pub fn victory_alignment_count(&self) -> usize {
        self.victory_alignments.len()
    }

    #[wasm_bindgen(js_name = placementCount)]
    pub fn placement_count(&self) -> usize {
        self.placements.len()
    }

    #[wasm_bindgen(js_name = redTokenPosition)]
    pub fn red_token_position(&self, index: usize) -> Result<Option<u8>, JsValue> {
        self.red_player
            .token(index)
            .map(Token::position_value)
            .map_err(DomainError::into)
    }

    #[wasm_bindgen(js_name = blackTokenPosition)]
    pub fn black_token_position(&self, index: usize) -> Result<Option<u8>, JsValue> {
        self.black_player
            .token(index)
            .map(Token::position_value)
            .map_err(DomainError::into)
    }

    #[wasm_bindgen(js_name = placeToken)]
    pub fn place_token_js(
        &mut self,
        player: &Player,
        token_index: usize,
        position: u8,
    ) -> Result<Placement, JsValue> {
        self.place_token(player, token_index, position)
            .map_err(DomainError::into)
    }

    #[wasm_bindgen(js_name = moveToken)]
    pub fn move_token_js(
        &mut self,
        player: &Player,
        token_index: usize,
        position: u8,
    ) -> Result<Placement, JsValue> {
        self.move_token(player, token_index, position)
            .map_err(DomainError::into)
    }

    #[wasm_bindgen(js_name = victoryAchieved)]
    pub fn victory_achieved_js(&self) -> Result<VictoryResult, JsValue> {
        let victory = self.victory_achieved()?;
        let player = match victory.player {
            Some(PlayerSide::Red) => PlayerSide::Red as i8,
            Some(PlayerSide::Black) => PlayerSide::Black as i8,
            None => -1,
        };
        Ok(VictoryResult {
            result: victory.result,
            player,
        })
    }

    #[wasm_bindgen(js_name = changeGridPhase)]
    pub fn change_grid_phase_js(&mut self) -> Result<(), JsValue> {
        self.change_grid_phase()
            .map(|_| ())
            .map_err(DomainError::into)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// Domain behavior failure codes.
///
/// Variants named in the specification map directly to the declared exception
/// names. Additional validation variants protect primitive constraints and
/// ownership checks needed by the concrete Rust API.
pub enum DomainError {
    InvalidPlayerLabel,
    InvalidTokenPosition,
    AdversarySamePlayer,
    WrongPlayerTurn,
    IllegalRedTokenPlacement,
    IllegalBlackTokenPlacement,
    IllegalRedTokenMovement,
    IllegalBlackTokenMovement,
    SamePositionNotAllowed,
    MovementNotAdjacent,
    RedPlayerUnsetTokens,
    BlackPlayerUnsetTokens,
    WrongGridPhase,
    TokenNotOwnedByPlayer,
}

impl DomainError {
    /// Returns the stable string code exposed to JavaScript callers and tests.
    pub fn code(self) -> &'static str {
        match self {
            Self::InvalidPlayerLabel => "invalid_player_label",
            Self::InvalidTokenPosition => "invalid_token_position",
            Self::AdversarySamePlayer => "adversary_same_player",
            Self::WrongPlayerTurn => "wrong_player_turn",
            Self::IllegalRedTokenPlacement => "illegal_red_token_placement",
            Self::IllegalBlackTokenPlacement => "illegal_black_token_placement",
            Self::IllegalRedTokenMovement => "illegal_red_token_movement",
            Self::IllegalBlackTokenMovement => "illegal_black_token_movement",
            Self::SamePositionNotAllowed => "same_position_not_allowed",
            Self::MovementNotAdjacent => "movement_not_adjacent",
            Self::RedPlayerUnsetTokens => "red_player_unset_tokens",
            Self::BlackPlayerUnsetTokens => "black_player_unset_tokens",
            Self::WrongGridPhase => "wrong_grid_phase",
            Self::TokenNotOwnedByPlayer => "token_not_owned_by_player",
        }
    }
}

impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.code())
    }
}

impl std::error::Error for DomainError {}

impl From<DomainError> for JsValue {
    fn from(error: DomainError) -> Self {
        JsValue::from_str(error.code())
    }
}

#[wasm_bindgen(js_name = definePlayer)]
/// WebAssembly export for `definePlayer`.
pub fn define_player(side: PlayerSide, label: &str) -> Result<Player, JsValue> {
    define_player_domain(side, label).map_err(DomainError::into)
}

/// Implements the `definePlayer` behavior for Rust callers.
pub fn define_player_domain(side: PlayerSide, label: &str) -> Result<Player, DomainError> {
    if label.trim().is_empty() || label.chars().count() > 35 {
        return Err(DomainError::InvalidPlayerLabel);
    }

    Ok(Player {
        id: NEXT_PLAYER_ID.fetch_add(1, Ordering::Relaxed),
        side,
        label: label.to_owned(),
        tokens: (0..TOKEN_COUNT_PER_PLAYER).map(|_| Token::new()).collect(),
    })
}

#[wasm_bindgen(js_name = startGame)]
/// WebAssembly export for `startGame`.
pub fn start_game(red_player: &Player, black_player: &Player) -> Result<Grid, JsValue> {
    start_game_domain(red_player, black_player).map_err(DomainError::into)
}

/// Implements the `startGame` behavior for Rust callers.
pub fn start_game_domain(red_player: &Player, black_player: &Player) -> Result<Grid, DomainError> {
    if red_player.id == black_player.id {
        return Err(DomainError::AdversarySamePlayer);
    }

    let positions = (0..BOARD_POSITION_COUNT as u8)
        .map(TokenPosition)
        .collect::<Vec<_>>();
    let victory_alignments = victory_alignments();
    let adjacents = adjacents_from_alignments(&victory_alignments);

    debug_assert_eq!(positions.len(), BOARD_POSITION_COUNT);
    debug_assert_eq!(victory_alignments.len(), VICTORY_ALIGNMENT_COUNT);

    Ok(Grid {
        positions,
        phase: BoardPhase::Placement,
        active_player: PlayerSide::Red,
        adjacents,
        victory_alignments,
        red_player: red_player.clone(),
        black_player: black_player.clone(),
        placements: Vec::new(),
    })
}

fn victory_alignments() -> Vec<[TokenPosition; 3]> {
    vec![
        [TokenPosition(0), TokenPosition(1), TokenPosition(2)],
        [TokenPosition(3), TokenPosition(4), TokenPosition(5)],
        [TokenPosition(6), TokenPosition(7), TokenPosition(8)],
        [TokenPosition(0), TokenPosition(3), TokenPosition(6)],
        [TokenPosition(1), TokenPosition(4), TokenPosition(7)],
        [TokenPosition(2), TokenPosition(5), TokenPosition(8)],
        [TokenPosition(0), TokenPosition(4), TokenPosition(8)],
        [TokenPosition(2), TokenPosition(4), TokenPosition(6)],
    ]
}

fn adjacents_from_alignments(
    alignments: &[[TokenPosition; 3]],
) -> Vec<(TokenPosition, TokenPosition)> {
    let mut adjacents = Vec::new();

    for alignment in alignments {
        for pair in [(alignment[0], alignment[1]), (alignment[1], alignment[2])] {
            if !adjacents.contains(&pair) && !adjacents.contains(&(pair.1, pair.0)) {
                adjacents.push(pair);
            }
        }
    }

    adjacents
}

#[wasm_bindgen(start)]
/// Initializes optional runtime support for WebAssembly consumers.
pub fn initialize() {
    utils::set_panic_hook();
}
