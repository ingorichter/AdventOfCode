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
	let guardWhoSleptTheMost = [];

	guardIdToSleepMap.forEach((value, key) => {
		let max = -1;
		let maxIndex = -1;
        
		for (let i = 0; i < value.length; i++) {
			if (value[i] > max) {
				max = value[i];
				maxIndex = i;
			}
		}

		guardWhoSleptTheMost.push({gid: key, sleepMostOnMinute: maxIndex, minutes: max});
	});

	let index = -1;
	let maxMinutes = -1;
	for (let t = 0; t < guardWhoSleptTheMost.length; t++) {
		if (guardWhoSleptTheMost[t].minutes > maxMinutes) {
			maxMinutes = guardWhoSleptTheMost[t].minutes;
			index = t;
		}
	}

	return (parseInt(guardWhoSleptTheMost[index].gid) * guardWhoSleptTheMost[index].sleepMostOnMinute);
};

module.exports = reposeRecords;
