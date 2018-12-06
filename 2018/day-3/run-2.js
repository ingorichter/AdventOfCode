const fs = require('fs');
const notOverlappingClaims = require('./OverlappingClaims-2');

const str = fs.readFileSync('./input.txt', 'utf8');
// console.log(str.split('\n'));
console.log(notOverlappingClaims(str.split('\n')));