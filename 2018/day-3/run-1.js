const fs = require('fs');
const overlappingClaims = require('./OverlappingClaims');

const str = fs.readFileSync('./input.txt', 'utf8');
// console.log(str.split('\n'));
console.log(overlappingClaims(str.split('\n')));