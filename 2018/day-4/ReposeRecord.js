const regex = /\[(.*)\]\s+(.*)$/gm;

const sortByTimestamp = (reposeRecords) => {
	const sortedReposeRecords = reposeRecords.sort((a, b) => {
		const partsA = a.split(regex);
		const partsB = b.split(regex);
		const dateA = new Date(partsA[1]);
		const dateB = new Date(partsB[1]);

		return dateA.getTime() > dateB.getTime() ? 1 : -1;
	});

	return sortedReposeRecords;
};

const reposeRecords = (data) => {
	// console.log(data);
	const sortedByTimestamp = sortByTimestamp(data.split('\n'));
	let guardId;
	let startMinute = 0;
	const guardIdToSleepMap = new Map();
    
	sortedByTimestamp.forEach(ts => {
		const parts = ts.split(regex);

		const timestamp = new Date(parts[1]);
		const remainingstr = parts[2];

		const gid = /#(\d+)/.exec(remainingstr);
		if (gid !== null) {
			// new guard
			guardid = gid[1];
			if (!guardIdToSleepMap.has(guardid)) {
				guardIdToSleepMap.set(guardid, new Array(60));
			}
			startminute = 0;
		} else {
			if (remainingstr.indexOf('falls asleep') > -1) {
				startminute = timestamp.getMinutes();
			}
			else {
				const endminute = timestamp.getMinutes();
				const sleepminutes = guardIdToSleepMap.get(guardid);

				for(let i = startminute; i < endminute; i++) {
					if (sleepminutes[i] === undefined) {
						sleepminutes[i] = 1;
					} else {
						sleepminutes[i]++;
					}
				}
			}
		}
	});

	// FIND THE GUARD WHO SLEPT THE MOST DURING HIS SHIFT
	let guardWhoSleptTheMost = {guardId: 0, minutesSlept: 0};

	guardIdToSleepMap.forEach((value, key) => {
		const minutesSlept = value.reduce((acc, val) => acc + val, 0);

		if (minutesSlept > guardWhoSleptTheMost.minutesSlept) {
			guardWhoSleptTheMost = {guardId: key, minutesSlept: minutesSlept};
		}
	});

	const sleepMinutes = guardIdToSleepMap.get(guardWhoSleptTheMost.guardId);
	let maxIndex = -1;
	let max = -1;
    
	for(let i = 0; i < sleepMinutes.length; i++) {
		if (sleepMinutes[i] > max) {
			max = sleepMinutes[i];
			maxIndex = i;
		}
	}

	return guardWhoSleptTheMost.guardId * maxIndex;
};

module.exports = reposeRecords;
