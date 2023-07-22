import test from 'ava'

import { decodeSync, encodeSync, encode, decode } from '../index.js'

test('Test sync', (t) => {
  const buf = Buffer.from([1, 1, 4, 5, 1, 4, 1, 9, 1, 9, 1, 1, 4, 5, 1, 4, 1, 9, 1, 9, 1, 1, 4, 5, 1, 4, 1, 9, 1, 9]);
  const encoded = encodeSync(buf, 11, 4);
  console.log('Encoded content:', encoded);
  const decoded = decodeSync(encoded, 11, 4);
  console.log('Decoded content:', decoded);

  t.true(Buffer.compare(buf, decoded) === 0)
})

test('Test async', async (t) => {
  const buf = Buffer.from([1, 1, 4, 5, 1, 4, 1, 9, 1, 9, 1, 1, 4, 5, 1, 4, 1, 9, 1, 9, 1, 1, 4, 5, 1, 4, 1, 9, 1, 9]);
  const encoded = await encode(buf, 11, 4);
  console.log('Encoded content:', encoded);
  const decoded = await decode(encoded, 11, 4);
  console.log('Decoded content:', decoded);

  t.true(Buffer.compare(buf, decoded) === 0)
})

