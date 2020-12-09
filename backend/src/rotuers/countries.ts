import express from 'express';
import {
  createCountry,
} from '../database';
import {
  Country,
} from '../models';

const router = express.Router();

router.post('/api/countries', async (req, res) => {
  const obj: Country = req.body;
  const created = await createCountry(obj);
  res.status(created ? 201 : 400).send({ created });
});

// eslint-disable-next-line import/no-default-export
export default router;
