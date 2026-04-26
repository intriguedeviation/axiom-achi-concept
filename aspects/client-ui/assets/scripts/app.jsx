// App entry point
import React from 'react';
import {PlayerSide, definePlayer, initialize, startGame} from 'axiom-achi';
import { Header } from './header';
import { Board } from './board';

initialize();

export const App = () => {
  const [redPlayer, setRedPlayer] = React.useState(definePlayer(PlayerSide.Red, 'Human 1'));
  const [blackPlayer, setBlackPlayer] = React.useState(definePlayer(PlayerSide.Black, 'Human 2'));

  let achiGame = startGame(redPlayer, blackPlayer);

  return (
    <>
      <Header game={achiGame} red={redPlayer} black={blackPlayer} />
      <Board scale={395} />
    </>
  );
};