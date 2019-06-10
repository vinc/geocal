Geocal
======

A command line tool displaying a calendar and ephemeris of a [geodate](geodate.org).


Installation
------------

First you need to install Rust:

    $ curl https://sh.rustup.rs -sSf | sh

Then you can install the latest stable version with cargo:

    $ cargo install geocal

Or the development version by fetching the git repository:

    $ git clone git://github.com/vinc/geocal.git
    $ cd geocal
    $ cargo install --path .


Usage
-----

Run this tool with a latitude, a longitude, and optionally a timestamp, and you
will get a calendar representation of a [geodate](geodate.org):

    $ geocal 51.1789 -1.8262 1403333333
    +-------------------------+
    | Date:        0114-05-24 |
    +-------------------------+
    | So Me Ve Te Ma Ju Sa Lu |
    | 00 01 02 03 04 05 06    |
    | 07 08 09 10 11 12 13 14 |
    | 15 16 17 18 19 20 21    |
    | 22 23 24 25 26 27 28 29 |
    +-------------------------+
    | Time:             27:76 |
    +-------------------------+

There's also an ephemeris option:

    $ geocal 51.1789 -1.8262 1403333333 --ephem
    +-------------------------+
    | Date:        0114-05-24 |
    +-------------------------+
    | So Me Ve Te Ma Ju Sa Lu |
    | 00 01 02 03 04 05 06    |
    | 07 08 09 10 11 12 13 14 |
    | 15 16 17 18 19 20 21    |
    | 22 23 24 25 26 27 28 29 |
    +-------------------------+
    | Time:             27:76 |
    +-------------------------+
    | Moonrise:         01:57 |
    | Sunrise:          15:46 |
    | Solstice:         44:61 |
    | Moonset:          58:86 |
    | Sunset:           84:53 |
    +-------------------------+

License
-------

Copyright (c) 2019 Vincent Ollivier. Released under the MIT License.
