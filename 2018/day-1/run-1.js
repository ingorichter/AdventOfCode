const fs = require('fs');
const chronalCalibration = require('./chronalCalibration');

const str = fs.readFileSync('./input.txt', 'utf8');
console.log(chronalCalibration(str));
