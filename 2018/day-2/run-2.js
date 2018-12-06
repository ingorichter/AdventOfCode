const fs = require('fs');
const commonLetters = require('./InventoryManagementSystem-2');

const str = fs.readFileSync('./input.txt', 'utf8');
console.log(commonLetters(str.split('\n')));
