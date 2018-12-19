const fs = require('fs');
const alchemicalReduction = require('./AlchemicalReduction');
const optimalReduction = require('./AlchemicalReduction-2');

const str = fs.readFileSync('/dev/stdin', 'utf-8');
const reducedResult = optimalReduction(alchemicalReduction(str));
console.log(`Units ${reducedResult.length}`);
console.log(reducedResult);
