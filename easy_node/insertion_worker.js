const insertion_sort = require('./insertion_sort.js');

const { parentPort, workerData } = require('worker_threads');

var arr = workerData;

arr = insertion_sort.insertion_sort(arr);

// Send the hashedArray to the parent thread
parentPort.postMessage(arr);
process.exit()

