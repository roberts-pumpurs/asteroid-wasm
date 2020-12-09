import express from 'express';
import cors from 'cors';
import {getNumNodes, initial } from './database';

import routerUser from './rotuers/users';
import routerGames from './rotuers/games';
import routerCountries from './rotuers/countries';
import routerLeaderboards from './rotuers/leaderboards';

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
// Actual endpoints used by the app
app.use('/api/users', routerUser);
app.use('/api/games', routerGames);
app.use('/api/countries', routerCountries);
app.use('/api/leaderboards', routerLeaderboards);

// NOTE: Misc test endpoint
app.get('/api/nodes', async (req, res) => {
  const nodeCount = await getNumNodes();
  res.send({ nodeCount });
});
