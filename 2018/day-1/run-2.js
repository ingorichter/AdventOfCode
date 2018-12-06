const fs = require('fs');
const chronalCalibration = require('./chronalCalibration-2');

const str = fs.readFileSync('./input.txt', 'utf8');
console.log(chronalCalibration(str));
