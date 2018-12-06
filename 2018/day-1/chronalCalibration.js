const adjustFrequency = (data) => {
	return data.split('\n').map(x => parseInt(x)).reduce((a, b) => a + b, 0);
};

module.exports = adjustFrequency;
