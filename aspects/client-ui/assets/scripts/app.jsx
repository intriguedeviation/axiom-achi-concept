// App entry point
import React from 'react';
import {BoardPhase, PlayerSide, definePlayer, initialize, startGame} from 'axiom-achi';
import { Header } from './header';
import { Board, boardMetadata } from './board';
import { NebulaBackground } from './nebula-background';

initialize();

const TOKEN_COUNT_PER_PLAYER = 4;
const NO_WINNER = -1;

const createGameState = () => {
  const redPlayer = definePlayer(PlayerSide.Red, 'Sage');
  const blackPlayer = definePlayer(PlayerSide.Black, 'Deseginak');

  return {
    redPlayer,
    blackPlayer,
    game: startGame(redPlayer, blackPlayer),
  };
};

const sideName = (side) => {
  if (side === PlayerSide.Red) {
    return 'red';
  }
  if (side === PlayerSide.Black) {
    return 'black';
  }

  return 'no player';
};

const formatDomainError = (error) => (
  String(error).replaceAll('_', ' ')
);

// Reads token positions from the domain grid snapshot owned by the game.
const tokenPositionsFor = (game, side) => (
  Array.from({length: TOKEN_COUNT_PER_PLAYER}, (_, tokenIndex) => ({
    side,
    tokenIndex,
    position: side === PlayerSide.Red
      ? game.redTokenPosition(tokenIndex)
      : game.blackTokenPosition(tokenIndex),
  }))
);

const positionOccupant = (tokens, position) => (
  tokens.find((token) => token.position === position)
);

const nextUnsetTokenIndex = (tokens, side) => (
  tokens.find((token) => token.side === side && token.position === undefined)?.tokenIndex
);

const readVictory = (game) => {
  const victory = game.victoryAchieved();
  const result = {
    achieved: victory.result,
    player: victory.player,
  };
  victory.free?.();

  return result;
};

export const App = () => {
  const [{redPlayer, blackPlayer, game}, setGameState] = React.useState(createGameState);
  const [selectedToken, setSelectedToken] = React.useState(null);
  const [revision, setRevision] = React.useState(0);
  const [status, setStatus] = React.useState('Place a red token.');
  const [victoryState, setVictoryState] = React.useState({achieved: false, player: NO_WINNER});

  const tokens = React.useMemo(
    () => [
      ...tokenPositionsFor(game, PlayerSide.Red),
      ...tokenPositionsFor(game, PlayerSide.Black),
    ],
    [game, revision],
  );

  const refreshAfterDomainChange = React.useCallback((message) => {
    let nextMessage = message;

    const allTokensPositioned = game.placementCount() >= boardMetadata.positionCount - 1;

    if (allTokensPositioned && game.phase === BoardPhase.Placement) {
      game.changeGridPhase();
      nextMessage = 'Movement phase. Select a token to move.';
    }

    if (allTokensPositioned) {
      const victory = readVictory(game);
      if (victory.achieved) {
        nextMessage = `${sideName(victory.player)} wins.`;
      }
      setVictoryState(victory);
    }

    setStatus(nextMessage);
    setRevision((value) => value + 1);
  }, [game]);

  const handlePositionAction = React.useCallback((position) => {
    if (victoryState.achieved) {
      setStatus(`${sideName(victoryState.player)} has already won.`);
      return;
    }

    const activeSide = game.activePlayer();
    const activePlayer = activeSide === PlayerSide.Red ? redPlayer : blackPlayer;
    const currentTokens = [
      ...tokenPositionsFor(game, PlayerSide.Red),
      ...tokenPositionsFor(game, PlayerSide.Black),
    ];
    const occupant = positionOccupant(currentTokens, position);

    try {
      if (game.phase === BoardPhase.Placement) {
        const tokenIndex = nextUnsetTokenIndex(currentTokens, activeSide);
        if (tokenIndex === undefined) {
          setStatus(`${sideName(activeSide)} has no unset tokens.`);
          return;
        }
        const placement = game.placeToken(activePlayer, tokenIndex, position);
        placement.free?.();
        setSelectedToken(null);
        refreshAfterDomainChange(`${sideName(game.activePlayer())} to place.`);
        return;
      }

      if (!selectedToken) {
        if (occupant?.side === activeSide) {
          setSelectedToken(occupant);
          setStatus(`Selected ${sideName(activeSide)} token ${occupant.tokenIndex + 1}.`);
        } else {
          setStatus(`Select a ${sideName(activeSide)} token to move.`);
        }
        return;
      }

      if (occupant?.side === activeSide && occupant.tokenIndex !== selectedToken.tokenIndex) {
        setSelectedToken(occupant);
        setStatus(`Selected ${sideName(activeSide)} token ${occupant.tokenIndex + 1}.`);
        return;
      }

      const movement = game.moveToken(activePlayer, selectedToken.tokenIndex, position);
      movement.free?.();
      setSelectedToken(null);
      refreshAfterDomainChange(`${sideName(game.activePlayer())} to move.`);
    } catch (error) {
      setStatus(formatDomainError(error));
      setRevision((value) => value + 1);
    }
  }, [
    blackPlayer,
    game,
    redPlayer,
    refreshAfterDomainChange,
    selectedToken,
    victoryState,
  ]);

  const resetGame = React.useCallback(() => {
    setGameState(createGameState());
    setSelectedToken(null);
    setRevision(0);
    setStatus('Place a red token.');
    setVictoryState({achieved: false, player: NO_WINNER});
  }, []);

  return (
    <div className="app-shell">
      <NebulaBackground />
      <div className="app-shell__content">
        <Header game={game} red={redPlayer} black={blackPlayer} onReset={resetGame} />
        <Board
          activeSide={game.activePlayer()}
          onPositionAction={handlePositionAction}
          phase={game.phase}
          scale={395}
          selectedToken={selectedToken}
          tokens={tokens}
          victory={victoryState}
        />
        <p className="game__status" aria-live="polite">{status}</p>
      </div>
    </div>
  );
};
