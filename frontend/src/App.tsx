import React, { ReactElement } from 'react';

import { Header } from 'core/header/Header';
import { Main } from 'core/main/Main';
import { Footer } from 'core/footer/Footer';
import 'utils/_base.scss';
import style from 'utils/stars.module.scss';

function App(): ReactElement {
  return (
    <>
      {/* <header> */}
        <Header />
      {/* </header> */}
      <main id="main">
        <section id="content">
          <Main />
          {/* Spawn stars */}
          {Array.apply(null, Array(100)).map(
            (_, index) => <div key={index} className={style.star} />,
            )}
        </section>
        <footer>
          <Footer />
        </footer>
      </main>
    </>
  );
}

export { App };
