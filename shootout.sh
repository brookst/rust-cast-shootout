#!/bin/bash

readme="README.rst"
cat > ${readme} <<END
=====================
\`Rust-Cast-Shootout\`_
=====================

Learning how to cast a ``Float`` into a ``u8``. See the \`Reddit thread\`_ for more information.

The original implementation was written by me for use in \`Rust-Trace\`_. The min/max approach was suggested by \`arthurprs\`_ and he also genericised the original implementation. The iterator based approach was suggested by \`thiez\`_.

.. _Reddit thread: http://www.reddit.com/r/rust/comments/2moc2u/float_to_u8_conversion_help
.. _Rust-Trace: https://github.com/brookst/rust-trace
.. _arthurprs: http://www.reddit.com/user/arthurprs
.. _thiez: http://www.reddit.com/user/thiez

.. image:: https://travis-ci.org/brookst/rust-cast-shootout.svg?branch=master
    :target: https://travis-ci.org/brookst/rust-cast-shootout

Benchmarks
----------

The current version gives the following timings::

END

cmd="cargo bench"
echo "> ${cmd}" | tee shootout.log
${cmd} | grep --line-buffered -v '\.\.\.\ ignored' | tee -a shootout.log
sed -e's/^/    /' shootout.log >> ${readme}

cat >> ${readme} <<END

Generating
----------

This readme is generated from the ``shootout.sh`` bash script.

.. _Rust-Cast-Shootout: https://github.com/brookst/rust-cast-shootout
END
