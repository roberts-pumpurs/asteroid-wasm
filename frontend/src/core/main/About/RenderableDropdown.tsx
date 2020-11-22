import React, {
  ReactElement, useState, useEffect,
} from 'react';
import {
  GlClient, RenderableOption,
} from 'wasm-app';
import style from './About.module.scss';

interface Props {
  client: GlClient;
  wasm: typeof import('wasm-app');
}

export function RenderableDropdown({
  client,
  wasm,
}: Props): ReactElement {
  const [options, setOption] = useState<RenderableOption>(wasm.RenderableOption.Cube);
  const [x, setX] = useState(0);
  const [y, setY] = useState(0);
  const [z, setZ] = useState(-6);

  useEffect(() => {
    client.set_renderable(options, new wasm.Transform(x, y, z));
  }, [options, client, wasm.Transform]);

  useEffect(() => {
    client.set_transform(new wasm.Transform(x, y, z));
  }, [x, y, z]);

  return client !== undefined && options !== undefined ? (
    <div className={style['sidebar-options']}>
      <h3>Displayable</h3>
      <div className={style.options}>
        <button
          className={`${style.btn} ${style['draw-border']}`}
          type="button"
          onClick={() => setOption(wasm.RenderableOption.Cube)}
        >
          Box3D
        </button>
        <button
          className={`${style.btn} ${style['draw-border']}`}
          type="button"
          onClick={() => setOption(wasm.RenderableOption.Box2D)}
        >
          Square2D
        </button>
      </div>
      <PlusMinusButton value={z} setValue={setZ} text="Z" />
      <PlusMinusButton value={x} setValue={setX} text="X" />
      <PlusMinusButton value={y} setValue={setY} text="Y" />
    </div>
  ) : (
    <div>Loading</div>
  );
}

interface BtnProps {
  value: number;
  setValue: (newNumber: number) => void;
  text: string;
}

function PlusMinusButton({
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
