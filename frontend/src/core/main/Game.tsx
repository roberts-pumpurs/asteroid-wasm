import React, {
  ReactElement, useState, useEffect, useRef, useCallback,
} from 'react';
import { CanvasData, GlClient, Transform } from 'wasm-app';
import RenderableDropdown from './RenderableDropdown';

interface Props {
  wasm: typeof import('wasm-app');
  // memory: WebAssembly.Memory,
}

const FPS_THROTTLE = 1000 / 144; // 60fps

export function Game({ wasm }: Props): ReactElement {
  const [canvas, setCanvas] = useState<CanvasData>();
  const [client, setClient] = useState<GlClient>();
  const [width, setWidth] = useState(1000);
  const [degrees, setDegrees] = useState(45);
  const [height, setHeight] = useState(1000);
  const canvasId = 'canvasRust';
  const [rectEl, setRectEl] = useState<DOMRect>();

  useEffect(() => {
    setCanvas(new wasm.CanvasData(width, height, degrees, canvasId));
  }, [width, height, canvasId]);

  // Use useRef for mutable variables that we want to persist
  // without triggering a re-render on their change
  const requestRef = React.useRef<number>();
  const previousTimeRef = React.useRef<number>();

  const animate = useCallback(
    (time: number) => {
      if (
        previousTimeRef.current != undefined
                && time - previousTimeRef.current > FPS_THROTTLE
      ) {
                client?.render();
      }
      previousTimeRef.current = time;
      requestRef.current = requestAnimationFrame(animate);
    },
    [client === undefined],
  );

  useEffect(() => {
    requestRef.current = requestAnimationFrame(animate);
    return () => cancelAnimationFrame(requestRef.current!);
  }, [client === undefined]); // Make sure the effect runs only once

  useEffect(() => {
    if (canvas !== undefined) {
      const tmpClient = new wasm.GlClient(canvas);
      setClient(tmpClient);
    }
  }, [canvas]);

  useEffect(() => {
    setRectEl(document.getElementById(canvasId)?.getBoundingClientRect());
  }, []);

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
    [client],
  );

  return (
    <div className="container-fluid mt-5 mr-5">
      <div className="row">
        <div className="col">
          <canvas
            id={canvasId}
            height={height}
            width={width}
            onMouseDown={mouseDown}
            onMouseUp={mouseUp}
            onMouseMove={mouseMove}
          />
        </div>
        <div className="col">
          {wasm && canvas && client && (
            <RenderableDropdown client={client} wasm={wasm} />
          )}
        </div>
      </div>
    </div>
  );
}
