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
  const [error, setError] = useState(false);

  const updateUsers = useCallback(
    () => {
      void fetchUsers(setAllUsers);
    },
    [setAllUsers],
  );

  useEffect(() => {
    updateUsers();
  }, [updateUsers]);

  const [props, set] = useSpring(() => ({
    opacity: 1,
    config: { mass: 1, tension: 1000, friction: 100 },
    onRest: (a: { opacity: number}) => {
      if (a.opacity === 0) {
        setError(false);
      }
    },
  }));

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
              updateExisting={(neUser: User) => updateUser(e.username, neUser).then(() => updateUsers()).catch(() => { set({ opacity: 1 }); setError(true); })}
              deleteExisting={() => deleteUser(e).then(() => updateUsers()).catch(() => { set({ opacity: 1 }); setError(true); })}
            />
          ))}
        </div>
      </div>
      { error && (
      <animated.div
        onClick={() => set({ opacity: 0 })}
        style={{ opacity: props.opacity }}
        className={style.error}
      >
        <abbr
          title="Click to dismiss"
        >
          Something went wrong with that request.
          Make sure the username is unique!
        </abbr>
      </animated.div>
      )}
    </>
  );
}
