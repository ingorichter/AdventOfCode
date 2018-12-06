import test from 'ava';

const calculateChecksum = require('./InventoryManagementSystem');
const commonLetters = require('./InventoryManagementSystem-2');

test('should calculate checksum for a given list of box IDs', t => {
    const boxIDS = [
	'abcdef',
	'bababc',
	'abbcde',
	'abcccd',
	'aabcdd',
	'abcdee',
	'ababab'
    ];

    t.plan(1);

    t.is(12, calculateChecksum(boxIDS));
});

test('should return the common letters between 2 strings from a list of strings', t => {
    const boxIDS = [
	'abcde',
	'fghij',
	'klmno',
	'pqrst',
	'fguij',
	'axcye',
	'wvxyz'
    ];
    
    t.plan(1);
    t.is('fgij', commonLetters(boxIDS));
});
