import React from 'react';

import {
  Switch,
  Route,
} from 'react-router-dom';
import { WasmContextConsumer } from 'WasmContext';
import { About } from './About/About';
import { Game } from './Game/Game';
import { GameHistory } from './Statistics/GameHistory/GameHistory';
import { LeaderboardComponent } from './Statistics/Leaderboard/Leaderboard';
import { Statistics } from './Statistics/Statistics';

export function Main(): React.ReactElement {
  return (
    <Switch>
      <Route exact path="/">
        <WasmContextConsumer>
          {
            (context) => context && (
              <Game wasm={context.wasm} />
            )
          }
        </WasmContextConsumer>
      </Route>
      <Route exact path="/statistics" component={Statistics} />
      <Route exact path="/about">
        <WasmContextConsumer>
          {
            (context) => context && (
              <About wasm={context.wasm} />
            )
          }
        </WasmContextConsumer>
      </Route>
    </Switch>
  );
}
