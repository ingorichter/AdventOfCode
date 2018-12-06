const calculateChecksum = (boxIdArray) => {
	let twoLetterCount = 0;
	let threeLetterCount = 0;
	const letterMap = new Map();
    
	boxIdArray.forEach(boxid => {
		for(let i = 0; i < boxid.length; i++) {
	    let count = letterMap.get(boxid[i]) || 0;
	    count++;
	    letterMap.set(boxid[i], count);
		}

		// console.log(letterMap);
		let hasOne2 = false;
		let hasOne3 = false;
		const letterCountIterator = letterMap[Symbol.iterator]();
		for (const [key, value] of letterCountIterator) {
	    if (value === 2 && !hasOne2) {
				hasOne2 = true;
	    }
	    if (value === 3 && !hasOne3) {
				hasOne3 = true;
	    }
		}

		if (hasOne2) {
	    twoLetterCount++;
		}
		if (hasOne3) {
	    threeLetterCount++;
		}

		letterMap.clear();
	});

	return twoLetterCount * threeLetterCount;
};

module.exports = calculateChecksum;
