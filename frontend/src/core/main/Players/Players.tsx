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
  const [editingUser, setEditingUser] = useState<User | null>(null);
  const [editingUsername, setEditingUsername] = useState<string | null>(null);
  const [editingName, setEditingName] = useState<string | null>(null);
  const [editingSurname, setEditingSurname] = useState<string | null>(null);

  useEffect(() => {
    void fetchUsers(setAllUsers);
  }, []);

  return (
    <div className={style.container}>

      <h2>
        Players
      </h2>
      <div className={style['user-container']}>
        {allUsers.map((e) => {
          if (editingUser && e.username === editingUser.username) {
            return (
              <div key={e.username + e.name + e.surname} className={style['user-card-editing']}>
                <div className={style.header}>
                  <input
                    value={editingUsername || ''}
                    onChange={(event) => setEditingUsername(event.target.value)}
                  />
                </div>
                <div className={style.detail}>
                  <input
                    value={editingName || ''}
                    onChange={(event) => setEditingName(event.target.value)}
                  />
                  <input
                    value={editingSurname || ''}
                    onChange={(event) => setEditingSurname(event.target.value)}
                  />
                </div>
                <div className={style.footer}>
                  <button
                    type="button"
                    className={`${style['btn-tiny']} ${style['draw-border-dark']}`}
                    onClick={() => { setEditingUser(null); }}
                  >
                    Save
                    <i className="fas fa-user-plus" />
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
            );
          }
          return (
            <div key={e.username + e.name + e.surname} className={style['user-card']}>
              <div className={style.header}>
                {e.username || '[Unknown]'}
              </div>
              <div className={style.detail}>
                {e.name && e.surname ? `${e.name} ${e.surname}` : '[Unknown Unknown]'}
              </div>
              <div className={style.footer}>
                <button
                  type="button"
                  className={`${style['btn-tiny']} ${style['draw-border-dark']}`}
                  onClick={() => {
                    setEditingUser(e);
                    setEditingName(e.name);
                    setEditingUsername(e.username);
                    setEditingSurname(e.surname);
                  }}
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
          );
        })}
      </div>
    </div>
  );
}
