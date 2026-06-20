# scarf

[![Crates.io Version](https://img.shields.io/crates/v/scarf)](https://crates.io/crates/scarf)
[![Crates.io License](https://img.shields.io/crates/l/scarf)](https://crates.io/crates/scarf)


A suite of tools for interacting with (System)Verilog hardware designs,
fully compliant with [1800-2023](https://ieeexplore.ieee.org/document/10458102)

This project is currently under initial development - stay tuned!

## Packages

Scarf is composed of many children projects to separate complexity and use cases

### `scarf-parser`

[![Crates.io Version](https://img.shields.io/crates/v/scarf-parser)](https://crates.io/crates/scarf-parser)
[![docs.rs](https://img.shields.io/docsrs/scarf-parser)](https://docs.rs/crate/scarf-parser/latest)
[![Crates.io License](https://img.shields.io/crates/l/scarf-parser)](https://crates.io/crates/scarf-parser)

A complete preprocessor and parser for SystemVerilog source text, forming an CST as defined by `scarf-syntax`

### `scarf-python`

[![Crates.io Version](https://img.shields.io/crates/v/scarf-python)](https://crates.io/crates/scarf-python)
[![PyPI Version](https://img.shields.io/pypi/v/scarf_python)](https://pypi.org/project/scarf_python)
[![docs.rs](https://img.shields.io/docsrs/scarf-python)](https://docs.rs/crate/scarf-python/latest)
[![Crates.io License](https://img.shields.io/crates/l/scarf-python)](https://crates.io/crates/scarf-python)

Python bindings for the `scarf` SystemVerilog tools

### `scarf-syntax`

An object definition of a SystemVerilog CST

[![Crates.io Version](https://img.shields.io/crates/v/scarf-syntax)](https://crates.io/crates/scarf-syntax)
[![docs.rs](https://img.shields.io/docsrs/scarf-syntax)](https://docs.rs/crate/scarf-syntax/latest)
[![Crates.io License](https://img.shields.io/crates/l/scarf-syntax)](https://crates.io/crates/scarf-syntax)