import React from 'react';

/* Provide the WASM components as a globally accessible context element */
export interface WasmContextInterface {
  wasm: typeof import('wasm-app');
}

const ctxt = React.createContext<WasmContextInterface | null>(null);

export const WasmContextProvider = ctxt.Provider;

export const WasmContextConsumer = ctxt.Consumer;
