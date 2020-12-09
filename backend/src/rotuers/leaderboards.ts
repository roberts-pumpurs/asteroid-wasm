import express from 'express';
import {
  createCountry, getGames,
} from '../database';
import {
  Country, GameListing, User,
} from '../models';

const router = express.Router();

interface SingleUserAggregate {
    user: User,
    score: number,
    duration: number,
    count: number,
    country: Country
}
interface Leaderboard {
    [key: string]: SingleUserAggregate
}
router.get('', async (req, res) => {
  const games: Array<GameListing> = await getGames();
  const aggregateLeaderbaord: Leaderboard = {};

  games.forEach((el) => {
    if (!(el.user.username in aggregateLeaderbaord)) {
      aggregateLeaderbaord[el.user.username] = {
        score: 0,
        count: 0,
        user: el.user,
        duration: 0,
        country: { country: '', countryCode: '' },
      };
    }
    const duration = (new Date(el.game.end).getTime() - new Date(el.game.start).getTime());
    aggregateLeaderbaord[el.user.username].score += el.game.score;
    aggregateLeaderbaord[el.user.username].count += 1;
    aggregateLeaderbaord[el.user.username].duration += duration;
    aggregateLeaderbaord[el.user.username].country = el.country;
  });

  res.send(aggregateLeaderbaord);
});

// eslint-disable-next-line import/no-default-export
export default router;
