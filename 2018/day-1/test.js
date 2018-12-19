import test from 'ava';
const chronalCalibration = require('./chronalCalibration');
const chronalCalibration2 = require('./chronalCalibration-2');

test('should calculate frequency from +1, -2, +3, +1', t => {
    const changes =
      `+1
       -2
       +3
       +1`;

    t.plan(1);

    t.is(3, chronalCalibration(changes));
});

test('should calculate frequency from +1, +1, -2', t => {
    const changes =
	  `+1
           +1
           -2`;

    t.plan(1);

    t.is(0, chronalCalibration(changes));
});

test('should calculate first duplicate frequency from +1, -2, +3, +1', t => {
      const changes =
        `+1
         -2
         +3
         +1`;

    t.plan(1);
    t.is(2, chronalCalibration2(changes));
});

test('should calculate first duplicate frequency from +3, +3, +4, -2, -4', t => {
    const changes = `+3
                     +3
                     +4
                     -2
                     -4`;

    t.plan(1);
    t.is(10, chronalCalibration2(changes));
});

test('should calculate first duplicate frequency from -6, +3, +8, +5, -6', t => {
    const changes = `-6
                     +3
                     +8
                     +5
                     -6`;

    t.plan(1);
    t.is(5, chronalCalibration2(changes));
});

test('should calculate first duplicate frequency from +7, +7, -2, -7, -4', t => {
    const changes = `+7
                     +7
                     -1
                     -7
                     -4`;

    t.plan(1);
    t.is(6, chronalCalibration2(changes));
});

