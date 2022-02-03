# Caveats

## Content
- [False negatives](#false-negatives)
- [False positives](#false-positives)


## False negatives
False negative is a case where for some reason `vulner` didn't report any CVE
for given package although in database there are some CVEs associated with this
package.

Some of the possible reasons are listed below.

### Name mismatch
Package name in your package manager might be different than package name used
in NVD. E.g. `nodejs` -> `node.js`


### CVE is reserved but not disclosed yet
https://cve.mitre.org/blog/May102017_Why_is_a_CVE_entry_marked_as_RESERVED_when_a_CVE_ID_is_being_publicly_used.html

## False positives
False positive is a case where `vulner` reports a particular CVE however it is
not exploitable.

Some of the possible reasons are listed below.


### Version for which CVE is registered was patched already at the repository level
Let's say that there is an upstream patch for given CVE but there is still no
new version of package that contains this patch.

In such situation linux distributions usually publish patched revision of the
same upstream version.

### "Wizard knowledge" indicating it's not exploitable
One wizard once said
> Present doesn't mean active. And active doesn't mean exploitable.

It often involves highly sophisticated knowledge to specify conditions under
which vulnerability is exploitable. There are cases where although there is a
CVE, the vulnerability can't be exploited.

For example:
- vulnerability is exploitable only if given package was compiled with some [`USE`
  flag](https://wiki.gentoo.org/wiki/Handbook:AMD64/Working/USE): e.g. [this
  SELinux + sudo case](https://nvd.nist.gov/vuln/detail/CVE-2021-23240)
- vulnerability can be exploited only when package is configured in specific
  way: e.g. [this `sudo`
  case](https://www.sudo.ws/security/advisories/minus_1_uid/)
- vulnerability exists only for specific distribution: e.g. [Debian/OpenSSL Fiasco from 2006](https://research.swtch.com/openssl)
- not exploitable because of implementation of C standard library used by given
  OS: e.g. [polkit case for
  Solaris](https://twitter.com/0xdea/status/1486272634971209728)
- only client side of package is affected, where server side is not: e.g. [this OpenSSH example](https://nvd.nist.gov/vuln/detail/CVE-2020-14145)
