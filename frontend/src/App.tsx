import React, { ReactElement } from 'react';

import { Header } from 'core/header/Header';
import { Main } from 'core/main/Main';
import { Footer } from 'core/footer/Footer';
import 'utils/_base.scss';

function App(): ReactElement {
  return (
    <>
      <header>
        <Header />
      </header>
      <main>
        <Main />
      </main>
      <footer>
        <Footer />
      </footer>
    </>
  );
}

export { App };
