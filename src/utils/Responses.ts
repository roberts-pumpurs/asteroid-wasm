import { Country, Game, User } from 'types';

export interface GameListingResponse {
  games: Array<GameListing>
}

export interface GameListing {
  game: Game,
  user: User,
  country: Country,
}

export interface SingleUserAggregate {
  user: User,
  score: number,
  duration: number,
  count: number,
  country: Country,
}

export interface Leaderboard {
  [key: string]: SingleUserAggregate
}
