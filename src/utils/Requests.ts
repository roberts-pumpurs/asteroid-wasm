import { Country, Game, User } from 'types';

export interface SaveGameRequestParam {
  game: Game,
  user: User,
  country: Country,
}

