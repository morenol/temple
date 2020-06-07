# temple
[![Coverage Status](https://coveralls.io/repos/github/morenol/temple/badge.svg?branch=master)](https://coveralls.io/github/morenol/temple?branch=master)
[![Actions Status](https://github.com/morenol/temple/workflows/CI/badge.svg)](https://github.com/morenol/temple/actions)



**TEMPL**ate **E**ngine. 

A port of *[Jinja2Cpp]* written in [rust].

[Jinja2Cpp]: https://github.com/jinja2cpp/jinja2cpp
[rust]: https://www.rust-lang.org

TODO:

- [ ] Statements:
  - [x] if/else/elif
  - [ ] for
  - [ ] with
  - [ ] set
  - [ ] filter
  - [ ] extends
  - [ ] include
  - [ ] macro
  - [ ] line statements
- [ ] expressions
  - [x] math operations (+,-,*,/,//,%)
  - [x] string concat
  - [x] unary operators(not,-,+)
  - [x] diccionaries
  - [x] tuples
  - [ ] left associative order in operations
  - [x] logical operators (and,or)
  - [x] logical comparisons (<,<=,==,!=,>,>=)
  - [x] filters (|)
  - [ ] if expressions
  - [ ] Accessors
    - [x] Square brackets ([])
    - [ ] Points (.) 
  - [x] Identifiers 
- [ ] Filters
  - [x] abs
  - [ ] attr
  - [ ] batch
  - [x] capitalize
  - [ ] center
  - [ ] default
  - [ ] dictsort
  - [ ] escape
  - [ ] filesizeformat
  - [x] first
  - [x] float
  - [ ] forceescape
  - [ ] format
  - [ ] groupby
  - [ ] indent
  - [x] int
  - [x] last
  - [x] length
  - [ ] list
  - [x] lower
  - [ ] map
  - [x] max
  - [x] min
  - [ ] pprint
  - [ ] random
  - [ ] reject
  - [ ] rejectattr
  - [ ] replace
  - [ ] reverse
  - [ ] round
  - [ ] safe
  - [ ] select
  - [ ] selectattr
  - [ ] slice
  - [ ] sort
  - [ ] string
  - [ ] striptags
  - [x] sum
  - [ ] title
  - [ ] tojson
  - [ ] truncate
  - [ ] unique
  - [x] upper
  - [ ] urlencode
  - [ ] urlize
  - [ ] wordcount
  - [ ] wordwrap
  - [ ] xmlattr
- [ ] Custom filters
- [x] Raw-blocks
- [x] Comments
- [ ] Use of context
  - [x] Basic context
  - [ ] Several levels of context (Global and Local variables)
- [x] Whitespace control
- [ ] Use of settings
