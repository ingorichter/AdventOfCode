const fs = require('fs');
const calculateChecksum = require('./InventoryManagementSystem');

const str = fs.readFileSync('./input.txt', 'utf8');
console.log(calculateChecksum(str.split('\n')));
