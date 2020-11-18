import React from 'react';
import { Link } from 'react-router-dom';

import style from './Header.module.scss';

export function Header(): React.ReactElement {
  return (
    <nav>
      <Link to="/" className={style.logo}> LOGO</Link>
      <ul className={style['nav-links']}>
        <li className={style['nav-item']}>
          <Link to="/">
            Home
          </Link>
        </li>
        <li className={style['nav-item']}>
          <Link to="/leaderboards">
            Leaderboards
          </Link>
        </li>
        <li className={style['nav-item']}>
          <Link to="/game-history">
            Game History
          </Link>
        </li>
        <li className={style['nav-item']}>
          <Link to="/about">
            About
          </Link>
        </li>
      </ul>
    </nav>
  );
}
