import React from 'react';
// import { Link } from "react-router-dom";
import style from './Footer.module.scss';

export function Footer(): React.ReactElement {
  return (
    <div className={style['footer-sub']}>
      <a href="https://www.linkedin.com/in/roberts-ivanovs-3b24b6159/">
        Made by Roberts Ivanovs
      </a>
      <a href="#" >
        <i className={`${'fab fa-github'} ${style.icon}`} />View project
      </a>
    </div>
  );
}
