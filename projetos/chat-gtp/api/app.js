const express = require('express')
const bodyParser = require('body-parser')
const app = express()
const jsonParser = bodyParser.json()
const port = 3000

app.post('/msg', jsonParser, (req, res) => {
    // print what the client sends
    console.log(req.body.msg)

    // send ok
    res.sendStatus(200)
})

app.post('/img', (req, res) => {
    // print what the client sends
    console.log(req.body.msg)

    // send ok
    res.sendStatus(200)
})

app.listen(port, () => {
    console.log(`API ouvindo na porta ${port}.`)
})