# vulner
[![master status](https://github.com/mrl5/vulner/actions/workflows/rust-just.yaml/badge.svg?event=push)](https://github.com/mrl5/vulner/actions/workflows/rust-just.yaml)

Discover CVEs for software.

- **Use case 1)** as a [Funtoo Linux] user I want to have awareness about CVEs on my system
- **Use case 2)** as user I want to list CVEs for given package

## DISCLAIMER

Running `vulner scan` doesn't guarantee that all CVEs present on your system will be
detected. It tries to map packages installed by the portage (funtoo package
manager) to a set of known NVD CPEs. It is possible that not all packages will
be successfully tagged.

For more info about false negatives and false positives check
[docs/CAVEATS.md](docs/CAVEATS.md)


## Examples

Check out [docs/COOKBOOK.md](docs/COOKBOOK.md)


## CVEs, CPEs, WTFs
Check this example: https://nvd.nist.gov/products/cpe/search/results?namingFormat=2.3&keyword=openssh

Notice how easy is to list all [CVE]s for given [CPE]. Using [CPE]s allows you
to have reliable vulnerability tracker.


## Howto build
```bash
$ git submodule update --init
$ cargo build --release && cargo install --path crates/cli/
```
or you can use `just` - fancy `make` replacement (check out
https://github.com/casey/just#packages)
```bash
$ just init build install check-runtime-deps
```
### Reminder
be sure to either add `~/.cargo/bin` to your PATH to be able to run the installed
binaries or to symlink `~/.cargo/bin/vulner` to some place covered by PATH


## Howto run
```bash
$ ./scripts/check-runtime-deps.sh
$ vulner --help
$ RUST_LOG=debug vulner sync
$ RUST_LOG=info vulner scan
```


## Why `vulner` needs python at runtime?

Because of reasons described in
[0001-runtime-python-dependencies.md](crates/cpe-tag/docs/adr/0001-runtime-python-dependencies.md)
ADR.


[Funtoo Linux]: https://www.funtoo.org/
[CVE]: https://nvd.nist.gov/vuln
[CPE]: https://nvd.nist.gov/products/cpe
