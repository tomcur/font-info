<div align="center">

# font-enumeration 🗚

**Enumerate fonts using system libraries**

![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)
[![Crates.io](https://img.shields.io/crates/v/font-enumeration.svg)](https://crates.io/crates/font-enumeration)
[![Build status](https://github.com/tomcur/font-info/workflows/CI/badge.svg)](https://github.com/tomcur/font-info/actions)

</div>

This is a cross-platform library for enumerating system fonts.

Supported platforms:

- Unix-like (Fontconfig)
- Windows (DirectWrite; **untested**)
- MacOS (Core Text; **untested**)

## Features and alternatives

This library is for very simple uses, where you're only interested in listing
installed fonts, perhaps filtering by family name. The listed fonts include
family and font name, file path, and some limited font attributes (style,
weight and stretch). It's unlikely this library will grow much beyond this
feature set, and its dependency tree will remain small.

Consider using [Fontique](https://crates.io/crates/fontique) or
[font-kit](https://crates.io/crates/font-kit) for features like font matching
and fallback.
