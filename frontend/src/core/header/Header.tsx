import React from 'react';
import { Link } from 'react-router-dom';

import style from './Header.module.scss';

export function Header(): React.ReactElement {
  return (
    <nav>
      <Link to="/" className={style.logo} />
      <ul className={style['nav-links']}>
        <li className={style['nav-item']}>
          <Link to="/">
            Home
          </Link>
        </li>
        <li className={`${style['nav-item']} ${style['nav-item-colored']}`}>
          <Link to="/game">
            Game
          </Link>
        </li>
      </ul>
    </nav>
  );
}
