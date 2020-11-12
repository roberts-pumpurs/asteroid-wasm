import React, { ReactElement, useState, useEffect, useCallback } from 'react';
import { GlClient, RenderableOption, CanvasData, Transform } from 'wasm-app';

interface Props {
  client: GlClient;
  wasm: typeof import('wasm-app');
}

interface Current {
  current: number;
}

export default function RenderableDropdown({
  client,
  wasm,
}: Props): ReactElement {
  const [options, setOption] = useState<RenderableOption>();
  const [x, setX] = useState(0);
  const [y, setY] = useState(0);
  const [z, setZ] = useState(-6);

  useEffect(() => {
    setOption(+wasm.RenderableOption.Cube);
  }, []);

  useEffect(() => {
    if (options !== undefined) {
      client.set_renderable(+options, new wasm.Transform(x,y,z));
    };
  }, [options]);

  return client !== undefined && options !== undefined ? (
    <div>
      <select
        className="custom-select custom-select-lg mb-3"
        value={Object.keys(wasm.RenderableOption)[options]}
        name="renderable"
        onChange={(e: React.ChangeEvent<HTMLSelectElement>) => {
          const val =
            wasm.RenderableOption[(e.target.value as unknown) as number];
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
          className="form-control"
          type="number"
          value={x}
          onChange={(e) => setX(Number(e.target.value))}
        />
      </div>
      <div>
        <div>Change Y</div>
        <input
          className="form-control"
          type="number"
          value={y}
          onChange={(e) => setY(Number(e.target.value))}
        />
      </div>
      <div>
        <div>Change Z</div>
        <input
          className="form-control"
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
