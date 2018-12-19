const fs = require('fs');
const alchemicalReduction = require('./AlchemicalReduction');

const str = fs.readFileSync('/dev/stdin', 'utf-8');
const reducedResult = alchemicalReduction(str);
console.log(`Units ${reducedResult.length}`);
console.log(reducedResult);
