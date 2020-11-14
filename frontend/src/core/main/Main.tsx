import React from 'react';

import {
  Switch,
  Route,
} from 'react-router-dom';
import { WasmContextConsumer } from 'WasmContext';
import { Game } from './Game/Game';

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
    </Switch>
  );
}
