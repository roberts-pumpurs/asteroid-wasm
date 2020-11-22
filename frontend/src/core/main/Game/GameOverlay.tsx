import React, { ReactElement } from 'react';

import style from './Game.module.scss';

interface Props {
  setActive: (arg0: boolean) => void;
}

export function GameOverlay({ setActive }: Props): ReactElement {
  return (
    <div className={style.rules}>
      <h3>CONTROLS</h3>
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
      {/* <button type="button" onClick={() => setActive(true)}>
        START
      </button> */}
    </div>
  );
}
