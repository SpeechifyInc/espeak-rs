# espeak-rs
Rust bindings for espeak-ng exposed to Node via N-API. The projects serves to provide improved phonemization performance by directly interacting with espeak instead of spawning a process for each phonemization request. Performance on a 3700x shows about 1ms/50 chars/thread. 

## Usage

```ts
import { EspeakAddon } from 'espeak-rs'

const espeak = EspeakAddon.default()
const phonemes = espeak.textToPhonemes('Hello world')
```

## Building

`docker-compose up`
