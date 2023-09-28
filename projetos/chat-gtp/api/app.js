const express = require('express');
const bodyParser = require('body-parser');
const app = express();
const jsonParser = bodyParser.json();
const port = 3000;
const net = require('net');
const cors = require('cors');

app.use(cors());

app.post('/msg', jsonParser, (req, res) => {
  console.log(req.body.msg)
    const client = new net.Socket();
  
    client.connect(8080, '127.0.0.1');
  
    client.on('error', (err) => {
      console.error('Error in socket:', err);
      client.end();
      res.status(500).send('Error in socket connection');
    });
  
    client.on('data', (data) => {
      res.status(200).json({ response: data.toString() });
  
      client.end();
    });
  
    client.write(req.body.msg);
  });

app.listen(port, () => {
  console.log(`API ouvindo na porta ${port}.`);
});