import React, { ReactElement, useEffect, useState } from 'react';

import { Requester } from 'utils/Requester';
import { GameListing } from 'utils/Responses';
import { PieChartCustom } from './Chart';

import style from './GameHistory.module.scss';

export function GameHistory(): ReactElement {
  const [games, setGames] = useState<Array<GameListing>>([]);

  useEffect(() => {
    async function getInitialGames(): Promise<void> {
      const gamesFetched = await Requester.FetchGames();
      setGames(gamesFetched.games.slice(0, 30));
    }
    void getInitialGames();
  }, []);

  /* Calculate data for chart */
  const countryScoreMap: {[key: string]: number} = {};
  games.forEach((e) => {
    if (!Object.keys(countryScoreMap).includes(e.country.countryCode)) {
      countryScoreMap[e.country.countryCode] = 0;
    }
    countryScoreMap[e.country.countryCode] += e.game.score;
  });
  const countryScoreData = Object.keys(countryScoreMap).map(
    (key) => ({ type: key, data: countryScoreMap[key] }),
  );

  return (
    <>
      <div className={style['game-table']}>
        <h1 className={style.title}>Latest Game History</h1>
        <table>
          <thead>
            <tr>
              <td>Game duration (sec)</td>
              <td>Score</td>
              <td>Player</td>
              <td>Country</td>
            </tr>
          </thead>
          <tbody>
            {games.map((el) => (
              <tr
                key={`${el.game.score} ${el.user.username} ${el.game.start}`}
              >
                <td className={style.duration}>{((new Date(el.game.end).getTime() - new Date(el.game.start).getTime()))}</td>
                <td className={style.score}>{el.game.score}</td>
                <td className={style.name}>{el.user.username}</td>
                <td className={style.country}>{el.country.countryCode}</td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
        <h1 className={style.title}>Countries With The Most Score</h1>
        <PieChartCustom data={countryScoreData}/>
    </>
  );
}
