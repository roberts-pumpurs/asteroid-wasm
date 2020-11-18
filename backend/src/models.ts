/* Simple interfaces */
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
  start: Date,
  end: Date,
}

/* Composites */

export interface GameListing {
  game: Game,
  user: User,
  country: Country,
}
