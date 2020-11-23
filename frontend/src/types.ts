export interface User {
  name: string,
  surname: string,
  username: string,
}

export interface Country {
  name: string,
  population: number,
}

export interface Game {
  score: number,
  start: string,
  end: string,
}

export enum GameState {
  INITIALIZING,
  RUNNING,
  GAME_OVER,
}
