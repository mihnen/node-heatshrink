# Heatshrink compression for Node.js

[Heatshrink](https://github.com/atomicobject/heatshrink/tree/develop)'s [Rust implementation](https://crates.io/crates/heatshrink) binding library using `napi-rs`.

## Usage

```typescript
export function encodeSync(input: Buffer, windowSize: number, lookaheadSize: number): Buffer
export function decodeSync(input: Buffer, windowSize: number, lookaheadSize: number): Buffer
export function encode(input: Buffer, windowSize: number, lookaheadSize: number, signal?: AbortSignal | undefined | null): Promise<Buffer>
export function decode(input: Buffer, windowSize: number, lookaheadSize: number, signal?: AbortSignal | undefined | null): Promise<Buffer>
```

where:

1. `encode()` and `encodeSync()` are for compression; `decode()` and `decodeSync()` are for decompression.
2. `windowSize` can be 8-11, `lookaheadSize` is recommended to be the half of, or slightly less than half of the `windowSize`. 

## License

MIT
