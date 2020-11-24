import express from 'express';
import cors from 'cors';
import {
  addUserToCounty,
  createCountry, createGame, createUser, getGames, getNumNodes, getUsers, initial,
} from './database';
import {
  Country, Game, GameListing, User,
} from './models';

const app = express();
const port = 8000; // default port to listen

/*  ---------- start the Express server ---------- */
app.listen(port, () => {
  console.log(
    `server started at http://localhost:${port}`,
  );

  // Perform initial DB setup
  initial();
});

app.use(express.json());
app.use(cors());

app.use((req, res, next) => {
  res.header('Access-Control-Allow-Origin', '*');
  res.header(
    'Access-Control-Allow-Headers',
    'Origin, X-Requested-With, Content-Type, Accept',
  );
  next();
});

/* -----------  Neo4j encompassing endpoints ----------- */
app.get('/api/nodes', async (req, res) => {
  const nodeCount = await getNumNodes();
  res.send({ nodeCount });
});

app.post('/api/users', async (req, res) => {
  const obj: User = req.body;
  const created = await createUser(obj);
  res.status(created ? 201 : 400);
  res.send({ created });
});

app.post('/api/countries', async (req, res) => {
  const obj: Country = req.body;
  const created = await createCountry(obj);
  res.status(created ? 201 : 400);
  res.send({ created });
});

interface Params {
  game: Game,
  user: User,
  country: Country,
}
app.post('/api/games', async (req, res) => {
  const obj: Params = req.body;

  const createdUser = await createUser(obj.user);
  const createdCountry = await createCountry(obj.country);
  const addedUser = await addUserToCounty(obj.country, obj.user);
  const createdGame = await createGame(obj.game, obj.user);

  const allCretedSuccessfully = createdUser && createdCountry && addedUser && createdGame;
  res.status(allCretedSuccessfully ? 201 : 400);
  res.send({ allCretedSuccessfully });
});

app.get('/api/users', async (req, res) => {
  const filterUser: User = {
    name: req.query.name?.toString() || '',
    surname: req.query.surname?.toString() || '',
    username: req.query.username?.toString() || '',
  };
  const users = await getUsers(filterUser);
  res.send({ users });
});

app.get('/api/games', async (req, res) => {
  const games: Array<GameListing> = await getGames();
  res.send({ games });
});

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
app.get('/api/leaderboards', async (req, res) => {
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
    const duration = (new Date(el.game.end).getTime() - new Date(el.game.start).getTime()) / 60000;
    aggregateLeaderbaord[el.user.username].score += el.game.score;
    aggregateLeaderbaord[el.user.username].count += 1;
    aggregateLeaderbaord[el.user.username].duration += duration;
    aggregateLeaderbaord[el.user.username].country = el.country;
  });

  res.send(aggregateLeaderbaord);
});
