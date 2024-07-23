<div align="center">

# font-metrics 🗚

**Load fonts and print their metrics**

![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)
[![Crates.io](https://img.shields.io/crates/v/font-metrics.svg)](https://crates.io/crates/font-metrics)
[![Build status](https://github.com/tomcur/font-metrics/workflows/CI/badge.svg)](https://github.com/tomcur/font-metrics/actions)

</div>

This is a cross-platform program that can parse font files and prints out
information such as metrics and font style. It can find fonts installed or your
system or you can point it to a specific font file.

Supported font files:
 - OpenType
 - TrueType

Supported platforms:
 - Linux
 - Windows (untested)
 - MacOS (untested)

## Usage

See `$ font-metrics --help` for CLI usage documentation. For example, to find
all fonts in the "Liberation Sans" font family on your system and print out
their metrics, run:

```bash
$ font-metrics --family-name "Liberation Sans"
-[ FONT 1 ]-------------------------------------------------
              Source: /path/to/share/fonts/truetype/LiberationSans-Bold.ttf
Font index in source: 0
              Weight: 700
               Style: normal
             Stretch: 1.00
         Glyph count: 2620
        Units per em: 2048
     Average advance: 1248
              Ascent: 1854
             Descent: 434
         Line height: 2288
             Leading: 67
      Capital height: 1409
          "x" height: 1082
    Stroke thickness: 215
    Underline offset: -2
    Strikeout offset: 530
-[ FONT 2 ]-------------------------------------------------
... etc
```

The previous command has human-readable output. To output as machine-readable
JSON, run:

```bash
$ font-metrics --family-name "Liberation Sans" --format json
```

You can print fonts' features and supported writing systems using the
`--print-features` and `--print-writing-systems` flags.

## Installation

Install using Cargo:

```bash
$ cargo install font-metrics
$ font-metrics --family-name "Liberation Sans"
```

Run using Nix flakes:

```bash
# Run ls
$ nix run github:tomcur/font-metrics -- --family-name "Liberation Sans"
```
