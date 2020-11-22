import React, { ReactElement, useEffect } from 'react';
import { GlClient } from 'wasm-app';
import style from 'core/main/About/About.module.scss';

interface Props {
  client: GlClient;
  wasm: typeof import('wasm-app');
  lives: number;
  score: number;
  seconds: number;
}

export function Status({
  client,
  wasm,
  lives,
  score,
  seconds,
}: Props): ReactElement {
  return (
    <div className={style['sidebar-options']}>
      <h3>Status</h3>
      <div>
        Score: {score}
      </div>
      <div>
        lives: {lives}
      </div>
      <div>
        seconds: {seconds}
      </div>
    </div>
  );
}
