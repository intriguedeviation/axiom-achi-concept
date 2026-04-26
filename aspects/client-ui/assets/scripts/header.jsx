import { BoardPhase, PlayerSide } from 'axiom-achi';

export const Header = ({red, black, game, onReset}) => {

  const currentPhase = game.phase === BoardPhase.Placement ? 'Placement' : 'Movement';
  
  let redClassName = 'header__billboard-red';
  if (game.activePlayer() === PlayerSide.Red) {
    redClassName += ' active';
  }

  let blackClassName = 'header__billboard-black';
  if (game.activePlayer() === PlayerSide.Black) {
    blackClassName += ' active';
  }

  return (
    <header className="header">
      <a href="/" className="header__title">Achi Experiment</a>

      <nav className="header__billboard">
        <span className="header__billboard-phase">{currentPhase}</span>
        <span className={redClassName}>{red.label}</span>
        <span className={blackClassName}>{black.label}</span>
        <button className="header__billboard-reset" onClick={onReset}>Reset</button>
      </nav>
    </header>
  );
};
