const fs = require('fs');
const reposeRecords = require('./ReposeRecord');

const str = fs.readFileSync('/dev/stdin', 'utf-8');
// const str = fs.readFileSync('./input.txt', 'utf8');
// console.log(str.split('\n'));
console.log(reposeRecords(str));
