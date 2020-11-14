import React, {
  ReactElement, useState, useEffect,
} from 'react';
import {
  GlClient, RenderableOption,
} from 'wasm-app';
import style from './Game.module.scss';

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
    if (options !== undefined) {
      client.set_renderable(+options, new wasm.Transform(x, y, z));
    }
  }, [client, options, wasm.Transform, x, y, z]);

  return client !== undefined && options !== undefined ? (
    <div className={style['sidebar-options']}>
      <h3>Displayable</h3>
      <div className={style.options}>
        <button
          className={`${style.btn} ${style['draw-border']}`}
          type="button"
          onClick={(_) => setOption(wasm.RenderableOption.Cube)}
        >
          Box3D
        </button>
        <button
          className={`${style.btn} ${style['draw-border']}`}
          type="button"
          onClick={(_) => setOption(wasm.RenderableOption.Box2D)}
        >
          Square2D
        </button>
        <button
          className={`${style.btn} ${style['draw-border']}`}
          type="button"
          onClick={(_) => setOption(wasm.RenderableOption.Asteroid)}
        >
          Asteroid
        </button>
      </div>
      <div className={style.variable}>
        <h3>Change Z</h3>
        <input
          className={`${style.input} ${style['draw-border']}`}
          type="number"
          // value={z}
          onChange={(e) => {
            if (!Number.isNaN(Number(e.target.value))) {
              setZ(Number(e.target.value));
            }
          }}
        />
      </div>
    </div>
  ) : (
    <div>Loading</div>
  );
}
