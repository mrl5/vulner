# vulner

*Use case 1)* as a [Funtoo Linux] user I want to have awareness about CVEs on my system

**DISCLAIMER**

Running `vulner` doesn't guarantee that all CVEs present on your system will be
detected. It tries to map packages installed by the portage (funtoo package
manager) to a set of known NVD CPEs. It is possible that not all packages will
be successfully tagged.


## CVEs, CPEs, WTFs
Check this example: https://nvd.nist.gov/products/cpe/search/results?namingFormat=2.3&keyword=openssh

Notice how easy is to list all [CVE]s for given [CPE]. Using [CPE]s allows you
to have reliable vulnerability tracker.


## Howto build
```bash
$ cargo build --release
```
or you can use just (if you don't have it yet then here is how to install
https://github.com/casey/just#packages)
```bash
$ just build
```


[Funtoo Linux]: https://www.funtoo.org/
[CVE]: https://nvd.nist.gov/vuln
[CPE]: https://nvd.nist.gov/products/cpe
