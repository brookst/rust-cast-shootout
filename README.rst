=====================
`Rust-Cast-Shootout`_
=====================

Learning how to cast a Float into a u8. See the `Reddit thread`_ for more information.

The original implementation was written by me for use in `Rust-Trace`_. The min/max approach was suggested by `arthurprs`_ and he also genericised the original implementation. The iterator based approach was suggested by `thiez`_.

.. _Reddit thread: http://www.reddit.com/r/rust/comments/2moc2u/float_to_u8_conversion_help
.. _Rust-Trace: https://github.com/brookst/rust-trace
.. _arthurprs: http://www.reddit.com/user/arthurprs
.. _thiez: http://www.reddit.com/user/thiez

.. image:: https://travis-ci.org/brookst/rust-cast-shootout.svg?branch=master
    :target: https://travis-ci.org/brookst/rust-cast-shootout

Benchmarks
----------

The current version gives the following timings::

    > cargo bench bench
         Running target/release/rust-cast-shootout-132f7401037e4cf7
    
    running 20 tests
    test bench_generic_max    ... bench:         2 ns/iter (+/- 0)
    test bench_generic_over   ... bench:         2 ns/iter (+/- 0)
    test bench_generic_under  ... bench:         2 ns/iter (+/- 0)
    test bench_generic_values ... bench:         2 ns/iter (+/- 0)
    test bench_generic_zero   ... bench:         2 ns/iter (+/- 0)
    test bench_if_max         ... bench:         2 ns/iter (+/- 0)
    test bench_if_over        ... bench:         2 ns/iter (+/- 0)
    test bench_if_under       ... bench:         2 ns/iter (+/- 0)
    test bench_if_values      ... bench:         2 ns/iter (+/- 0)
    test bench_if_zero        ... bench:         2 ns/iter (+/- 0)
    test bench_iter_max       ... bench:         9 ns/iter (+/- 0)
    test bench_iter_over      ... bench:         9 ns/iter (+/- 0)
    test bench_iter_under     ... bench:         9 ns/iter (+/- 0)
    test bench_iter_values    ... bench:         2 ns/iter (+/- 0)
    test bench_iter_zero      ... bench:         9 ns/iter (+/- 0)
    test bench_minmax_max     ... bench:         8 ns/iter (+/- 0)
    test bench_minmax_over    ... bench:         8 ns/iter (+/- 0)
    test bench_minmax_under   ... bench:         8 ns/iter (+/- 0)
    test bench_minmax_values  ... bench:         2 ns/iter (+/- 0)
    test bench_minmax_zero    ... bench:         9 ns/iter (+/- 2)
    
    test result: ok. 0 passed; 0 failed; 0 ignored; 20 measured
    

Generating
----------

This readme is generated from the shootout.sh bash script.

.. _Rust-Cast-Shootout: https://github.com/brookst/rust-cast-shootout
