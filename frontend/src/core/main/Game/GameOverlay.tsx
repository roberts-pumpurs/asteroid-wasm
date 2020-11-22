import React, { ReactElement } from 'react';
import { GameState } from './Game';

import style from './Game.module.scss';

interface Props {
  setActive: (arg0: boolean) => void;
  currentState: GameState;
}

export function GameOverlay({ setActive, currentState }: Props): ReactElement {
  let title = <></>;
  let content = <></>;
  switch (currentState) {
    case GameState.INITIALIZING:
      title = <>CONTROLS</>;
      content = (
        <>
          <div>
            W - Forwards
          </div>
          <div>
            S - Backwards
          </div>
          <div>
            A - Turn Left
          </div>
          <div>
            D - Turn Right
          </div>
          <div>
            SPACE - Shoot
          </div>

          <button
            className={`${style.btn} ${style['draw-border']}`}
            type="button"
            onClick={() => setActive(true)}
          > START
          </button>
        </>
      );
      break;
    case GameState.GAME_OVER:
      title = <>GAME OVER</>;
      content = (
        <>
          Data input
        </>
      );
      break;
    default:
      return <></>;
  }
  return (
    <div className={style.rules}>
      <h3>{title}</h3>
      {content}
    </div>
  );
}
