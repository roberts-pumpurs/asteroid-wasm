import React, { ReactElement, useEffect, useState } from 'react';
import { Requester } from 'utils/Requester';
import { GameListing } from 'utils/Responses';
import { PieChartCustom } from './Chart';

export function CountryChartPlaytime(): ReactElement {
  const [games, setGames] = useState<Array<GameListing>>([]);

  useEffect(() => {
    async function getInitialGames(): Promise<void> {
      const gamesFetched = await Requester.FetchGames();
      setGames(gamesFetched.games.slice(0, 30));
    }
    void getInitialGames();
  }, []);

  /* Calculate data for chart */
  const countryScoreMap: { [key: string]: number } = {};
  games.forEach((e) => {
    if (!Object.keys(countryScoreMap).includes(e.country.countryCode)) {
      countryScoreMap[e.country.countryCode] = 0;
    }
    countryScoreMap[e.country.countryCode] += (new Date(e.game.end).getTime() - new Date(e.game.start).getTime());
  });
  const countryScoreData = Object.keys(countryScoreMap).map(
    (key) => ({ type: key, data: countryScoreMap[key] }),
  );

  return <PieChartCustom data={countryScoreData} />;
}
