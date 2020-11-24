/* Simple interfaces */
export interface User {
  name: string,
  surname: string,
  username: string,
}


export interface Country {
  country: string,
  countryCode: string,
}

export interface Game {
  score: number,
  start: string,
  end: string,
}

/* Composites */

export interface GameListing {
  game: Game,
  user: User,
  country: Country,
}
