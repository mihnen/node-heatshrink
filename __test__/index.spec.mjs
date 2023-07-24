import test from 'ava'

import { decodeSync, encodeSync, encode, decode } from '../index.js'

test('Test sync w13 l4', (t) => {
  const buf = Buffer.from([1, 1, 4, 5, 1, 4, 1, 9, 1, 9, 1, 1, 4, 5, 1, 4, 1, 9, 1, 9, 1, 1, 4, 5, 1, 4, 1, 9, 1, 9]);
  const encoded = encodeSync(buf, 13, 4);
  console.log('Encoded content:', encoded);
  const decoded = decodeSync(encoded, 13, 4);
  console.log('Decoded content:', decoded);

  t.true(Buffer.compare(buf, decoded) === 0)
})

test('Test async w13 l4', async (t) => {
  const buf = Buffer.from([1, 1, 4, 5, 1, 4, 1, 9, 1, 9, 1, 1, 4, 5, 1, 4, 1, 9, 1, 9, 1, 1, 4, 5, 1, 4, 1, 9, 1, 9]);
  const encoded = await encode(buf, 13, 4);
  console.log('Encoded content:', encoded);
  const decoded = await decode(encoded, 13, 4);
  console.log('Decoded content:', decoded);

  t.true(Buffer.compare(buf, decoded) === 0)
})

test('Test sync w8 l4', (t) => {
  const buf = Buffer.from([1, 1, 4, 5, 1, 4, 1, 9, 1, 9, 1, 1, 4, 5, 1, 4, 1, 9, 1, 9, 1, 1, 4, 5, 1, 4, 1, 9, 1, 9]);
  const encoded = encodeSync(buf, 8, 4);
  console.log('Encoded content:', encoded);
  const decoded = decodeSync(encoded, 8, 4);
  console.log('Decoded content:', decoded);

  t.true(Buffer.compare(buf, decoded) === 0)
})

test('Test async w8 l4', async (t) => {
  const buf = Buffer.from([1, 1, 4, 5, 1, 4, 1, 9, 1, 9, 1, 1, 4, 5, 1, 4, 1, 9, 1, 9, 1, 1, 4, 5, 1, 4, 1, 9, 1, 9]);
  const encoded = await encode(buf, 8, 4);
  console.log('Encoded content:', encoded);
  const decoded = await decode(encoded, 8, 4);
  console.log('Decoded content:', decoded);

  t.true(Buffer.compare(buf, decoded) === 0)
})

test('Test sync w11 l4', (t) => {
  const buf = Buffer.from([1, 1, 4, 5, 1, 4, 1, 9, 1, 9, 1, 1, 4, 5, 1, 4, 1, 9, 1, 9, 1, 1, 4, 5, 1, 4, 1, 9, 1, 9]);
  const encoded = encodeSync(buf, 11, 4);
  console.log('Encoded content:', encoded);
  const decoded = decodeSync(encoded, 11, 4);
  console.log('Decoded content:', decoded);

  t.true(Buffer.compare(buf, decoded) === 0)
})

test('Test async w11 l4', async (t) => {
  const buf = Buffer.from([1, 1, 4, 5, 1, 4, 1, 9, 1, 9, 1, 1, 4, 5, 1, 4, 1, 9, 1, 9, 1, 1, 4, 5, 1, 4, 1, 9, 1, 9]);
  const encoded = await encode(buf, 11, 4);
  console.log('Encoded content:', encoded);
  const decoded = await decode(encoded, 11, 4);
  console.log('Decoded content:', decoded);

  t.true(Buffer.compare(buf, decoded) === 0)
})

const chineseString = "心上的人儿 有笑的脸庞 他曾在深秋 给我春光 心上的人儿 有多少宝藏 他能在黑夜 给我太阳 我不能够给谁夺走仅有的春光 我不能够让谁吹熄胸中的太阳 心上的人儿 你不要悲伤 愿你的笑容 永远那样 我不能够给谁夺走仅有的春光 我不能够让谁吹熄胸中的太阳 心上的人儿 你不要悲伤 愿你的笑容 永远那样";

test('Test sync w11 l4 chinese', (t) => {
  const buf = Buffer.from(chineseString);
  const encoded = encodeSync(buf, 11, 4);
  console.log('Encoded content len:', encoded.length);
  const decoded = decodeSync(encoded, 11, 4);
  console.log('Decoded content len:', decoded.length);

  t.true(Buffer.compare(buf, decoded) === 0)
})

test('Test async w11 l4 chinese', async (t) => {
  const buf = Buffer.from(chineseString);
  const encoded = await encode(buf, 11, 4);
  console.log('Encoded content len:', encoded.length);
  const decoded = await decode(encoded, 11, 4);  
  console.log('Decoded content len:', decoded.length);

  t.true(Buffer.compare(buf, decoded) === 0)
})

test('Test sync w10 l5 chinese', (t) => {
  const buf = Buffer.from(chineseString);
  const encoded = encodeSync(buf, 10, 5);
  console.log('Encoded content len:', encoded.length);
  const decoded = decodeSync(encoded, 10, 5);
  console.log('Decoded content len:', decoded.length);

  t.true(Buffer.compare(buf, decoded) === 0)
})

test('Test async w10 l5 chinese', async (t) => {
  const buf = Buffer.from(chineseString);
  const encoded = await encode(buf, 10, 5);
  console.log('Encoded content len:', encoded.length);
  const decoded = await decode(encoded, 10, 5);  
  console.log('Decoded content len:', decoded.length);

  t.true(Buffer.compare(buf, decoded) === 0)
})

test('Test sync w8 l4 chinese', (t) => {
  const buf = Buffer.from(chineseString);
  const encoded = encodeSync(buf, 8, 4);
  console.log('Encoded content len:', encoded.length);
  const decoded = decodeSync(encoded, 8, 4);
  console.log('Decoded content len:', decoded.length);

  t.true(Buffer.compare(buf, decoded) === 0)
})

test('Test async w8 l4 chinese', async (t) => {
  const buf = Buffer.from(chineseString);
  const encoded = await encode(buf, 8, 4);
  console.log('Encoded content len:', encoded.length);
  const decoded = await decode(encoded, 8, 4);  
  console.log('Decoded content len:', decoded.length);

  t.true(Buffer.compare(buf, decoded) === 0)
})

test('Test sync w13 l5 chinese', (t) => {
  const buf = Buffer.from(chineseString);
  const encoded = encodeSync(buf, 13, 5);
  console.log('Encoded content len:', encoded.length);
  const decoded = decodeSync(encoded, 13, 5);
  console.log('Decoded content len:', decoded.length);

  t.true(Buffer.compare(buf, decoded) === 0)
})

test('Test async w13 l5 chinese', async (t) => {
  const buf = Buffer.from(chineseString);
  const encoded = await encode(buf, 13, 5);
  console.log('Encoded content len:', encoded.length);
  const decoded = await decode(encoded, 13, 5);  
  console.log('Decoded content len:', decoded.length);

  t.true(Buffer.compare(buf, decoded) === 0)
})

test('Test sync w6 l3 chinese', (t) => {
  const buf = Buffer.from(chineseString);
  const encoded = encodeSync(buf, 6, 3);
  console.log('Encoded content len:', encoded.length);
  const decoded = decodeSync(encoded, 6, 3);
  console.log('Decoded content len:', decoded.length);

  t.true(Buffer.compare(buf, decoded) === 0)
})

test('Test async w6 l3 chinese', async (t) => {
  const buf = Buffer.from(chineseString);
  const encoded = await encode(buf, 6, 3);
  console.log('Encoded content len:', encoded.length);
  const decoded = await decode(encoded, 6, 3);  
  console.log('Decoded content len:', decoded.length);

  t.true(Buffer.compare(buf, decoded) === 0)
})
