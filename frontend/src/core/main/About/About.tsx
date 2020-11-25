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

      <div className={style.text}>
        <section>
          <h1>About</h1>
          <div>
            The whole website is built on top of some hot and emerging technologies,
            some of them even have still newly emerging and developing standards.
          </div>
        </section>

        <section>
          <h2>TypeScript</h2>
          <div>
            I've chosen TypeScript for writing the frontend (React) and backend
            (NodeJS/Express), because I'm fan of strongly typed languages, compiled
            languages and I really dislike JavaScript with all of its undefined
            behaviour.
          </div>
        </section>
      </div>

      <h1>Small Rust/WASM/WebGL Samples</h1>
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
      <div className={style['error-message']}>
        <h1>To see the WASM demo, screen size must be at least 768x721px</h1>
      </div>

      <div className={style.text}>
        <section>
          <h2>React</h2>
          <div>
            Well... React is an obligatory part of the task. I have solid prior
            knowledge of React before working on this SPA, therefore there's
            nothing much I can comment. If I had a choice, then I would definitely
            try out something more interesting like Vue or Svetle.
          </div>
        </section>

        <section>
          <h2>Rust</h2>
          <div>
            C/C++ speed? Check. Amazing compiler? Check. Amazing type system?
            Check. Amazing WASM support? Check. This language is everything what
            C++ 20 is aspiring be. Some experimental language design
            features as `ownership` and `borrowing` make this a steep learning curve (
            which I am still fighting myself to this day), but it well worth it.
            No wonder it has been the most loved StackOOverflow language for 5
            years in a row.
          </div>
        </section>

        <section>
          <h2>Wasm</h2>
          <div>
            I need to get the Rust code up and running in t he browser, don't I?
            WebAssemly is a new and emerging standard for compiling any langauge
            into a binary that can be executed by modern browsers, making it faster,
            more transportable and safer than JavaScript (do you also get a
            feeling that I try everyhting to escape JavaScript?)!
          </div>
        </section>

        <section>
          <h2>WebGL</h2>
          <div>
            To actually draw the shapes and everything else on the canvas element,
            I am using Rust, compiled to WASM, which interacts with the Canvas
            element and performs WebGL procedures. This, to be honest, had to be
            the most difficult part of the whole app, as learning graphics programming,
            the whole graphics pipeline, a shader langauge, design a simple game
            engine and everything else just to make this beast work was truly
            not the easiest of the tasks I've encountered.
          </div>
        </section>

        <section>
          <h2>NodeJS Express</h2>
          <div>
            If I had a choice, I would never use anything JS related for the
            backend. Rust would be my go to choice with something like `actix-web`
            or `Rocket`. I've built a lot APIs on interpreted languages, my
            suggestion is to use something stable. Express was nice in a way that
            it is so simple and stripped down that I did not have to spend a lot
            of time learning the grips of it. As well as using TypeScript helped
            with some of the issues that pure JS or something like Python
            would've caused.
          </div>
        </section>

        <section>
          <h2>Neo4J graph database</h2>
          <div>
            Why not something more mainstream like MongoDB? Because it seemed
            boring. Set theory (what SQL is based on) is no way superior than
            graph theory (Neo4Js implementation of a graph DB), so it is fully
            capable of storing data, relationships, querying data. Neo4j even
            introduced its own graph querying langauge called `Cypher`. I had a
            lot to learn to figure out how to manage this database. Sadly, Neo4j
            is pretty much the only Graph DB supplier, so it isn't really standardised.
          </div>
        </section>

      </div>
    </div>
  );
}
