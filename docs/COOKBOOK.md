# `vulner` cookbook

```bash
vulner --help
```

## Content
- [Scanning Funtoo Linux system for CVEs](#scanning-funtoo-linux-system-for-cves)
- [Listing CVEs for given packages](#listing-cves-for-given-packages)


## Scanning Funtoo Linux system for CVEs
```bash
export VULNER_FEED_DIR=$HOME/vulner/feeds/json
export VULNER_OUT_DIR=$HOME/vulner/scan-results

RUST_LOG=info vulner sync
RUST_LOG=warn vulner scan
```
Results in:
```bash
$ tree ~/vulner/scan-results/
```
```
└── 2022-01-30UTC
    └── 11:10:06Z
        ├── app-emulation
        │   ├── cpe:2.3:a:linuxfoundation:containerd:1.5.5:*:*:*:*:*:*:*.txt
        │   ├── cpe:2.3:a:linuxfoundation:runc:1.0.1:*:*:*:*:*:*:*.txt
        │   └── cpe:2.3:a:qemu:qemu:5.2.0:-:*:*:*:*:*:*.txt
        ├── dev-libs
        │   ├── cpe:2.3:a:openssl:openssl:1.1.1l:*:*:*:*:*:*:*.txt
        │   └── cpe:2.3:a:xmlsoft:libxml2:2.9.10:*:*:*:*:*:*:*.txt
        └── x11-terms
            └── cpe:2.3:a:twistedmatrix:twisted:18.7.0:*:*:*:*:*:*:*.txt
```
Report for particular package:
```bash
$ cat ~/vulner/scan-results/2022-01-30UTC/*/app-emulation/*containerd*.txt
```
```
CVE-2021-43816
CVE-2021-41103
```


## Listing CVEs for given packages
**NOTE** this example requires third party package to be present on your OS -
[jq](https://stedolan.github.io/jq/) (for pretty output)

```bash
RUST_LOG=debug vulner sync && echo '
[
  {
    "name": "busybox",
    "versions": [
      {
        "version": "1.29.3"
      },
      {
        "version": "1.31.0"
      }
    ]
  },
  {
    "name": "libxml2",
    "versions": [
      {
        "version": "2.9.10-r5"
      }
    ]
  }
]
' | jq -c '.' | vulner cpe | vulner cve | jq -c ".result.CVE_Items[] | \
    {
      id: .cve.CVE_data_meta.ID,
      desc: .cve.description,
      impact: .impact.baseMetricV2
    }"
```
example produces:
```
[2022-01-29T14:22:27Z DEBUG vulner] initialized logger
[2022-01-29T14:22:27Z INFO  security_advisories::service] fetching CPE match feed checksum ...
[2022-01-29T14:22:27Z DEBUG reqwest::connect] starting new connection: https://nvd.nist.gov/
[2022-01-29T14:22:27Z INFO  vulner::utils] computing checksum of "/tmp/vulner/feeds/json/nvdcpematch-1.0.json" ...
[2022-01-29T14:22:28Z DEBUG reqwest::async_impl::client] response '200 OK' for https://nvd.nist.gov/feeds/json/cpematch/1.0/nvdcpematch-1.0.meta
CPE match feed is up to date, available in "/tmp/vulner/feeds/json/nvdcpematch-1.0.json"
{"id":"CVE-2020-7595","desc":{"description_data":[{"lang":"en","value":"xmlStringLenDecodeEntities in parser.c in libxml2 2.9.10 has an infinite loop in a certain end-of-file situation."}]},"impact":{"acInsufInfo":false,"cvssV2":{"accessComplexity":"LOW","accessVector":"NETWORK","authentication":"NONE","availabilityImpact":"PARTIAL","baseScore":5,"confidentialityImpact":"NONE","integrityImpact":"NONE","vectorString":"AV:N/AC:L/Au:N/C:N/I:N/A:P","version":"2.0"},"exploitabilityScore":10,"impactScore":2.9,"obtainAllPrivilege":false,"obtainOtherPrivilege":false,"obtainUserPrivilege":false,"severity":"MEDIUM","userInteractionRequired":false}}
```
