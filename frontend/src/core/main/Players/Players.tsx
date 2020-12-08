import React, { ReactElement, useEffect, useState } from 'react';
import { User } from 'types';
import { Requester } from 'utils/Requester';
import style from './Player.module.scss';

async function fetchUsers(callback: (arg0: Array<User>) => void): Promise<void> {
  const result = await Requester.getAllUsers();
  callback(result.users);
}

export function Players(): ReactElement {
  const [allUsers, setAllUsers] = useState<Array<User>>([]);
  console.log(style);
  useEffect(() => {
    void fetchUsers(setAllUsers);
  }, []);

  return (
    <div className={style.container}>

      <h2>
        Players
      </h2>
      <div className={style['user-container']}>
        {allUsers.map((e) => (
          <div key={e.username + e.name + e.surname} className={style['user-card']}>
            <div className={style.header}>
              {e.username}
            </div>
            <div className={style.detail}>
              {e.name} {e.surname}
            </div>
            <div className={style.footer}>
              <button
                type="button"
                className={`${style['btn-tiny']} ${style['draw-border-dark']}`}
              >
                Edit
                <i className="fas fa-user-edit" />
              </button>
              <button
                type="button"
                className={`${style['btn-tiny']} ${style['draw-border-dark']}`}
              >
                Delete
                <i className="fas fa-user-slash" />
              </button>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
