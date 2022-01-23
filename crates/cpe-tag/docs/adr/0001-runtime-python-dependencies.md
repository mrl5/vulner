# Runtime Python dependencies

Date: 2022-01-16


## Status

accepted


## Context
One of `vulner`'s usecases is
> as a Funtoo Linux user I want to have awareness about CVEs on my system

Using CPE as a representation of given software allows to track vulnerabilities
(CVEs) for that software in a reliable way.

The usecase is related to [this Funtoo Linux Optimization
Proposal](https://www.funtoo.org/FLOP:CPE_tagger). There is already
[metarepo-cpe-tag] repository that was developed with a purpose of implementing
mentioned FLOP so that later it can be included as a plugin for
[ego](https://github.com/funtoo/ego) (Funtoo's configuration and management
meta-tool)


## Decision

`cpe-tag` lib should reuse logic that allows finding CPEs for given software.
This logic shall be taken from [metarepo-cpe-tag] python codebase.


## Consequences

1. Consistency between `vulner` and `ego` (or any other Funtoo Linux tooling).
2. No need to maintain logic that converts packages into CPEs.
3. `vulner` binary needs CPython3 and subset of pypi packages used by
   [metarepo-cpe-tag] to be present at runtime.



[metarepo-cpe-tag]: https://github.com/mrl5/metarepo-cpe-tag
