import React from 'react';

import {
  Switch,
  Route,
} from 'react-router-dom';
import { WasmContextConsumer } from 'WasmContext';
import { Game } from './Game/Game';
import { GameHistory } from './GameHistory/GameHistory';

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
      <Route exact path="/game-history" component={GameHistory} />
    </Switch>
  );
}
