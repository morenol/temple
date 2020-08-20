# temple
[![Coverage Status](https://coveralls.io/repos/github/morenol/temple/badge.svg?branch=master)](https://coveralls.io/github/morenol/temple?branch=master)
[![Actions Status](https://github.com/morenol/temple/workflows/CI/badge.svg)](https://github.com/morenol/temple/actions)



**TEMPL**ate **E**ngine. 

A jinja2-like template engine in [rust] inspired by  *[Jinja2Cpp]*.

## Goals
 * Easy-to-use public interface.
 * Shared template environment.
 * Conformance to Jinja2 specification.
 * Rich error reporting.
 * Shared template environment with templates cache support.

## Current Jinja2 support

* expressions. You can use almost every expression style: simple, filtered, conditional, and so on.
* filters that can be used  via '|' operator (default, first, last, length, max, min, abs, float, int, string, sum, round, capitalize, title, upper, wordcount, truncate and center).
* 'if' statement (with 'elif' and 'else' branches)
* 'for' statement (without 'else' branch and 'if' part support)
* 'with' statement
* 'include' statement
* space control and 'raw'/'endraw' blocks

[Jinja2Cpp]: https://github.com/jinja2cpp/jinja2cpp
[rust]: https://www.rust-lang.org

TODO:

- [ ] Statements:
  - [x] for (Partial implementation)
  - [ ] set
  - [ ] filter
  - [ ] extends
  - [x] include
  - [ ] macro
  - [ ] line statements
- [ ] expressions
  - [ ] left associative order in operations
  - [ ] if expressions
  - [ ] Accessors
    - [ ] Points (.) 
- [ ] Filters
  - [ ] attr
  - [ ] batch
  - [ ] dictsort
  - [ ] filesizeformat
  - [ ] forceescape
  - [ ] format
  - [ ] groupby
  - [ ] indent
  - [ ] list
  - [ ] map
  - [ ] pprint
  - [ ] random
  - [ ] reject
  - [ ] rejectattr
  - [ ] replace
  - [ ] reverse
  - [ ] safe
  - [ ] select
  - [ ] selectattr
  - [ ] slice
  - [ ] sort
  - [ ] striptags
  - [ ] tojson
  - [ ] unique
  - [ ] urlencode
  - [ ] urlize
  - [ ] wordwrap
  - [ ] xmlattr
- [ ] Custom filters
- [ ] Use of settings
