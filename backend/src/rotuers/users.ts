import express from 'express';
import {
  createUser, deleteUser, getUsers, updateUser,
} from '../database';
import { User } from '../models';

const router = express.Router();

router.get('', async (req, res) => {
  const filterUser: User = {
    name: req.query.name?.toString() || '',
    surname: req.query.surname?.toString() || '',
    username: req.query.username?.toString() || '',
  };
  const users = await getUsers(filterUser);
  res.send({ users });
});

router.post('/', async (req, res) => {
  const obj: User = req.body;
  const created = await createUser(obj);
  res.status(created ? 201 : 400).send({ created });
});

router.delete('/:username', async (req, res) => {
  const { username } = req.params;
  const deleted = await deleteUser(username);
  res.status(deleted ? 204 : 400).send({ deleted });
});

router.put('/:username', async (req, res) => {
  const { username } = req.params;
  const obj: User = req.body;
  obj.username = obj.username || 'Anonymous';
  const updated = await updateUser(obj, username);
  res.status(updated ? 204 : 400).send({ updated });
});

// eslint-disable-next-line import/no-default-export
export default router;
