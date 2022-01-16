# For Contributors


Welcome to our project! Before you start contributing please get familiar at
least with [HOWTO contribute](#howto-contribute) section.

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD",
"SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL" in this document are to be
interpreted as described in [RFC 2119].


## Table of contents
* [HOWTO contribute](#howto-contribute)
* [Commit Message Format](#commit-message-format)
* [DEV CLI](#dev-cli)


## HOWTO contribute
1. To request changes, a user SHOULD log an issue on [the issue tracker].
2. The user or Contributor SHOULD write the issue by describing the problem they face or observe.
3. Before making changes Contributor SHOULD fork this project.
4. To submit a patch, a Contributor MUST create a pull request back to the project.
5. To submit a patch, a Contributor SHOULD reference the issue in the commit
   message.
6. To submit a patch, the commit message redacted by a Contributor SHOULD be
   compliant with [Conventional Commits
   specification].

(this section was inspired by [ZMQ C4] contract)


## Commit Message Format
We stick to [Conventional Commits specification] i.a. to generate changelog
automatically.

Here is a [list of valid
types](https://github.com/conventional-changelog/commitlint/tree/master/%40commitlint/config-conventional#type-enum)
and [user friendly description from
Angular](https://github.com/angular/angular/blob/7aea5256de55056e424f9c6d92ac1d6f38d3abee/CONTRIBUTING.md#commit-message-header).

If you need more examples check commits on master branch.


## DEV CLI
```bash
just init-dev
just lint
just test
```
if you don't have `just` yet then here is how to install
https://github.com/casey/just#packages

In case you're curious why `just` then check
https://vino.dev/blog/node-to-rust-day-2-cargo/



[RFC 2119]: https://datatracker.ietf.org/doc/html/rfc2119
[the issue tracker]: https://github.com/mrl5/vulner/issues
[Conventional Commits specification]: https://www.conventionalcommits.org/en/v1.0.0-beta.2/
[ZMQ C4]: https://rfc.zeromq.org/spec/42/
