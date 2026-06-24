.. scarf_python documentation master file, created by
   sphinx-quickstart on Mon Jun 22 22:55:29 2026.
   You can adapt this file completely to your liking, but it should at least
   contain the root `toctree` directive.

Python Bindings for Scarf
==========================

To increase usability, `Scarf <https://github.com/Aidan-McNay/scarf>`_ provides
Python bindings to a limited subset of its functionality through the
`scarf_python <https://pypi.org/project/scarf_python/>`_ package. This allows
Python users to interact with SystemVerilog sources while leveraging the
existing (fast) tools written in Rust.

When crossing the FFI boundary, Rust's borrow checker can no longer
provide lifetime guarantees; as such, many data structures must be
cloned, and have associated ``From`` / ``Into`` implementations for
their reference-based Rust counterparts. This quickly becomes the
dominant factor in runtime; if speed/space usage becomes a concern,
native Rust applications should be considered instead.

.. warning::
    ``scarf_python`` is currently considered unstable. Expect API breaks
    between minor versions and during development.


.. toctree::
   :maxdepth: 2
   :caption: Contents:

