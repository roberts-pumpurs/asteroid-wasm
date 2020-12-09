import express from 'express';
import {
  addUserToCounty, createCountry, createGame, createUser, getGames,
} from '../database';
import {
  Country, Game, GameListing, User,
} from '../models';

const router = express.Router();

interface Params {
  game: Game,
  user: User,
  country: Country,
}

router.post('', async (req, res) => {
  const obj: Params = req.body;
  obj.user.username = obj.user.username || 'Anonymous';
  await createUser(obj.user);
  await createCountry(obj.country);
  await addUserToCounty(obj.country, obj.user);
  const createdGame = await createGame(obj.game, obj.user);

  res.status(createdGame ? 201 : 400).send({ createdGame });
});

router.get('', async (req, res) => {
  const games: Array<GameListing> = await getGames();
  res.send({ games });
});

// eslint-disable-next-line import/no-default-export
export default router;
