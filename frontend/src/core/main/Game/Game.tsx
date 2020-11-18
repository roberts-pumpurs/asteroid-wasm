import React, {
  ReactElement, useState, useEffect, useCallback,
} from 'react';
import { CanvasData, GlClient } from 'wasm-app';
import style from './Game.module.scss';
import { RenderableDropdown } from './RenderableDropdown';

interface Props {
  wasm: typeof import('wasm-app');
}

const FPS_THROTTLE = 1000 / 144; // 144 fps

export function Game({ wasm }: Props): ReactElement {
  const [canvas, setCanvas] = useState<CanvasData>();
  const [client, setClient] = useState<GlClient>();
  const [width, setWidth] = useState(1000);
  const [degrees, setDegrees] = useState(45);
  const [height, setHeight] = useState(600);
  const canvasId = 'canvasRust';
  const [rectEl, setRectEl] = useState<DOMRect>();

  useEffect(() => {
    setCanvas(new wasm.CanvasData(width, height, degrees, canvasId));
  }, [width, height, canvasId, wasm.CanvasData, degrees]);

  // Use useRef for mutable variables that we want to persist
  // without triggering a re-render on their change
  const requestRef = React.useRef<number>();
  const previousTimeRef = React.useRef<number>();

  const animate = useCallback(
    (time: number) => {
      if (
        previousTimeRef.current !== undefined
        && time - previousTimeRef.current > FPS_THROTTLE
      ) {
        client?.render();
      }
      client?.update(time - (previousTimeRef.current || 0));
      previousTimeRef.current = time;
      requestRef.current = requestAnimationFrame(animate);
    },
    [client],
  );

  useEffect(() => {
    requestRef.current = requestAnimationFrame(animate);
    return () => cancelAnimationFrame(requestRef.current!);
  }, [animate]); // Make sure the effect runs only once

  useEffect(() => {
    if (canvas !== undefined) {
      const tmpClient = new wasm.GlClient(canvas);
      setClient(tmpClient);
    }
  }, [canvas, wasm.GlClient]);

  useEffect(() => {
    setRectEl(document.getElementById(canvasId)?.getBoundingClientRect());
  }, []);

  /* Register canvas events to be captured by the WASM module */
  const mouseDown = useCallback(
    (e: React.MouseEvent<HTMLCanvasElement, MouseEvent>) => {
      client?.update_mouse_down(e.clientX, e.clientY, true);
    },
    [client],
  );
  const mouseUp = useCallback(
    (e: React.MouseEvent<HTMLCanvasElement, MouseEvent>) => {
      client?.update_mouse_down(e.clientX, e.clientY, false);
    },
    [client],
  );
  const mouseMove = useCallback(
    (e: React.MouseEvent<HTMLCanvasElement, MouseEvent>) => {
      if (rectEl !== undefined) {
        client?.update_mouse_position(
          ((e.clientX - rectEl.left) / (rectEl.right - rectEl.left)) * width,
          ((e.clientY - rectEl.top) / (rectEl.bottom - rectEl.top)) * height,
        );
      }
    },
    [client, height, rectEl, width],
  );
  const keyDown = useCallback(
    (e: React.KeyboardEvent<HTMLCanvasElement>) => {

      if (rectEl !== undefined) {
          if (e.key === 'a') {
            client?.keyboard_a(true);
          } else if (e.key === 'd') {
            client?.keyboard_d(true);
          } else if (e.key === 'w') {
            client?.keyboard_w(true);
          } else if (e.key === 's') {
            client?.keyboard_s(true);
          } else if (e.key === 'Space') {
            client?.keyboard_space(true);
          }
      }
    },
    [client, rectEl],
  );
  const keyUp = useCallback(
    (e: React.KeyboardEvent<HTMLCanvasElement>) => {
      if (rectEl !== undefined) {
        if (e.key === 'a') {
          client?.keyboard_a(false);
        } else if (e.key === 'd') {
          client?.keyboard_d(false);
        } else if (e.key === 'w') {
          client?.keyboard_w(false);
        } else if (e.key === 's') {
          client?.keyboard_s(false);
        } else if (e.key === 'Space') {
          client?.keyboard_space(false);
        }
      }
    },
    [client, rectEl],
  );

  return (
    <div className={style['game-wrapper']}>
      <h1>WebAssembly Test samples programmed in Rust</h1>
      <div className={style['canvas-and-options']}>
        {/* Create the canvas that will be used for rendering stuff */}
        <canvas
          tabIndex={0}
          id={canvasId}
          height={height}
          width={width}
          onMouseDown={mouseDown}
          onMouseUp={mouseUp}
          onMouseMove={mouseMove}
          onKeyDown={keyDown}
          onKeyUp={keyUp}
        />
        {wasm && canvas && client && (
          <RenderableDropdown client={client} wasm={wasm} />
        )}
      </div>
    </div>
  );
}
