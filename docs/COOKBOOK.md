# `vulner` cookbook

```bash
vulner --help
```

## Content
- [Scanning Funtoo Linux system for CVEs](#scanning-funtoo-linux-system-for-cves)
- [Scanning packages in Funtoo Linux kit for CVEs](#scanning-packages-in-funtoo-linux-kit-for-cves)
- [Scanning Funtoo Linux meta-repo for CVEs](#scanning-funtoo-linux-meta-repo-for-cves)
- [Listing CVEs for given packages](#listing-cves-for-given-packages)
- [Printing known exploited vulnerabilities catalog](#printing-known-exploited-vulnerabilities-catalog)
- [Using API keys](#using-api-keys)


## Scanning Funtoo Linux system for CVEs
```bash
export VULNER_FEED_DIR=$HOME/vulner/feeds/json
export VULNER_OUT_DIR=$HOME/vulner/scan-results

vulner sync
RUST_LOG=warn vulner scan
```
Results in:
```bash
$ tree ~/vulner/scan-results/
```
```
└── 2022-04-05UTC
    └── 21:13:11Z
        ├── app-emulation
        │   ├── containerd-1.5.9.txt
        │   └── qemu-6.2.0-r3.txt
        └── x11-terms
            └── xterm-346.txt
```
Report for particular package:
```bash
$ cat ~/vulner/scan-results/2022-01-30UTC/*/app-emulation/*containerd*.txt | jq '.'
```
```
{
  "id": "CVE-2021-41103",
  "is_known_exploited_vuln": false,
  "related_cpe": "cpe:2.3:a:linuxfoundation:containerd:1.5.5:*:*:*:*:*:*:*",
  "description": "A bug was found in containerd where container root directories and some plugins had insufficiently restricted permissions, allowing otherwise unprivileged Linux users to traverse directory contents and execute programs.",
,
  "urls": [
    "https://nvd.nist.gov/vuln/detail/CVE-2021-41103",
    "https://github.com/containerd/containerd/commit/5b46e404f6b9f661a205e28d59c982d3634148f8",
    "https://github.com/containerd/containerd/security/advisories/GHSA-c2h3-6mxw-7mvq",
    "https://lists.fedoraproject.org/archives/list/package-announce@lists.fedoraproject.org/message/ZNFADTCHHYWVM6W4NJ6CB4FNFM2VMBIB/",
    "https://lists.fedoraproject.org/archives/list/package-announce@lists.fedoraproject.org/message/B5Q6G6I4W5COQE25QMC7FJY3I3PAYFBB/",
    "https://www.debian.org/security/2021/dsa-5002"
  ]
}
```


## Scanning packages in Funtoo Linux kit for CVEs
```bash
kit="gnome-kit"

export VULNER_FEED_DIR=$HOME/vulner/feeds/json
export VULNER_OUT_DIR=$HOME/vulner/${kit}-scan-results

vulner sync
vulner scan --pkg-dir /var/git/meta-repo/kits/${kit}/
```


## Scanning Funtoo Linux meta-repo for CVEs
```bash
export VULNER_FEED_DIR=$HOME/vulner/feeds/json
export VULNER_OUT_DIR=$HOME/vulner/${kit}-scan-results

vulner sync
vulner scan --pkg-dir /var/git/meta-repo/ --recursive
```


## Listing CVEs for given packages

### Example 1
```bash
vulner sync
vulner cpe '[{"name":"lua", "version":"5.3.5-r1"}]' | vulner cve --summary
```

### Example 2
**NOTE** this example requires third party package to be present on your OS -
[jq](https://stedolan.github.io/jq/) (for pretty output)

```bash
RUST_LOG=debug vulner sync && echo '
[
  {
    "name": "busybox",
    "version": "1.29.3"
  },
  {
    "name": "busybox",
    "version": "1.31.0"
  },
  {
    "name": "libxml2",
    "version": "2.9.10-r5"
  }
]
' | jq -c '.' | vulner cpe | vulner cve --summary --check-known-exploited
```
example produces:
```
[2022-01-29T14:22:27Z DEBUG vulner] initialized logger
[2022-01-29T14:22:27Z INFO  security_advisories::service] fetching CPE match feed checksum ...
[2022-01-29T14:22:27Z DEBUG reqwest::connect] starting new connection: https://nvd.nist.gov/
[2022-01-29T14:22:27Z INFO  vulner::utils] computing checksum of "/tmp/vulner/feeds/json/nvdcpematch-1.0.json" ...
[2022-01-29T14:22:28Z DEBUG reqwest::async_impl::client] response '200 OK' for https://nvd.nist.gov/feeds/json/cpematch/1.0/nvdcpematch-1.0.meta
CPE match feed is up to date, available in "/tmp/vulner/feeds/json/nvdcpematch-1.0.json"
{"id":"CVE-2020-7595","is_known_exploited_vuln":false,"desc":{"description_data":[{"lang":"en","value":"xmlStringLenDecodeEntities in parser.c in libxml2 2.9.10 has an infinite loop in a certain end-of-file situation."}]},"impact":{"acInsufInfo":false,"cvssV2":{"accessComplexity":"LOW","accessVector":"NETWORK","authentication":"NONE","availabilityImpact":"PARTIAL","baseScore":5,"confidentialityImpact":"NONE","integrityImpact":"NONE","vectorString":"AV:N/AC:L/Au:N/C:N/I:N/A:P","version":"2.0"},"exploitabilityScore":10,"impactScore":2.9,"obtainAllPrivilege":false,"obtainOtherPrivilege":false,"obtainUserPrivilege":false,"severity":"MEDIUM","userInteractionRequired":false}}
```


## Printing known exploited vulnerabilities catalog
```bash
vulner kev | jq '.' > known-exploited-vulnerabilities.json
```
Results in:
```bash
$ head known-exploited-vulnerabilities.json

{
  "catalogVersion": "2022.02.25",
  "count": 383,
  "dateReleased": "2022-02-25T09:45:26.2626Z",
  "title": "CISA Catalog of Known Exploited Vulnerabilities",
  "vulnerabilities": [
    {
      "cveID": "CVE-2021-27104",
      "dateAdded": "2021-11-03",
      "dueDate": "2021-11-17",
      "product": "FTA",
      "requiredAction": "Apply updates per vendor instructions.",
      "shortDescription": "Accellion FTA 9_12_370 and earlier is affected by OS
command execution via a crafted POST request to various admin endpoints.",
      "vendorProject": "Accellion",
      "vulnerabilityName": "Accellion FTA OS Command Injection Vulnerability"
    },
```


## Using API keys
```bash
export NVD_API_KEY=your-api-key
```
or change this line in local config:
```bash
grep -n nvd_api_key ~/.config/vulner/vulner.toml
```
