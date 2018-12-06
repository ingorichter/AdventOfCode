const regex = /#\d+\s+@\s+(\d+),(\d+):\s+(\d+)x(\d+)/g;

const makeRect = (claim) => {
	// const match = regex.exec(claim);
	const parts = claim.split(regex);

	return {
		x1: parseInt(parts[1]),
		y1: parseInt(parts[2]),
		x2: parseInt(parts[1]) + parseInt(parts[3]),
		y2: parseInt(parts[2]) + parseInt(parts[4])
	};
};

const howMuchOverlap = (rectangleA, rectangleB) => {
	// https://stackoverflow.com/questions/9324339/how-much-do-two-rectangles-overlap
	// const area1 = Math.abs(rectangleA.x1 - rectangleA.x2) * Math.abs(rectangleA.y1 - rectangleA.y2);
	// const area2 = Math.abs(rectangleB.x1 - rectangleB.x2) * Math.abs(rectangleB.y1 - rectangleB.y2);
	// Blue = Rectangle A
	// Orange = Rectangle B
	// Triangle = top left corner
	// Circle = botton right corner
	// intersecting_area = 
	// 	max(0, min(orange.circle.x, blue.circle.x) - max(orange.triangle.x, blue.triangle.x)) * max(0, min(orange.circle.y, blue.circle.y) - max(orange.triangle.y, blue.triangle.y));

	const overlap = Math.max(0, Math.min(rectangleA.x2, rectangleB.x2) - Math.max(rectangleA.x1, rectangleB.x1)) * Math.max(0, Math.min(rectangleA.y2, rectangleB.y2) - Math.max(rectangleA.y1, rectangleB.y1));

	return overlap;
};

const overlappingClaims = (claims) => {
	const rectangles = claims.map(claim => makeRect(claim));
	// console.log(`Rectangles: ${JSON.stringify(rectangles)}`);
	const overlapMap = new Map();

	rectangles.forEach((rectangle) => {
		for (let i = rectangle.x1; i < rectangle.x2; i++) {
			for (let row = rectangle.y1; row < rectangle.y2; row++) {
				const key = i + '-' + row;
				let overlapCount = overlapMap.get(key) || 0;
				overlapCount++;

				overlapMap.set(key, overlapCount);
			}
		}
	});

	const overlappingRects = Array.from(overlapMap.values()).filter(entry => entry >= 2);

	// for (let outer = 0; outer < rectangles.length; outer++) {
	// 	const rectA = rectangles[outer];
	// 	for (let inner = outer + 1; inner < rectangles.length; inner++) {
	// 	    // console.log(`Rect A ${JSON.stringify(rectA)}, Rect B ${JSON.stringify(rectangles[inner])}`);
	// 	    const overlap = howMuchOverlap(rectA, rectangles[inner]);
	// 	    // console.log(`Overlap ${overlap}`);
	// 	    overlappingRects += overlap;
	// 	}
	// }

	return overlappingRects.length;
};

module.exports = overlappingClaims;