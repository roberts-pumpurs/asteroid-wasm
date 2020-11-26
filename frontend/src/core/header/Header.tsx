import React from 'react';
import { push as Menu } from 'react-burger-menu';
import { Link } from 'react-router-dom';

import sass from './Header.module.scss';

/* inline-JS CSS for react-burger-menu  */
const styles = {
  bmBurgerButton: {
    position: 'fixed',
    width: '36px',
    height: '30px',
    left: '36px',
    top: '36px',
  },
  bmBurgerBars: {
    background: '#FFFFFF',
  },
  bmBurgerBarsHover: {
    background: '#a90000',
  },
  bmCrossButton: {
    height: '24px',
    width: '24px',
  },
  bmCross: {
    background: '#bdc3c7',
  },
  bmMenuWrap: {
    position: 'fixed',
    height: '100%',
  },
  bmMenu: {
    background: '#FFFFFF',
    padding: '2.5em 1.5em 0',
    fontSize: '1.15em',
  },
  bmMorphShape: {
    fill: '#373a47',
  },
  bmItemList: {
    color: '#b8b7ad',
    padding: '0.8em',
  },
  bmItem: {
    display: 'inline-block',
  },
  bmOverlay: {
    background: 'rgba(0, 0, 0, 0.3)',
  },
};
export function Header(): React.ReactElement {
  return (
    <Menu styles={styles} pageWrapId="main" outerContainerId="root">
      <nav>
        <span className={sass.logo}>
          <Link to="/">
            ATARI Asteroids
          </Link>
        </span>
        <ul className={sass['nav-links']}>
          <li className={sass['nav-item']}>
            <Link to="/">
              Home
            </Link>
          </li>
          <li className={sass['nav-item']}>
            <Link to="/statistics">
              Statistics
            </Link>
          </li>
          <li className={sass['nav-item']}>
            <Link to="/about">
              About
            </Link>
          </li>
        </ul>
      </nav>
    </Menu>
  );
}
