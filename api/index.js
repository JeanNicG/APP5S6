const express = require('express');
const app = express();
const controlRoutes = require('./routes/control');

app.use(express.json());

// Routes
app.use('/control', controlRoutes);

const PORT = process.env.PORT || 3000;
app.listen(PORT, () => {
    console.log(`API Server is running on port ${PORT}`);
});
