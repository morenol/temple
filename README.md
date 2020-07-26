# temple
[![Coverage Status](https://coveralls.io/repos/github/morenol/temple/badge.svg?branch=master)](https://coveralls.io/github/morenol/temple?branch=master)
[![Actions Status](https://github.com/morenol/temple/workflows/CI/badge.svg)](https://github.com/morenol/temple/actions)



**TEMPL**ate **E**ngine. 

A jinja2-like template engine in [rust] inspired by  *[Jinja2Cpp]*.

[Jinja2Cpp]: https://github.com/jinja2cpp/jinja2cpp
[rust]: https://www.rust-lang.org

TODO:

- [ ] Statements:
  - [x] if/else/elif
  - [x] for (Partial implementation)
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
  - [x] default
  - [ ] dictsort
  - [x] escape
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
  - [x] string
  - [ ] striptags
  - [x] sum
  - [ ] title
  - [ ] tojson
  - [ ] truncate
  - [ ] unique
  - [x] upper
  - [ ] urlencode
  - [ ] urlize
  - [x] wordcount
  - [ ] wordwrap
  - [ ] xmlattr
- [ ] Custom filters
- [x] Use of context
  - [x] Basic context
  - [x] Several levels of context (Global and Local variables)
- [ ] Use of settings
