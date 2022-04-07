# espeak-rs
Rust bindings for Espeak-NG exposed to Node via N-API

## Usage

```ts
import { EspeakAddon } from 'espeak-rs'

const espeak = EspeakAddon.default()
const phonemes = espeak.textToPhonemes('Hello world')
```
