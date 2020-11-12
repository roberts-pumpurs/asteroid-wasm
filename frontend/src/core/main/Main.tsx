import React from 'react';

import {
  Switch,
  Route,
} from 'react-router-dom';
export function Main(): React.ReactElement {
  return (
    <Switch>
      <Route exact path="/">
        <div></div>
      </Route>
    </Switch>
  );
}
