const adjustFrequency = (data) => {
	const freqChanges = data.split('\n').map(x => parseInt(x));
	const frequencies = new Set();
    
	let frequency = 0;
	let index = 0;
    
	while (true) {
		if (index === freqChanges.length) {
			index = 0; // reset

			continue;
		}

		frequency += freqChanges[index];

		if (frequencies.has(frequency)) {
			break;
		}

		frequencies.add(frequency);

		index++;
	}

	return frequency;
};

module.exports = adjustFrequency;
