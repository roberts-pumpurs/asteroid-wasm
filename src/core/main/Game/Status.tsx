import React, { ReactElement, useEffect } from 'react';
import { GlClient } from 'wasm-app';
import style from 'core/main/About/About.module.scss';
import styleG from './Game.module.scss';

interface Props {
  lives: number;
  score: number;
  seconds: number;
}

export function Status({
  lives,
  score,
  seconds,
}: Props): ReactElement {
  const maxLives = 3;
  const heartsEmpty = Array.apply(null, Array(maxLives - lives)).map((_, index) => <i className={`${'far fa-heart' + ' '}${styleG.heart}`} key={index} />);
  const heartsFull = Array.apply(null, Array(lives)).map((_, index) => <i className={`${'fas fa-heart' + ' '}${styleG.heart}`} key={index} />);
  return (
    <div className={style['sidebar-options']}>
      <h3>Status</h3>
      <div>
        Score: {score}
      </div>
      <div>
        seconds: {seconds}
      </div>
      <div>
        {heartsFull}{heartsEmpty}
      </div>
    </div>
  );
}
