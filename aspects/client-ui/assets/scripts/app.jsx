// App entry point
import React from 'react';
import {PlayerSide, definePlayer, initialize, startGame} from 'axiom-achi';
import { Header } from './header';
import { Board } from './board';
import { NebulaBackground } from './nebula-background';

initialize();

export const App = () => {
  const [redPlayer, setRedPlayer] = React.useState(definePlayer(PlayerSide.Red, 'Sage'));
  const [blackPlayer, setBlackPlayer] = React.useState(definePlayer(PlayerSide.Black, 'Deseginak'));

  let achiGame = startGame(redPlayer, blackPlayer);

  return (
    <div className="app-shell">
      <NebulaBackground />
      <div className="app-shell__content">
        <Header game={achiGame} red={redPlayer} black={blackPlayer} />
        <Board scale={395} />
      </div>
    </div>
  );
};
