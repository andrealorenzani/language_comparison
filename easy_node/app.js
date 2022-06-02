'use strict';

const express = require('express');
const {Worker} = require('worker_threads');
const insertion_sort = require('./insertion_sort.js');
if (!Date.ts) {
    Date.ts = function() { return new Date().getTime(); }
}

// Constants
const PORT = 8081;
const HOST = '0.0.0.0';

// App
const app = express();

app.use(
  express.urlencoded({
    extended: true,
  })
);
//app.use(express.raw({type:'text/plain'}))
app.use(express.json());


var farmprocstart = 0;
var farmprocend = 0;


app.post('/', (req, res) => {
  console.log(req.body);
  console.time("Execution");
  var result = insertion_sort.insertion_sort(req.body.test);
  console.timeEnd("Execution");
  console.time("ExecutionFarm");
  farmprocstart = Date.ts();
  start_farm(req.body.test);
  console.timeEnd("ExecutionFarm");
  res.send('PONG: '+result);
});

var start_farm = function(arr) {
  for (let i = 0; i < 200; i++) {
    start_worker(arr, i);
  }
}

var start_worker = function(arr, id) {
  const worker = new Worker('./insertion_worker.js', { 
      workerData: arr
  });
  worker.once('message', (arr) => {
    farmprocend = Date.ts();
    console.log(farmprocend - farmprocstart);
  });
}


app.listen(PORT, HOST);
console.log(`Running on http://${HOST}:${PORT}`);