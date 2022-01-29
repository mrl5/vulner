# vulner
[![master status](https://github.com/mrl5/vulner/actions/workflows/rust-just.yaml/badge.svg?event=push)](https://github.com/mrl5/vulner/actions/workflows/rust-just.yaml)

Discover CVEs for packages installed by the portage.

- **Use case 1)** as a [Funtoo Linux] user I want to have awareness about CVEs on my system
- **Use case 2)** as user I want to list CVEs for given package

**DISCLAIMER**

Running `vulner` doesn't guarantee that all CVEs present on your system will be
detected. It tries to map packages installed by the portage (funtoo package
manager) to a set of known NVD CPEs. It is possible that not all packages will
be successfully tagged.


## Examples

Check out [COOKBOOK](COOKBOOK.md)


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


## Howto run
```bash
$ ./scripts/check-runtime-deps.sh
$ vulner --help
$ RUST_LOG=debug vulner sync
$ vulner cpe \
    '[{"name": "busybox", "versions": [{"version": "1.29.3"}, {"version": "1.31.0"}]}, {"name":"libxml2", "versions":[{"version":"2.9.10-r5"}]}]' |
      vulner cve |
        jq -c ".result.CVE_Items[] | {id: .cve.CVE_data_meta.ID, desc: .cve.description}"
```


## Why `vulner` needs some python packages at runtime?

Because of reasons described in
[0001-runtime-python-dependencies.md](crates/cpe-tag/docs/adr/0001-runtime-python-dependencies.md)
ADR.


[Funtoo Linux]: https://www.funtoo.org/
[CVE]: https://nvd.nist.gov/vuln
[CPE]: https://nvd.nist.gov/products/cpe
