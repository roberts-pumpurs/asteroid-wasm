import React, { ReactElement, useEffect, useState } from 'react';
import { Requester } from 'utils/Requester';
import { GameListing } from 'utils/Responses';

import style from './GameHistory.module.scss';

export function GameHistory(): ReactElement {
  const [games, setGames] = useState<Array<GameListing>>([]);

  useEffect(() => {
    async function getInitialGames(): Promise<void> {
      const gamesFetched = await Requester.FetchGames();
      setGames(gamesFetched.games);
    }
    void getInitialGames();
  }, []);

  return (
    <div className={style['game-table']}>
      <table>
        <thead>
          <th>Game duration (min)</th>
          <th>Score</th>
          <th>Player</th>
          <th>Country</th>
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
  );
}
