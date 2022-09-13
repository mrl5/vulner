# vulner
![GitHub commits since latest release (by SemVer)](https://img.shields.io/github/commits-since/mrl5/vulner/latest)
![GitHub last commit](https://img.shields.io/github/last-commit/mrl5/vulner)
[![cargo security audit](https://github.com/mrl5/vulner/actions/workflows/cargo-audit.yaml/badge.svg)](https://github.com/mrl5/vulner/actions/workflows/cargo-audit.yaml)
[![build status](https://github.com/mrl5/vulner/actions/workflows/build.yaml/badge.svg?event=push)](https://github.com/mrl5/vulner/actions/workflows/build.yaml)
[![tests status](https://github.com/mrl5/vulner/actions/workflows/tests.yaml/badge.svg?event=push)](https://github.com/mrl5/vulner/actions/workflows/tests.yaml)
[![linter status](https://github.com/mrl5/vulner/actions/workflows/linter.yaml/badge.svg?event=push)](https://github.com/mrl5/vulner/actions/workflows/linter.yaml)
![GitHub license](https://img.shields.io/github/license/mrl5/vulner)

Discover CVEs for software.

- **Use case 1)** as a [Funtoo Linux] user I want to have awareness about CVEs on my system
- **Use case 2)** as user I want to list CVEs for given package
- **Use case 3)** as a [Gentoo Linux] user I want to have awareness about CVEs on my system
- **Use case 4)** as a [Funtoo Linux] maintainer I want to scan all packages in kit for CVEs
- **Use case 5)** as a [Funtoo Linux] maintainer I want to scan all meta-repo for CVEs
- **Use case 6)** as a [Funtoo Linux] user I want to list bug tracker security
  vulnerability tickets that are not fixed


## API keys

For better user experience consider using API keys:
* [NVD API Key](https://nvd.nist.gov/developers/request-an-api-key)

More details in [COOKBOOK.md](docs/COOKBOOK.md#using-api-keys)


## DISCLAIMER

Running `vulner scan` doesn't guarantee that all CVEs present on your system will be
detected. It tries to map packages installed by the [portage] to a set of known
NVD CPEs. It is possible that not all packages will be successfully tagged.

For more info about false negatives and false positives check
[docs/CAVEATS.md](docs/CAVEATS.md)


## Examples

Check out [docs/COOKBOOK.md](docs/COOKBOOK.md)


## CVEs, CPEs, WTFs
Check this example: https://nvd.nist.gov/products/cpe/search/results?namingFormat=2.3&keyword=openssh

Notice how easy is to list all [CVE]s for given [CPE]. Using [CPE]s allows you
to have reliable vulnerability tracker.


## Howto build and install
You can find ebuild in [ebuilds/](./ebuilds) (it's also available in [funtoo
security-kit](https://github.com/funtoo/security-kit/tree/1.4-release/app-admin/vulner))
...

... or you can use `make`
```console
make install
```


## Howto run

```console
./scripts/check-runtime-deps.sh
vulner --help
RUST_LOG=debug vulner sync
RUST_LOG=info vulner scan -o ~/vulner/scan-results
```


## Why `vulner` needs python at runtime?

Because of reasons described in
[0001-runtime-python-dependencies.md](crates/cpe-tag/docs/adr/0001-runtime-python-dependencies.md)
ADR.


[Funtoo Linux]: https://www.funtoo.org/
[Gentoo Linux]: https://www.gentoo.org/
[CVE]: https://nvd.nist.gov/vuln
[CPE]: https://nvd.nist.gov/products/cpe
[portage]: https://en.wikipedia.org/wiki/Portage_(software)
