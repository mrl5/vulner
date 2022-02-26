## [0.3.1](https://github.com/mrl5/vulner/compare/v0.3.0...v0.3.1) (2022-02-26)


### Bug Fixes

* adapter for apache http server ([e28c04a](https://github.com/mrl5/vulner/commit/e28c04ac1b08ccd6e2c89e692e421ba97dcaa970))



# [0.3.0](https://github.com/mrl5/vulner/compare/v0.2.0...v0.3.0) (2022-02-22)


### Features

* **cli:** allow defining pkg dir for scan [[#20](https://github.com/mrl5/vulner/issues/20)] ([057eab8](https://github.com/mrl5/vulner/commit/057eab85892d8570fbc06bfda71b3265de9045c9))



# [0.2.0](https://github.com/mrl5/vulner/compare/v0.1.0...v0.2.0) (2022-02-14)


### Bug Fixes

* **cli:** dont stop but log on http error ([b9494f5](https://github.com/mrl5/vulner/commit/b9494f506163997d719139ec03e887350f2f3f0c))
* **cpe-tag:** more verbose error when cpe match feed not found ([52f4061](https://github.com/mrl5/vulner/commit/52f4061d2e16dc2ea03db40905ce6e060876af5b))
* **os-adapter:** gentoo flavor is gentoo ([0e42c43](https://github.com/mrl5/vulner/commit/0e42c439e632a44ba6c326538159a8ae1254a7ad))


### Features

* **cli:** richer scan reports ([23799cc](https://github.com/mrl5/vulner/commit/23799ccad50e578abbe7cf2850c756a3c913b857))
* **cli:** summary flag for cve command ([890f588](https://github.com/mrl5/vulner/commit/890f5887aec4290224347a3a2f7d6f9025630a91))
* **os-adapter:** support gentoo linux ([3fcfce8](https://github.com/mrl5/vulner/commit/3fcfce81f89f52e5cd415917aa1fb42bc953788e))



# 0.1.0 (2022-01-30)


### Features

* **cli:** allow piping (input from stdin) ([790c5f6](https://github.com/mrl5/vulner/commit/790c5f607ecf934474a6898d3f69f658b4838ecf))
* **cli:** init CLI ([78b0d8d](https://github.com/mrl5/vulner/commit/78b0d8d7790073080eb10616dcbdc81b23e4d07e))
* cpe - new command for returning valid and existing CPEs ([bac30f5](https://github.com/mrl5/vulner/commit/bac30f5da64479ac25b1007402fe05720d9675f8))
* cve - new command for listing CVEs for given CPEs ([5b6ca09](https://github.com/mrl5/vulner/commit/5b6ca095fc1de36163d3ba36bd8f929a8144718c))
* **lib:** reuse python lib for grep patterns ([2dfcfcf](https://github.com/mrl5/vulner/commit/2dfcfcf703a1fc87eb3aca32b65281f486240181))
* scan - new command for CVE scanning ([5a96512](https://github.com/mrl5/vulner/commit/5a9651281da9cffad2432fc9d9f4164bf417f799))
* sync - new command for fetching NVD CPE match feed ([55e053d](https://github.com/mrl5/vulner/commit/55e053d129738293de7bb5714ead197d28330759))


### Performance Improvements

* **cpe:** deserialize from string only once ([47fe076](https://github.com/mrl5/vulner/commit/47fe0760a55cfc28b1804cd050455cee98c7cdcb))



