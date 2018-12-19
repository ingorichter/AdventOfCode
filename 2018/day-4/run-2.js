const fs = require('fs');
const reposeRecords2 = require('./ReposeRecord-2');

const str = fs.readFileSync('/dev/stdin', 'utf-8');
console.log(reposeRecords2(str));
