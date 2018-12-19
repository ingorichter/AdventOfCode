const regex = /(#\d+)\s+@\s+(\d+),(\d+):\s+(\d+)x(\d+)/g;

const makeRect = (claim) => {
	// const match = regex.exec(claim);
	const parts = claim.split(regex);

	return {
		id: parts[1],
		x1: parseInt(parts[2]),
		y1: parseInt(parts[3]),
		x2: parseInt(parts[2]) + parseInt(parts[4]),
		y2: parseInt(parts[3]) + parseInt(parts[5])
	};
};

const notOverlappingClaim = (claims) => {
	const rectangles = claims.map(claim => makeRect(claim));
	// console.log(`Rectangles: ${JSON.stringify(rectangles)}`);
	const overlapMap = new Map();
	const notOverlappingClaims = new Set();

	rectangles.forEach((rectangle) => {
		let overlapped = false;
		for (let i = rectangle.x1; i < rectangle.x2; i++) {
			for (let row = rectangle.y1; row < rectangle.y2; row++) {
				const key = i + '-' + row;
				let claimID = overlapMap.get(key);

				// update the map
				overlapMap.set(key, rectangle.id);

				if (!claimID) {
					notOverlappingClaims.add(rectangle.id);
				}

				if (claimID && !overlapped) {
					overlapped = true;
				} else {
					notOverlappingClaims.delete(claimID);
				}
			}
		}
		if (overlapped) {
			notOverlappingClaims.delete(rectangle.id);
		} else {
			notOverlappingClaims.add(rectangle.id);
		}
	});
	
	return Array.from(notOverlappingClaims.values())[0];
};

module.exports = notOverlappingClaim;
