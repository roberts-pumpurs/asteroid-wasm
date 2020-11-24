import React, { ReactElement, useEffect, useState } from 'react';
import { Requester } from 'utils/Requester';
import { Leaderboard } from 'utils/Responses';

import style from '../GameHistory/GameHistory.module.scss';

export function LeaderboardComponent(): ReactElement {
  const [scoreboard, setScoreboard] = useState<Leaderboard>();

  useEffect(() => {
    async function getInitialBoards(): Promise<void> {
      const scoreboardsFetched = await Requester.FetchLeaderboards();
      setScoreboard(scoreboardsFetched);
    }
    void getInitialBoards();
  }, []);

  return (
    <div className={style['game-table']}>
      <table>
        <thead>
          <th>Player</th>
          <th>Avg duration (min)</th>
          <th>Avg Score</th>
          <th>Country</th>
        </thead>
        <tbody>
          {scoreboard && Object.keys(scoreboard).map((key) => {
            const el = scoreboard[key];
            return (
              <tr
                key={`${el.score}-${el.user.username}`}
              >
                <td className={style.name}>{el.user.username}</td>
                <td className={style.duration}>{el.duration / el.count}</td>
                <td className={style.score}>{el.score / el.count}</td>
                <td className={style.country}>{el.country.countryCode}</td>
              </tr>
            );
          })}
        </tbody>
      </table>
    </div>
  );
}
