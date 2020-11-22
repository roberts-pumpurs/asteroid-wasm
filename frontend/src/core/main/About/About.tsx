import React, {
  ReactElement, useState, useEffect, useCallback, useRef,
} from 'react';
import { CanvasData, GlClient } from 'wasm-app';
import style from './About.module.scss';
import { RenderableDropdown } from './RenderableDropdown';

interface Props {
  wasm: typeof import('wasm-app');
}

const FPS_THROTTLE = 1000 / 144; // 144 fps

export function About({ wasm }: Props): ReactElement {
  const [canvas, setCanvas] = useState<CanvasData>();
  const [client, setClient] = useState<GlClient>();
  const [width] = useState(1000);
  const [degrees] = useState(45);
  const [height] = useState(600);
  const canvasId = 'canvasRust';
  const [rectEl, setRectEl] = useState<DOMRect>();

  useEffect(() => {
    if (wasm) setCanvas(new wasm.CanvasData(width, height, degrees, canvasId));
  }, [width, height, canvasId, wasm.CanvasData, degrees, wasm]);

  // Use useRef for mutable variables that we want to persist
  // without triggering a re-render on their change
  const requestRef = useRef<number>();
  const previousTimeRef = useRef<number>();

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

  return (
    <div className={style['game-wrapper']}>
      <h1>Rust/WASM/WebGL Samples</h1>
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
        />
        {wasm && canvas && client && (
          <RenderableDropdown client={client} wasm={wasm} />
        )}
      </div>
    </div>
  );
}
