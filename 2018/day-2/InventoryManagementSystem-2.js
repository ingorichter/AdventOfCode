const removeFromArray = (array, position) => [
	...array.slice(0, position),
	...array.slice(position + 1, array.length)
];

const commonLetters = (boxIdArray) => {
	for(let outerIndex = 0; outerIndex < boxIdArray.length; outerIndex++) {
		const boxId = boxIdArray[outerIndex];

		for (let innerIndex = 0; innerIndex < boxId.length; innerIndex++) {
			const boxToCompare = removeFromArray(boxId, innerIndex).join('');

			const boxIds = removeFromArray(boxIdArray, outerIndex).map(id => removeFromArray(id, innerIndex).join(''));
			
			if (boxIds.some(id => id === boxToCompare)) {
				return boxToCompare;
			}
		}
	}
};

module.exports = commonLetters;
