const alchemicalReduction = require('./AlchemicalReduction');

const removeUnitFromPolymer = (polymer, unit) => {
    const otherUnitCase = isLowerCase(unit) ? unit.toUpperCase() : unit.toLowerCase();

    return polymer.replace(new RegExp(otherUnitCase, 'g'), '').replace(new RegExp(unit, 'g'), '');
};

const isLowerCase = (str) => str === str.toLowerCase();

const findAllUnitsToRemove = (polymer) => {
    const unitsToRemoveSet = new Set();

    [...polymer].forEach(unit => {
        const unitOtherCase = (isLowerCase(unit) ? unit.toUpperCase() : unit.toLowerCase());
        if (!unitsToRemoveSet.has(unit) && !unitsToRemoveSet.has(unitOtherCase)) {
            unitsToRemoveSet.add(unit);
        }
    });

    return unitsToRemoveSet;
}

const optimalReduction = (polymer) => {
    let shortestPolymer = '';
    const unitToLengthMap = {};  //
    const unitsToRemove = findAllUnitsToRemove(polymer);

    unitsToRemove.forEach(unit => {
        // console.log(unit);
        const reducedPolymer = alchemicalReduction(removeUnitFromPolymer(polymer, unit));
        unitToLengthMap[unit] = {rp: reducedPolymer, length: reducedPolymer.length };
    });

    let shortest = polymer.length;
    Object.keys(unitToLengthMap).forEach((unit) => {
        // console.log(unit);
        const value = unitToLengthMap[unit];
        // console.log(`${unit} => ${value.length}`);
        if (value.length < shortest) {
            shortestPolymer = value.rp;
            shortest = value.length;
        }
    });
    return shortestPolymer;
};

module.exports = optimalReduction;
