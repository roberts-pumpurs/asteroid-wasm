import express from 'express';
import database from './database';
import { Mouse } from "./models";

const app = express();
const port = 8000; // default port to listen

// start the Express server
app.listen(port, () => {
  console.log(
    `server started at http://localhost:${port}\n`
  + 'Access assets via GET /mouses\n'
  + 'Access assets via POST /mouses\n'
  + 'Access assets via GET /mouses/{:id}\n'
  + 'Access assets via PUT /mouses/{:id}\n'
  + 'Access assets via DELETE /mouses/{:id}',
  );
});
// Parse request body as json
app.use(express.json());

// Get All
app.get('/api/mouses', (req, res) => {
  res.send(database.mouse.getAll());
});

// GET specific
app.get('/api/mouses/:id', (req, res) => {
  const item = database.mouse.get(Number(req.params.id));
  res.send(item);
  res.status(item === null ? 404 : 200);
});

// Create new
app.post('/api/mouses', (req, res) => {
  const obj: Mouse = req.body;
  const saved = database.mouse.create(obj);
  res.send({ success: saved });
  res.status(saved ? 201 : 400);
});

// Delete
app.delete('/api/mouses/:id', (req, res) => {
  const deleted = database.mouse.delete(Number(req.params.id));
  res.send({ success: deleted });
  res.status(deleted ? 204 : 400);
});

// Update
app.put('/api/mouses/:id', (req, res) => {
  const obj: Mouse = { ...req.body, pk: Number(req.params.id) };
  const updated = database.mouse.update(obj);
  res.send({ success: updated });
  res.status(updated ? 204 : 400);
});
