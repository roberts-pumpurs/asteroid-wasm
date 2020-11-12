import React from 'react';
import style from './Footer.module.scss';

export function Footer(): React.ReactElement {
  return (
    <footer>
      <a href="#" className={style.logo} />
      <ul className={style['nav-links-center']}>
        <li className={style['nav-item']}><a href="#">&#169; Lorem ipsum</a></li>
        <li className={style['nav-item']}><a href="#">Lorem ipsum</a></li>
        <li className={style['nav-item']}><a href="#">Consectetur</a></li>
      </ul>
      <ul className={style['nav-links-right']}>
        <li className={style['nav-item']}><i className="fab fa-facebook" /></li>
        <li className={style['nav-item']}><i className="fab fa-instagram" /></li>
      </ul>
    </footer>
  );
}
