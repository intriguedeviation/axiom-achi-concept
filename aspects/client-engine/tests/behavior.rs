use achi::{
    define_player_domain, start_game_domain, BoardPhase, DomainError, Grid, Player, PlayerSide,
    Victory,
};
use cucumber::{given, then, when, World};

#[derive(Debug, Default, World)]
struct AchiWorld {
    red_player: Option<Player>,
    black_player: Option<Player>,
    grid: Option<Grid>,
    victory: Option<Victory>,
    last_error: Option<DomainError>,
}

impl AchiWorld {
    fn player(&self, side: PlayerSide) -> &Player {
        match side {
            PlayerSide::Red => self.red_player.as_ref().expect("red player exists"),
            PlayerSide::Black => self.black_player.as_ref().expect("black player exists"),
        }
    }

    fn grid(&self) -> &Grid {
        self.grid.as_ref().expect("grid exists")
    }

    fn grid_mut(&mut self) -> &mut Grid {
        self.grid.as_mut().expect("grid exists")
    }

    fn define_player(&mut self, side: PlayerSide, label: &str) -> Result<(), DomainError> {
        let player = define_player_domain(side, label)?;
        match side {
            PlayerSide::Red => self.red_player = Some(player),
            PlayerSide::Black => self.black_player = Some(player),
        }
        Ok(())
    }

    fn start_game(&mut self) -> Result<(), DomainError> {
        let grid = start_game_domain(self.player(PlayerSide::Red), self.player(PlayerSide::Black))?;
        self.grid = Some(grid);
        Ok(())
    }

    fn place_token(
        &mut self,
        side: PlayerSide,
        token_index: usize,
        position: u8,
    ) -> Result<(), DomainError> {
        let player = self.player(side).clone();
        self.grid_mut()
            .place_token(&player, token_index, position)
            .map(|_| ())
    }

    fn move_token(
        &mut self,
        side: PlayerSide,
        token_index: usize,
        position: u8,
    ) -> Result<(), DomainError> {
        let player = self.player(side).clone();
        self.grid_mut()
            .move_token(&player, token_index, position)
            .map(|_| ())
    }

    fn set_error(&mut self, result: Result<(), DomainError>) {
        self.last_error = result.err();
    }
}

#[when(regex = r#"^I define the (red|black) player "([^"]*)"$"#)]
fn i_define_player(world: &mut AchiWorld, side: String, label: String) {
    world
        .define_player(parse_side(&side), &label)
        .expect("player can be defined");
}

#[when(regex = r#"^I try to define the (red|black) player "([^"]*)"$"#)]
fn i_try_to_define_player(world: &mut AchiWorld, side: String, label: String) {
    let result = world.define_player(parse_side(&side), &label);
    world.set_error(result);
}

#[given(regex = r#"^the (red|black) player "([^"]*)"$"#)]
fn given_player(world: &mut AchiWorld, side: String, label: String) {
    i_define_player(world, side, label);
}

#[then(regex = r#"^the (red|black) player has side (red|black)$"#)]
fn player_has_side(world: &mut AchiWorld, player_side: String, expected_side: String) {
    assert_eq!(
        world.player(parse_side(&player_side)).side_value(),
        parse_side(&expected_side)
    );
}

#[then(regex = r#"^the (red|black) player has label "([^"]*)"$"#)]
fn player_has_label(world: &mut AchiWorld, side: String, label: String) {
    assert_eq!(world.player(parse_side(&side)).label_value(), label);
}

#[then(regex = r#"^the (red|black) player has (\d+) tokens$"#)]
fn player_has_token_count(world: &mut AchiWorld, side: String, count: usize) {
    assert_eq!(world.player(parse_side(&side)).token_count(), count);
}

#[then(regex = r#"^the (red|black) player has no positioned tokens$"#)]
fn player_has_no_positioned_tokens(world: &mut AchiWorld, side: String) {
    let player = world.player(parse_side(&side));
    for index in 0..player.token_count() {
        assert_eq!(player.token_position_js(index).unwrap(), None);
    }
}

#[when("I start the game")]
fn i_start_the_game(world: &mut AchiWorld) {
    world.start_game().expect("game can be started");
}

#[given("a started game")]
fn given_started_game(world: &mut AchiWorld) {
    i_start_the_game(world);
}

#[when("I try to start a game with the red player as both adversaries")]
fn try_start_game_with_same_player(world: &mut AchiWorld) {
    let red = world.player(PlayerSide::Red);
    world.last_error = start_game_domain(red, red).err();
}

#[then(regex = r#"^the grid phase is (placement|movement)$"#)]
fn grid_phase_is(world: &mut AchiWorld, phase: String) {
    assert_eq!(world.grid().phase_value(), parse_phase(&phase));
}

#[then(regex = r#"^the active player is (red|black)$"#)]
fn active_player_is(world: &mut AchiWorld, side: String) {
    assert_eq!(world.grid().active_player_value(), parse_side(&side));
}

#[then(regex = r#"^the grid has (\d+) positions$"#)]
fn grid_has_position_count(world: &mut AchiWorld, count: usize) {
    assert_eq!(world.grid().position_count(), count);
}

#[then(regex = r#"^the grid has (\d+) victory alignments$"#)]
fn grid_has_victory_alignment_count(world: &mut AchiWorld, count: usize) {
    assert_eq!(world.grid().victory_alignment_count(), count);
}

#[then(regex = r#"^the grid has (\d+) placements$"#)]
fn grid_has_placement_count(world: &mut AchiWorld, count: usize) {
    assert_eq!(world.grid().placement_count(), count);
}

#[given(regex = r#"^(red|black) places token (\d+) at position (\d+)$"#)]
#[when(regex = r#"^(red|black) places token (\d+) at position (\d+)$"#)]
fn place_token(world: &mut AchiWorld, side: String, token_index: usize, position: u8) {
    world
        .place_token(parse_side(&side), token_index, position)
        .expect("token can be placed");
}

#[when(regex = r#"^(red|black) tries to place token (\d+) at position (\d+)$"#)]
fn try_place_token(world: &mut AchiWorld, side: String, token_index: usize, position: u8) {
    let result = world.place_token(parse_side(&side), token_index, position);
    world.set_error(result);
}

#[when("I try to change the grid phase")]
fn try_change_grid_phase(world: &mut AchiWorld) {
    world.last_error = world.grid_mut().change_grid_phase().err();
}

#[when("I change the grid phase")]
#[given("the grid phase is changed")]
fn change_grid_phase(world: &mut AchiWorld) {
    world
        .grid_mut()
        .change_grid_phase()
        .expect("grid phase can be changed");
}

#[given("a filled game without victory")]
fn filled_game_without_victory(world: &mut AchiWorld) {
    given_player(world, "red".to_owned(), "Red".to_owned());
    given_player(world, "black".to_owned(), "Black".to_owned());
    given_started_game(world);

    place_token(world, "red".to_owned(), 0, 0);
    place_token(world, "black".to_owned(), 0, 1);
    place_token(world, "red".to_owned(), 1, 2);
    place_token(world, "black".to_owned(), 1, 3);
    place_token(world, "red".to_owned(), 2, 4);
    place_token(world, "black".to_owned(), 2, 6);
    place_token(world, "red".to_owned(), 3, 7);
    place_token(world, "black".to_owned(), 3, 8);
}

#[when(regex = r#"^(red|black) tries to move token (\d+) to position (\d+)$"#)]
fn try_move_token(world: &mut AchiWorld, side: String, token_index: usize, position: u8) {
    let result = world.move_token(parse_side(&side), token_index, position);
    world.set_error(result);
}

#[when(regex = r#"^(red|black) moves token (\d+) to position (\d+)$"#)]
fn move_token(world: &mut AchiWorld, side: String, token_index: usize, position: u8) {
    world
        .move_token(parse_side(&side), token_index, position)
        .expect("token can be moved");
}

#[then(regex = r#"^(red|black) token (\d+) is at position (\d+)$"#)]
fn token_is_at_position(world: &mut AchiWorld, side: String, token_index: usize, position: u8) {
    let actual = match parse_side(&side) {
        PlayerSide::Red => world.grid().red_token_position(token_index).unwrap(),
        PlayerSide::Black => world.grid().black_token_position(token_index).unwrap(),
    };
    assert_eq!(actual, Some(position));
}

#[when("I check victory")]
fn check_victory(world: &mut AchiWorld) {
    match world.grid().victory_achieved() {
        Ok(victory) => world.victory = Some(victory),
        Err(error) => world.last_error = Some(error),
    }
}

#[given("a game where red has a victory alignment")]
fn game_where_red_has_victory_alignment(world: &mut AchiWorld) {
    given_player(world, "red".to_owned(), "Red".to_owned());
    given_player(world, "black".to_owned(), "Black".to_owned());
    given_started_game(world);

    place_token(world, "red".to_owned(), 0, 0);
    place_token(world, "black".to_owned(), 0, 3);
    place_token(world, "red".to_owned(), 1, 1);
    place_token(world, "black".to_owned(), 1, 4);
    place_token(world, "red".to_owned(), 2, 2);
    place_token(world, "black".to_owned(), 2, 6);
    place_token(world, "red".to_owned(), 3, 5);
    place_token(world, "black".to_owned(), 3, 7);
}

#[then(regex = r#"^victory is achieved by (red|black)$"#)]
fn victory_is_achieved_by(world: &mut AchiWorld, side: String) {
    let victory = world.victory.as_ref().expect("victory result exists");
    assert!(victory.result);
    assert_eq!(victory.player, Some(parse_side(&side)));
}

#[then(regex = r#"^the domain error is "([^"]+)"$"#)]
fn domain_error_is(world: &mut AchiWorld, error_code: String) {
    assert_eq!(
        world.last_error.expect("domain error exists").code(),
        error_code
    );
}

fn parse_side(value: &str) -> PlayerSide {
    match value {
        "red" => PlayerSide::Red,
        "black" => PlayerSide::Black,
        _ => panic!("unknown player side: {}", value),
    }
}

fn parse_phase(value: &str) -> BoardPhase {
    match value {
        "placement" => BoardPhase::Placement,
        "movement" => BoardPhase::Movement,
        _ => panic!("unknown board phase: {}", value),
    }
}

fn main() {
    futures::executor::block_on(AchiWorld::run("tests/features"));
}
