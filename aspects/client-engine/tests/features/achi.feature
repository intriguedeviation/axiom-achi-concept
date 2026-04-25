Feature: Achi domain behavior
  The client engine implements the specification-defined game behaviors.

  Scenario: Defining a player creates four unset tokens
    When I define the red player "Ada"
    Then the red player has side red
    And the red player has label "Ada"
    And the red player has 4 tokens
    And the red player has no positioned tokens

  Scenario: Empty player labels are rejected
    When I try to define the red player " "
    Then the domain error is "invalid_player_label"

  Scenario: Starting a game initializes a placement grid
    Given the red player "Red"
    And the black player "Black"
    When I start the game
    Then the grid phase is placement
    And the active player is red
    And the grid has 9 positions
    And the grid has 8 victory alignments

  Scenario: A player cannot start a game against itself
    Given the red player "Red"
    When I try to start a game with the red player as both adversaries
    Then the domain error is "adversary_same_player"

  Scenario: Placement records the move and changes turns
    Given the red player "Red"
    And the black player "Black"
    And a started game
    When red places token 0 at position 4
    Then red token 0 is at position 4
    And the grid has 1 placements
    And the active player is black

  Scenario: Placement rejects the wrong turn
    Given the red player "Red"
    And the black player "Black"
    And a started game
    When black tries to place token 0 at position 0
    Then the domain error is "wrong_player_turn"

  Scenario: Placement rejects a red occupied position
    Given the red player "Red"
    And the black player "Black"
    And a started game
    And red places token 0 at position 0
    When black tries to place token 0 at position 0
    Then the domain error is "illegal_red_token_placement"

  Scenario: Changing grid phase requires red tokens to be set
    Given the red player "Red"
    And the black player "Black"
    And a started game
    When I try to change the grid phase
    Then the domain error is "red_player_unset_tokens"

  Scenario: Changing grid phase requires black tokens to be set
    Given the red player "Red"
    And the black player "Black"
    And a started game
    And red places token 0 at position 0
    And black places token 0 at position 1
    And red places token 1 at position 2
    And black places token 1 at position 3
    And red places token 2 at position 4
    And black places token 2 at position 5
    And red places token 3 at position 6
    When I try to change the grid phase
    Then the domain error is "black_player_unset_tokens"

  Scenario: A filled placement grid changes to movement
    Given a filled game without victory
    When I change the grid phase
    Then the grid phase is movement

  Scenario: Movement rejects the same position
    Given a filled game without victory
    And the grid phase is changed
    When red tries to move token 0 to position 0
    Then the domain error is "same_position_not_allowed"

  Scenario: Movement rejects a black occupied position
    Given a filled game without victory
    And the grid phase is changed
    When red tries to move token 0 to position 1
    Then the domain error is "illegal_black_token_movement"

  Scenario: Movement rejects non-adjacent positions
    Given a filled game without victory
    And the grid phase is changed
    When red tries to move token 0 to position 5
    Then the domain error is "movement_not_adjacent"

  Scenario: Movement updates the token and changes turns
    Given a filled game without victory
    And the grid phase is changed
    When red moves token 2 to position 5
    Then red token 2 is at position 5
    And the active player is black

  Scenario: Victory requires all tokens to be set
    Given the red player "Red"
    And the black player "Black"
    And a started game
    When I check victory
    Then the domain error is "red_player_unset_tokens"

  Scenario: Victory returns the aligned player
    Given a game where red has a victory alignment
    When I check victory
    Then victory is achieved by red
