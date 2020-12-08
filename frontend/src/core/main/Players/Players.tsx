import React, {
  ReactElement, useCallback, useEffect, useState,
} from 'react';
import { User } from 'types';
import { Requester } from 'utils/Requester';
import { useSpring, animated } from 'react-spring';
import style from './Player.module.scss';
import { PlayerCard } from './PlayerCard';

async function fetchUsers(callback: (arg0: Array<User>) => void): Promise<void> {
  const result = await Requester.getAllUsers();
  callback(result.users);
}

async function deleteUser(user: User): Promise<void> {
  await Requester.deleteUser(user.username);
}

async function updateUser(username: string, user: User): Promise<void> {
  await Requester.updateUser(username, user);
}

export function Players(): ReactElement {
  /* All users */
  const [allUsers, setAllUsers] = useState<Array<User>>([]);

  /* Error */
  const [error, setError] = useState(true);

  const updateUsers = useCallback(
    () => {
      void fetchUsers(setAllUsers);
    },
    [setAllUsers],
  );

  useEffect(() => {
    updateUsers();
  }, [updateUsers]);

  return (
    <>
      <div className={style.container}>
        <h2>
          Players
        </h2>
        <div className={style['user-container']}>
          {allUsers.map((e) => (
            <PlayerCard
              key={e.name + e.surname + e.username}
              user={e}
              updateExisting={(neUser: User) => updateUser(e.username, neUser).then(() => updateUsers()).catch(() => setError(true))}
              deleteExisting={() => deleteUser(e).then(() => updateUsers()).catch(() => setError(true))}
            />
          ))}
        </div>
      </div>
      { error && (
        <button
          type="button"
          onClick={() => setError(false)}
          className={style.error}
        >
          <abbr
            title="Click to dismiss"
          >
            Something went wrong with that request.
            Make sure the username is unique!
          </abbr>
        </button>
      )}
    </>
  );
}
