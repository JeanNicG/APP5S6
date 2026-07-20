/**
    author: Jean-Nicolas Gosselin, Anahì Michelle Mongelos Toledo
    cip: gosj2008, mona3503
    date: 2026-07-20
**/

const express = require('express');
const cors = require('cors');
const app = express();
const controlRoutes = require('./routes/control');
const archiveRoutes = require('./routes/archive');
app.use(cors());
app.use(express.json());

app.use('/control', controlRoutes);
app.use('/archive', archiveRoutes);
const PORT = 3000;
app.listen(PORT, () => {
    console.log(`API Server is running on port ${PORT}`);
});