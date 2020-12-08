import React, { ReactElement, useState } from 'react';
import { User } from 'types';
import { useSpring, animated } from 'react-spring';
import style from './Player.module.scss';

interface Props {
  updateExisting: (newData: User) => void;
  deleteExisting: () => void;
  user: User;
}

const calc = (x: number, y: number): Array<number> => [-(y - window.innerHeight / 2) / 20, (x - window.innerWidth / 2) / 20, 1.1];
const trans = (x: number, y: number, s: number): string => `perspective(600px) rotateX(${Math.max(-20, Math.min(x, 20))}deg) rotateY(${Math.max(-20, Math.min(y, 20))}deg) scale(${s})`;
// max
export function PlayerCard({ updateExisting, user, deleteExisting }: Props): ReactElement {
  /* Currently editing a user */
  const [editingUser, setEditingUser] = useState<User|null>(null);
  const [editingUsername, setEditingUsername] = useState<string>('');
  const [editingName, setEditingName] = useState<string>('');
  const [editingSurname, setEditingSurname] = useState<string>('');

  /* Cool animation */
  const [props, set] = useSpring(() => ({ xys: [1, 1, 1], config: { mass: 5, tension: 350, friction: 40 } }));

  const content = editingUser
    ? (
      <div className={style['user-card-editing']}>
        <div className={style.header}>
          <input
            value={editingUsername || ''}
            onChange={(e) => setEditingUsername(e.target.value)}
          />
        </div>
        <div className={style.detail}>
          <input
            value={editingName || ''}
            onChange={(e) => setEditingName(e.target.value)}
          />
          <input
            value={editingSurname || ''}
            onChange={(e) => setEditingSurname(e.target.value)}
          />
        </div>
        <div className={style.footer}>
          <button
            type="button"
            className={`${style['btn-tiny']} ${style['draw-border-dark']}`}
            onClick={() => {
              updateExisting({
                username: editingUsername,
                name: editingName,
                surname: editingSurname,
              });
            }}
          >
            Save
            <i className="fas fa-user-plus" />
          </button>
          <button
            type="button"
            className={`${style['btn-tiny']} ${style['draw-border-dark']}`}
            onClick={() => { setEditingUser(null); }}
          >
            Cancel
            <i className="fas fa-times" />
          </button>
        </div>
      </div>
    ) : (
      <div className={style['user-card']}>
        <div className={style.header}>
          {user.username || '[Unknown]'}
        </div>
        <div className={style.detail}>
          {user.name || '[Unknown]'}
          {' '}
          {user.surname || '[Unknown]'}
        </div>
        <div className={style.footer}>
          <button
            type="button"
            className={`${style['btn-tiny']} ${style['draw-border-dark']}`}
            onClick={() => {
              setEditingUser(user);
              setEditingName(user.name);
              setEditingUsername(user.username);
              setEditingSurname(user.surname);
            }}
          >
            Edit
            <i className="fas fa-user-user.it" />
          </button>
          <button
            type="button"
            className={`${style['btn-tiny']} ${style['draw-border-dark']}`}
            onClick={deleteExisting}
          >
            Delete
            <i className="fas fa-user-slash" />
          </button>
        </div>
      </div>
    );
  return (
    <animated.div
      key={user.username + user.name + user.surname}
      onMouseMove={({ clientX: x, clientY: y }) => set({ xys: calc(x, y) })}
      onMouseLeave={() => set({ xys: [1, 1, 1] })}
      style={{ transform: props.xys.interpolate(trans as any) }}
    >  {content}
    </animated.div>
  );
}
