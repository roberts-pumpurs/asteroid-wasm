import React from 'react';

import {
  Switch,
  Route,
} from 'react-router-dom';
import { WasmContextConsumer } from 'WasmContext';
import { About } from './About/About';
import { Game } from './Game/Game';
import { GameHistory } from './GameHistory/GameHistory';
import { LeaderboardComponent } from './Leaderboard/Leaderboard';

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
      <Route exact path="/leaderboards" component={LeaderboardComponent} />
      <Route exact path="/game-history" component={GameHistory} />
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
