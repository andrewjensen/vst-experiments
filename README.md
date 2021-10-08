# VST Experiments

Just a scratchpad for building VSTs in Rust, using the [vst](https://crates.io/crates/vst) crate.

## Usage

```bash
# Build a .vst bundle and copy it into the main directory - see the script for details
./package.sh
```

## Decibel conversion

Other tools:

- Element's volume sliders
  - shows 0 dB to -60 dB, on a logarithmic scale
  - also goes up to +12 dB
- Live's track volume sliders
  - shows 6 dB to -70 dB, on a logarithmic scale
  - shows 10% increments (-6, -12, ..., -60)
  - -70 dB becomes "-inf"
  - Pixel measurements
    - 0 to -6: 29 px (10% on dB scale, 12.5% visually)
    - 0 to -12: 56 px (20% on dB scale, 24% visually)
    - 0 to -18: 83 px
    - 0 to -30: 138 px (50% on dB scale, 59% visually)
    - 0 to -60: 232 px
