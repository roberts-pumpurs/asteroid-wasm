import React, { ReactElement } from 'react';
import style from './About.module.scss';

interface BtnProps {
  value: number;
  setValue: (newNumber: number) => void;
  text: string;
}

export function PlusMinusButton({
  value,
  setValue,
  text,
}: BtnProps): ReactElement {
  return (
    <div className={style.variable}>
      <h4>{text} {value}</h4>
      <div>
        <button
          className={`${style.btn} ${style['draw-border']}`}
          type="button"
          onClick={() => {
            setValue(value + 1);
          }}
        > +
        </button>
        <button
          className={`${style.btn} ${style['draw-border']}`}
          type="button"
          onClick={() => {
            setValue(value - 1);
          }}
        > -
        </button>
      </div>
    </div>
  );
}
