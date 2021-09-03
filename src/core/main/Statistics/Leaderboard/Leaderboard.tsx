import React, { ReactElement, useEffect, useState } from 'react';
import { Requester } from 'utils/Requester';
import { Leaderboard } from 'utils/Responses';

import style from '../Statistics.module.scss';

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
    <table>
      <thead>
        <tr>
          <td>Player</td>
          <td>Avg duration (sec)</td>
          <td>Avg Score</td>
          <td>Country</td>
        </tr>
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
  );
}
