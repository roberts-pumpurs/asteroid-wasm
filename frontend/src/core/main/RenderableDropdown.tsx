import React, {
  ReactElement, useState, useEffect, useCallback,
} from 'react';
import {
  GlClient, RenderableOption, CanvasData, Transform,
} from 'wasm-app';

interface Props {
  client: GlClient;
  wasm: typeof import('wasm-app');
}

interface Current {
  current: number;
}

export function RenderableDropdown({
  client,
  wasm,
}: Props): ReactElement {
  const [options, setOption] = useState<RenderableOption>();
  const [x, setX] = useState(0);
  const [y, setY] = useState(0);
  const [z, setZ] = useState(-6);

  useEffect(() => {
    setOption(+wasm.RenderableOption.Asteroid);
  }, []);

  useEffect(() => {
    if (options !== undefined) {
      client.set_renderable(+options, new wasm.Transform(x, y, z));
    }
  }, [client, options, wasm.Transform, x, y, z]);

  return client !== undefined && options !== undefined ? (
    <div>
      <select
        value={Object.keys(wasm.RenderableOption)[options]}
        name="renderable"
        onChange={(e: React.ChangeEvent<HTMLSelectElement>) => {
          const val = wasm.RenderableOption[(e.target.value as unknown) as number];
          setOption(Number(val));
        }}
      >
        {Object.keys(wasm.RenderableOption).map((el) => (
          <option key={el} value={el}>
            {el}
          </option>
        ))}
      </select>
      <div>
        <div>Change X</div>
        <input
          type="number"
          value={x}
          onChange={(e) => setX(Number(e.target.value))}
        />
      </div>
      <div>
        <div>Change Y</div>
        <input
          type="number"
          value={y}
          onChange={(e) => setY(Number(e.target.value))}
        />
      </div>
      <div>
        <div>Change Z</div>
        <input
          type="number"
          value={z}
          onChange={(e) => setZ(Number(e.target.value))}
        />
      </div>
    </div>
  ) : (
    <div>Loading</div>
  );
}
