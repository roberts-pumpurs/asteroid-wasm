import express from 'express';
import cors from 'cors';
import {
  addUserToCounty,
  createCountry, createGame, createUser, getGames, getNumNodes, getUsers, initial, deleteUser, updateUser,
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
  res.status(created ? 201 : 400).send({ created });
});

app.delete('/api/users/:username', async (req, res) => {
  const { username } = req.params;
  const deleted = await deleteUser(username);
  res.status(deleted ? 204 : 400).send({ deleted });
});

app.put('/api/users/:username', async (req, res) => {
  const { username } = req.params;
  const obj: User = req.body;
  const updated = await updateUser(obj, username);
  res.status(updated ? 204 : 400).send({ updated });
});

app.post('/api/countries', async (req, res) => {
  const obj: Country = req.body;
  const created = await createCountry(obj);
  res.status(created ? 201 : 400).send({ created });
});

interface Params {
  game: Game,
  user: User,
  country: Country,
}
app.post('/api/games', async (req, res) => {
  const obj: Params = req.body;

  await createUser(obj.user);
  await createCountry(obj.country);
  await addUserToCounty(obj.country, obj.user);
  const createdGame = await createGame(obj.game, obj.user);

  res.status(createdGame ? 201 : 400).send({ createdGame });
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
    const duration = (new Date(el.game.end).getTime() - new Date(el.game.start).getTime());
    aggregateLeaderbaord[el.user.username].score += el.game.score;
    aggregateLeaderbaord[el.user.username].count += 1;
    aggregateLeaderbaord[el.user.username].duration += duration;
    aggregateLeaderbaord[el.user.username].country = el.country;
  });

  res.send(aggregateLeaderbaord);
});
