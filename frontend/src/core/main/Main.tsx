import React from 'react';

import {
  Switch,
  Route,
  useLocation,
} from 'react-router-dom';
import { WasmContextConsumer } from 'WasmContext';
import { animated, useTransition } from 'react-spring';
import { About } from './About/About';
import { Game } from './Game/Game';
import { Statistics } from './Statistics/Statistics';
import { Players } from './Players/Players';

export function Main(): any {
  const location = useLocation();
  const transitions = useTransition(location, (loc) => loc.pathname, {
    from: { opacity: 1, transform: 'scale(0.5) translateY(-50%)', position: 'relative' },
    enter: { opacity: 1, transform: 'scale(1) translateY(0)', position: 'relative' },
    leave: { marginLeft: '10rem', opacity: 0, transform: 'scale(0) translateY(50%)', position: 'absolute' },
  });
  return transitions.map(({ item: l, props, key }) => (
    <animated.div key={key} style={props}>
      <Switch location={l}>
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
        <Route exact path="/players" component={Players} />
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
    </animated.div>
  ));
}
