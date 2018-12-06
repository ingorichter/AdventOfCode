import test from 'ava';
const overlappingClaims = require('./OverlappingClaims');
const notOverlappingClaim = require('./OverlappingClaims-2');

test('should calculate how many sqaure inches of fabric are withing two or more claims', t => {
    const claims = [
	'#1 @ 1,3: 4x4',
	'#2 @ 3,1: 4x4',
	'#3 @ 5,5: 2x2'
    ];

    t.plan(1);

    t.is(4, overlappingClaims(claims));
});

test('should calculate how many sqaure inches of fabric are withing two or more claims', t => {
    const claims = [
	'#1196 @ 835,510: 29x14',
	'#1197 @ 427,904: 23x25',
	'#1198 @ 327,956: 18x27',
	'#1199 @ 168,609: 16x23'
    ];

    t.plan(1);

    t.is(0, overlappingClaims(claims));
});

// Part 2
test('should calculate which claim does not overlap', t => {
    const claims = [
	'#1 @ 1,3: 4x4',
	'#2 @ 3,1: 4x4',
	'#3 @ 5,5: 2x2'
    ];

    t.plan(1);

    t.is('#3', notOverlappingClaim(claims));
});
