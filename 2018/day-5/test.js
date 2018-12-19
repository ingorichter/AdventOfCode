import test from 'ava';

const alchemicalReduction = require('./AlchemicalReduction');
const optimalReduction = require('./AlchemicalReduction-2');

test('should calculate the reduced sequence of a given polymer', t => {
    const polymer = 'dabAcCaCBAcCcaDA';

    t.plan(1);

    t.is('dabCBAcaDA', alchemicalReduction(polymer));
});

test('should calculate the reduced sequence of a given polymer without leaving nothing left', t => {
    const polymer = 'ingoOGNI';

    t.plan(1);

    t.is('', alchemicalReduction(polymer));
});

test('should calculate the reduced sequence of a given polymer variation', t => {
    const polymer = 'alLesStvOoVT';

    t.plan(1);

    t.is('ae', alchemicalReduction(polymer));
});

test('should calculate the reduced sequence of a longer polymer', t => {
    const polymer = 'JjDdoODgGdNnIiTtJjFfnsSNsVUaAugGpPLAalvVHRrhvhoOZzQqHLhjJSsOoJjSaAsrqkBbKQRtTHlSqQeEioOICcUuHmMhPVvJjpJjDdhHUuSLllswWSLsJjFxXbBVvaAfuUoaFPpPpWwZzfAOrKkRxXnNqIJjKkiJCcjVvrRTIHhNnipPWwvVtLlBbMmrReEAaQwWlLSLldjgAattTTGZzJqUnNVvuQlLDsEDdLlzuWwUZDdCceFRrNpPnRrfFGLlgXWwxiAaILkKDYydlYqQytdDTtNnMFCcBbTQqtfsSmTaAyCcYhSsGgkDgBbfFGdKPpmMjuUJHfkuYyUXnNxDdyYlLOoQqmMuUuUPpYrRSrRnEeNOosyYSosSNMmnOOojJsFfYyEuUhMmTtzZQLhHFflmMkKJBMmbDdjoPpxXLwWlBbOQhHIiLlXxqqxXHjoOmMJdlLYyAariqQITtRHhoODiIzZqZzxXFfQlQOoqLtpPpeEPNLlncldDLCTIieEdZzoaAOWwIihHBbhHyIiIiYVvRrxXqQiIQDzZdqtTLlPccCCpPpHhWwiYyoOIBbVfFfhHFvWoYyfpPaAFbwWPTDdtrRpDGfXDdxFQqtTxXfFgxEZzZzeQqBbZzXWBbwdzZByYAuUgGaTKkcCtnNOlkKxXLFrRMmUiIFEefcUiIuCgGOoIioUujeEIiJjxXiIyYGrRyYoOJjgeRrEEeEelLKkbBzBbZJFTtfbBAkKqQASsGgaFOoIipqtTQPfxFfTrRaABYOYZzyoymMMmbAfzLKkEeCcxXlXxjpPeEJVvzZZmMIihHoOFsSamRrMhHBqQHhBPpbbBtTFAantTNnNfoObsqQTtSsyRXxrYSsSBrRHhRrAabPILlilPmBbZFfkHhuUTRrXxtKxXhHzMdDHLkeEfFKlPwFeEfWpDdOohoQqnNOpWwTNnPpoOtLyYPpTeEuUtzTrRtZBbeEBbIoeFfLYuUylOoEfFySsdDYOViIvisSmDdiINtdDWwTLUutTlPnNhyYHXxkKbBvVpJjqdDQnpPwWqQAaFgFfGDdecVvXIiGgzZxCGgZgGzWwqQqQwWyYnNmnNMtTUEeuEIVUoOooOzsHhCcSPpZHhOjNnYyOPpoGgUuJujaAGgXxJifRrFInNUuisSxXIKNxXnkPpTQqoBbOtuUdDgGrRbBgGUuIDdhHiHc';

    t.plan(1);

    t.is('kYEdWFUoAxTPmFIVHc', alchemicalReduction(polymer));
});

// part 2

test('should find the optimal reduction unit and return the length', t => {
    const polymer = 'dabAcCaCBAcCcaDA';

    t.plan(2);

    const optimzedPolymer = optimalReduction(polymer);
    t.is('daDA', optimzedPolymer);
    t.is(4, optimzedPolymer.length);
});
