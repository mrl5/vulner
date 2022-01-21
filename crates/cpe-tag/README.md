# CPE tag

Generates valid NVD CPEs.

> CPE is a structured naming scheme for information technology systems,
> software, and packages. Based upon the generic syntax for Uniform Resource
> Identifiers (URI), CPE includes a formal name format, a method for checking
> names against a system, and a description format for binding text and tests
> to a name.

(from: https://nvd.nist.gov/products/cpe)


## About

`cpe-tag` provides interface that can be used for obtaining **A VALID AND
EXISTING** CPE for given package, software or technology system.


## CVEs, CPEs, WTFs
Check this example: https://nvd.nist.gov/products/cpe/search/results?namingFormat=2.3&keyword=openssh

Notice how easy is to list all CVEs for given CPE. Using CPEs allows you to
have reliable vulnerability tracker.


## See also

* https://nvd.nist.gov/products/cpe
* The CPE 2.3 XML Schema: https://csrc.nist.gov/schema/cpe/2.3/cpe-dictionary_2.3.xsd
