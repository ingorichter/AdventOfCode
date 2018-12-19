const alchemicalReduction = (data) => {
    let reducedData = new Array(12000); // empirical value
    let pos = 0;

    for (let i = 0; i < data.length; i++) {
        // console.log(data.charCodeAt(i) ^ reducedData[pos - 1].charCodeAt(0));
        if (pos > 0 && Math.abs(data.charCodeAt(i) - reducedData[pos - 1].charCodeAt(0)) === 32) {
            pos--;
        } else {
            reducedData[pos] = data[i];
            pos++;
        }
    }

    return reducedData.slice(0, pos).join('');
};

module.exports = alchemicalReduction;
