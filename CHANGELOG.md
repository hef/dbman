# Changelog

## [0.123.2](https://github.com/hef/dbman/compare/v0.123.1...v0.123.2) (2024-03-10)


### Bug Fixes

* **deps:** update rust crate anyhow to 1.0.80 ([c67424e](https://github.com/hef/dbman/commit/c67424eae55cf2c8683581a9ab35edf340c0d06d))
* **deps:** update rust crate clap to 4.5.0 ([0858ee9](https://github.com/hef/dbman/commit/0858ee96f706db228ea8e6ffaabc8dd8c9734c20))
* **deps:** update rust crate clap to 4.5.1 ([85fda99](https://github.com/hef/dbman/commit/85fda99bb49d2f56100d40bfcfb10bd5ab773bbd))
* **deps:** update rust crate clap to 4.5.2 ([a779634](https://github.com/hef/dbman/commit/a7796348824aa1afcd75b6398a6c93c2069b0b77))
* **deps:** update rust crate env_logger to 0.11.3 ([c963d58](https://github.com/hef/dbman/commit/c963d58666742bf28e9b184c1a87b8fe2c29078f))
* **deps:** update rust crate k8s-openapi to 0.21.1 ([0e52d94](https://github.com/hef/dbman/commit/0e52d94809c4b59af6086abad61190f5419efb29))
* **deps:** update rust crate log to 0.4.21 ([8a27b2f](https://github.com/hef/dbman/commit/8a27b2f15ab380d4d5386b723f76cc0a144b23c3))
* **deps:** update rust crate serde to 1.0.197 ([fb871b8](https://github.com/hef/dbman/commit/fb871b8859b92621c1ce379f0c9494bf9bcf00a0))
* **deps:** update rust crate serde_json to 1.0.114 ([cc85eab](https://github.com/hef/dbman/commit/cc85eabbdc136e330123184ec62d3f3455d8ebc5))
* **deps:** update rust crate serde_yaml to 0.9.32 ([ca0fa9c](https://github.com/hef/dbman/commit/ca0fa9c80d38e798cd29b69ab3dff73d38adc14c))
* **deps:** update rust crate thiserror to 1.0.57 ([3dd43b1](https://github.com/hef/dbman/commit/3dd43b1501f60e974681ae409da148331fb7e73a))
* Fix helm chart default env object ([56b4c74](https://github.com/hef/dbman/commit/56b4c741146b2d25a67137661eb6052014cf3fd9))

## [0.123.1](https://github.com/hef/dbman/compare/v0.123.0...v0.123.1) (2024-02-05)


### Bug Fixes

* **helm:** adding operator and prelease tags for artifacthub ([116512c](https://github.com/hef/dbman/commit/116512ce5b6769ea8e2c7fb3991b6254d5d30c2c))
* **helm:** moving artifacthub metadata file ([9d99f93](https://github.com/hef/dbman/commit/9d99f93dd278bf3f78e682f9809128a9d2a51f61))

## [0.123.0](https://github.com/hef/dbman/compare/v0.122.1...v0.123.0) (2024-02-05)


### Features

* **artifacthub:** adding ownership info for artifacthub ([baf4e39](https://github.com/hef/dbman/commit/baf4e396dd455e325a02e108a042876488a200f4))
* **credentials:** detect if credentials have not changed ([052987c](https://github.com/hef/dbman/commit/052987c8b37e9ffac20d80b427b26d7585d05a3c))
* **helm:** adding artifactub ownership info ([fde8d4e](https://github.com/hef/dbman/commit/fde8d4e81d4492686b595fe385a925044ffcb199))


### Bug Fixes

* **cache:** fix cacheing xid ([246ffd6](https://github.com/hef/dbman/commit/246ffd601cf0a924d3002415c3f9f830b94395cc))
* **ci:** updating k8s-openapi and kube ([e50b35a](https://github.com/hef/dbman/commit/e50b35a6a597235fc3b241a78cba8905bffc3e68))
* **cred_cache:** removing all code that can panic ([88f12ab](https://github.com/hef/dbman/commit/88f12ab9b7bc96999a8e2788357645b4df74712a))
* **deps:** update rust crate actix-web to 4.5.1 ([75e3f7e](https://github.com/hef/dbman/commit/75e3f7e941b6d0c74d5b592847c064b351d273d8))
* **deps:** update rust crate env_logger to 0.11.1 ([bde095a](https://github.com/hef/dbman/commit/bde095a9fa2252f7e3b7d1a0c9e2249952424162))
* **deps:** update rust crate serde to 1.0.196 ([6ea8fe7](https://github.com/hef/dbman/commit/6ea8fe7760e1a838b238f2010844defe461f32cb))
* **deps:** update rust crate serde_json to 1.0.112 ([e0522ae](https://github.com/hef/dbman/commit/e0522aeb4dccb269165862422c2bd1ed50f2a237))
* **deps:** update rust crate serde_json to 1.0.113 ([d7dae00](https://github.com/hef/dbman/commit/d7dae0098536056ce0e58d4b809060332fb90ef4))
* **deps:** update rust crate serde_yaml to 0.9.31 ([db256ae](https://github.com/hef/dbman/commit/db256ae08bcb2a00c400ec77d0268504a16c143a))
* **doc:** adding artifacthub badge to readme ([3e3ac51](https://github.com/hef/dbman/commit/3e3ac51dd7c85538515dbc4c004ba0bf08582dad))

## [0.122.1](https://github.com/hef/dbman/compare/v0.122.0...v0.122.1) (2024-01-21)


### Bug Fixes

* **doc:** fix formatting in readme ([190b79d](https://github.com/hef/dbman/commit/190b79d942f8e94479c06856cbc08f53707dc3ae))
* **doc:** fix some missed snake_case to camelCase issues ([6f588bc](https://github.com/hef/dbman/commit/6f588bcc3569ecdeada4e79a016ee5b0d7d7f387))
* **doc:** updating docs on ownerRef and credentials ([dcda17b](https://github.com/hef/dbman/commit/dcda17b4e5cb6cbd21cf64ccdad6724e01f39e36))
* **owner:** check for out of date owner and update ([d03de00](https://github.com/hef/dbman/commit/d03de00314d067faee2b0bc4cd49931118a93ba1))
* **ownerRef:** don't try and overwrite owner heritage ([f60e514](https://github.com/hef/dbman/commit/f60e51450f8fc723e6b26cdb270d4c0ec8ca3939))

## [0.122.0](https://github.com/hef/dbman/compare/v0.121.8...v0.122.0) (2024-01-21)


### Features

* **db:** making setting a db owner optional ([d45e589](https://github.com/hef/dbman/commit/d45e58982403ce3611f7d8ffeaa4262ce9ba888a))
* **dbman:** enable pulling owner from other database cr ([02f0860](https://github.com/hef/dbman/commit/02f0860c3a57109c52e439c88f0fa4682dabbe71))


### Bug Fixes

* **ci:** only push helm chart on tag ([d11b42f](https://github.com/hef/dbman/commit/d11b42fb2aa09f402c437ef28f915e73c7621d4b))
* **code:** minor code cleanup, handle all errors ([6cfbcff](https://github.com/hef/dbman/commit/6cfbcff81884e3ceb6f6b1cddeac0ed6e581091d))
* **creds:** make missing credentials block not an error on database CR ([411b609](https://github.com/hef/dbman/commit/411b609fa48421c08150443aedf1291d26b90474))

## [0.121.8](https://github.com/hef/dbman/compare/v0.121.7...v0.121.8) (2024-01-20)


### Bug Fixes

* **ci:** don't use helm for crds. ([d77704d](https://github.com/hef/dbman/commit/d77704d5750dca275c973ba49349a119214149a2))

## [0.121.7](https://github.com/hef/dbman/compare/v0.121.6...v0.121.7) (2024-01-20)


### Bug Fixes

* **crd:** making sure only 1 crd version is set to storage ([1f37363](https://github.com/hef/dbman/commit/1f37363b85cc185c9f7638b4226c93db47b409b5))

## [0.121.6](https://github.com/hef/dbman/compare/v0.121.5...v0.121.6) (2024-01-20)


### Bug Fixes

* **crd:** combining crd versions again ([78cd7f2](https://github.com/hef/dbman/commit/78cd7f2c0c141b61865adb4f1c6bda705f2db014))

## [0.121.5](https://github.com/hef/dbman/compare/v0.121.4...v0.121.5) (2024-01-20)


### Bug Fixes

* **crd:** putting old crd versions back in ([801a8e0](https://github.com/hef/dbman/commit/801a8e0f25b52aca1fc3373f8c5f36b8435c0e14))

## [0.121.4](https://github.com/hef/dbman/compare/v0.121.3...v0.121.4) (2024-01-20)


### Bug Fixes

* **helm:** fixing escaping `{{` in helm chart ([a40805a](https://github.com/hef/dbman/commit/a40805ab4291d2b41c1d83003382972a6ca19eac))

## [0.121.3](https://github.com/hef/dbman/compare/v0.121.2...v0.121.3) (2024-01-20)


### Bug Fixes

* **crd:** fix crd gen ([16e379c](https://github.com/hef/dbman/commit/16e379c2ff26714908c74a491a890abdd81362e6))

## [0.121.2](https://github.com/hef/dbman/compare/v0.121.1...v0.121.2) (2024-01-20)


### Bug Fixes

* **crds:** add support for old crd versions ([6c724ea](https://github.com/hef/dbman/commit/6c724ea1696fcb95e9aafc1a7f3f9c35abfcd77e))

## [0.121.1](https://github.com/hef/dbman/compare/v0.121.0...v0.121.1) (2024-01-20)


### Bug Fixes

* **ci:** stop uploading helm chart early ([9be01ff](https://github.com/hef/dbman/commit/9be01ffc0680c9cc038c9953347766192aaec604))
* **ci:** updating cargo.lock ([27aa722](https://github.com/hef/dbman/commit/27aa722b06d57bc01e15cdbadfb3301a80e68a39))
* **helm:** fix crd issue ([8f3f9f6](https://github.com/hef/dbman/commit/8f3f9f6f80b7cd5bd5797d1e167451ee16459dab))

## [0.121.0](https://github.com/hef/dbman/compare/v0.120.0...v0.121.0) (2024-01-20)


### Features

* **helm:** add crds.enabled to helm chart ([5fb5c21](https://github.com/hef/dbman/commit/5fb5c21f140b5f0ecf09ef1ccab29cb6f951c5ab))

## [0.120.0](https://github.com/hef/dbman/compare/v0.119.0...v0.120.0) (2024-01-19)


### Features

* **doc:** documenting crd changes ([b3ded2e](https://github.com/hef/dbman/commit/b3ded2ef878bea5a5b339a933456fca494898c4d))


### Bug Fixes

* **deps:** update rust crate clap to 4.4.17 ([97774e0](https://github.com/hef/dbman/commit/97774e087a7754226682e875aae4277574687a71))
* **deps:** update rust crate clap to 4.4.18 ([67611bb](https://github.com/hef/dbman/commit/67611bb1f64876f8528874d469ad99d64b97e11a))
* **deps:** update rust crate env_logger to 0.10.2 ([27bab1b](https://github.com/hef/dbman/commit/27bab1b67af6e1418707e3411773cdb8e2dd30d3))
* **doc:** fixing crd change version ([762decd](https://github.com/hef/dbman/commit/762decd4e6987c1433576b9bbf5226cb75c3c88e))
* **test:** adding a recreate dropped database test ([ad57116](https://github.com/hef/dbman/commit/ad571161d70601f14545d8a40ae334945ca6e5d7))
* **test:** fixing database existance test  in extra_db test ([19d80e7](https://github.com/hef/dbman/commit/19d80e7dde38384bfbe4bf171887347adf79ec76))

## [0.119.0](https://github.com/hef/dbman/compare/v0.118.0...v0.119.0) (2024-01-14)


### Features

* **ci:** adding helm push ([25c2b2e](https://github.com/hef/dbman/commit/25c2b2e38a5fc7ce8b776a7a856e0c3a9caa8155))
* **ci:** adding major and major.miner version labels ([b1e6ae8](https://github.com/hef/dbman/commit/b1e6ae89af10dfa6a0790dc8cfc63b52f59ade3e))
* **ci:** adding version tagging ([4adad92](https://github.com/hef/dbman/commit/4adad9233cc7409db69812e95f2450711b66899d))
* **ci:** significant refactoring of integration test ([4aab6cc](https://github.com/hef/dbman/commit/4aab6cca789347d42434abe4092c11b9076e7b61))
* **crd:** switching to database server defined in crd ([4928111](https://github.com/hef/dbman/commit/4928111ae0eee099047fd846b33357127eb0d0cb))
* create release on release ([8720a0a](https://github.com/hef/dbman/commit/8720a0a123e28f565fe427e03fa1fb795c9a7cb0))
* **error:** clearer error message for DatabaseServer missing database ([e5baefb](https://github.com/hef/dbman/commit/e5baefb35d08487962001ef17528e1c7c460b763))
* **helm:** add env setting ([95add47](https://github.com/hef/dbman/commit/95add4778879683ce04e15f0a951c16a885a0fc2))
* **helm:** moving helm chart ([cc65dfa](https://github.com/hef/dbman/commit/cc65dfadee88cbf0109f7fb4a33c657f8745d17f))
* **heritage:** applying heritage to roles too ([fd5185d](https://github.com/hef/dbman/commit/fd5185de39ba3ac4e64e3a0af09316db82c18a09))
* **heritage:** some sanity checks for ownership are being added ([607a6a3](https://github.com/hef/dbman/commit/607a6a314249d6444f3cfa49917ea0450c7e0187))
* **rust:** upgrade rust again ([599cb56](https://github.com/hef/dbman/commit/599cb5610fad5df88aa42a8325cd56f819073c01))
* **test:** adding an integration test ([885ee70](https://github.com/hef/dbman/commit/885ee7012ef8b90a71fc0fa96a598e46a953a47e))


### Bug Fixes

* **build:** fixing build error ([316d114](https://github.com/hef/dbman/commit/316d1143f8bd3fc53385b06130392443e1ddd1e7))
* **build:** random code that needs mods ([7a332b3](https://github.com/hef/dbman/commit/7a332b31c13cf2837d6d46523684985313bf7f7d))
* **cargo:** adding changes from rustls-tls change ([3735adc](https://github.com/hef/dbman/commit/3735adc51dcbc0f83591cf6d47b453a5364c3a4a))
* **ci:** actually setting build-mode as output ([f6350bc](https://github.com/hef/dbman/commit/f6350bc10a2af85ed023264e6ee78728c36e2e42))
* **ci:** Add concurrency settings to workflows ([55c05c7](https://github.com/hef/dbman/commit/55c05c7de325a2c01b60e21273c03d1663d6cc6e))
* **ci:** add kind to rust tests action ([03d6fb3](https://github.com/hef/dbman/commit/03d6fb36345283dbe2d35376953ce1849f32058d))
* **ci:** addinb `v` to helm-push version ([5f67352](https://github.com/hef/dbman/commit/5f67352a7ce9dcffd559c0ffc8784b8957937040))
* **ci:** adding chef to dockerfile ([c531fa4](https://github.com/hef/dbman/commit/c531fa43eb7affc0fc2371a091cb225ce7e0297d))
* **ci:** adjusting docker metadata ([c7cceec](https://github.com/hef/dbman/commit/c7cceec1efc1b8b130b4edbaa327c5fff4c5ab82))
* **ci:** adjusting metadata output to explicitly include tags ([0f9e55c](https://github.com/hef/dbman/commit/0f9e55c5d7cf37fef6759e46d7c54bceb149b80d))
* **ci:** allow release to write github releases ([c4190cc](https://github.com/hef/dbman/commit/c4190ccaab57e7b1ebd3bc29ba75ae732091631b))
* **ci:** bumping helm versions as part of upgrade ([a3b8f67](https://github.com/hef/dbman/commit/a3b8f6778969f96f069d50947fa5bb25eaa2ff2f))
* **ci:** changing bsord/helm-push to a real version ([8f4af00](https://github.com/hef/dbman/commit/8f4af0031ca850d09f1dcfa253926833465dccec))
* **ci:** changing build profile for tagged versions ([0e6e1bb](https://github.com/hef/dbman/commit/0e6e1bbf9f8fd9441018f04fb70d1d7096f4900e))
* **ci:** Cut off "new release spam" to stargazers ([0ff9f96](https://github.com/hef/dbman/commit/0ff9f960c491c65064db074d4569ca0e7720c6f8))
* **ci:** disabling extra optimizations in for-distribution for now ([4df1075](https://github.com/hef/dbman/commit/4df10752ae5bd1ea37e91cfcd84150b90937b345))
* **ci:** doing automerges into develop branch ([e843ca1](https://github.com/hef/dbman/commit/e843ca126db03667c3e910dffc18ed04c94dbec9))
* **ci:** don't include dbman in git tag. it breaks docker ([d646517](https://github.com/hef/dbman/commit/d64651784fe35ecf981a8b32f85f3db3cc6e308c))
* **ci:** exclude release action from running on renovate branches ([e89b476](https://github.com/hef/dbman/commit/e89b4766268ac783370649c462db2137f03ae8ab))
* **ci:** fix helm template verison to not include a `v` ([e205bab](https://github.com/hef/dbman/commit/e205babe36dd6e2ceafae35133132b7efa636c21))
* **ci:** fixing chart-folder locaiton ([e94cc08](https://github.com/hef/dbman/commit/e94cc08b07f3b0178964d8febc5d96f4492ce986))
* **ci:** fixing for-distributtion for tagged builds ([43d3c04](https://github.com/hef/dbman/commit/43d3c049a13272693991a3a8af2f953c054fea4a))
* **ci:** fixing github-token arg to go-sematnic-release ([d826e73](https://github.com/hef/dbman/commit/d826e73b4a8b3c2164d3066bed7323b7afad5ffc))
* **ci:** fixing type enabled -&gt; enable ([1534be3](https://github.com/hef/dbman/commit/1534be3e0d2c77c9e812d67fd0999bb45fe78dca))
* **ci:** fixing version parsing ([5410f52](https://github.com/hef/dbman/commit/5410f526d238d8816c4758f0dfe78db1da875876))
* **ci:** get tag after creating tag ([47ec639](https://github.com/hef/dbman/commit/47ec6399e9811471d4dd862f5b972f1f1a1f7b79))
* **ci:** make build finish ([fe8ee60](https://github.com/hef/dbman/commit/fe8ee60e724e09e365f0924c739ba4a1973c8f74))
* **ci:** make incremental builds a little faster ([0ccfefa](https://github.com/hef/dbman/commit/0ccfefa36acf4873e9ada266bcc8ea397bd97019))
* **ci:** make kind-action use default kind cluster_name ([d7ef779](https://github.com/hef/dbman/commit/d7ef779091dfd10390d03cf79e51b24871f3dc19))
* **ci:** making build complete in a reasonable amount of time ([a4baf13](https://github.com/hef/dbman/commit/a4baf13b4b1775d45ccf16a80e98d86126050fe4))
* **ci:** making docker build faster ([bd43be2](https://github.com/hef/dbman/commit/bd43be206f520d4f8ac09564ff2fca287f8330b8))
* **ci:** only create real release on push to main ([f710332](https://github.com/hef/dbman/commit/f7103327266c6975d7a48916e915d06f96057b09))
* **ci:** remove for-distribution profile ([3ac1d4f](https://github.com/hef/dbman/commit/3ac1d4f51feb1d7ba27d25d99332817d64fe0f30))
* **ci:** removing provider from semrel ([c919b3d](https://github.com/hef/dbman/commit/c919b3d1c210569c17ead783b2e1ec04491db41d))
* **ci:** renovate should use default branch, which is now develop ([3924103](https://github.com/hef/dbman/commit/39241039af81a926a58ec3058e0c15d1f113a986))
* **ci:** running rust test on all branches, and all PRs ([1eed4a0](https://github.com/hef/dbman/commit/1eed4a019c5c7d052fcdd941299b631c6c4da1b0))
* **ci:** setting all checked in versions to 0.0.0 ([3b4e431](https://github.com/hef/dbman/commit/3b4e431faf20a823c30e3599dade86ce420e710a))
* **ci:** setting lto to thin to ci can finish ([67befea](https://github.com/hef/dbman/commit/67befeaad33d14c6598b6774138946fadca2effc))
* **ci:** speeding up docker build ([e9645fb](https://github.com/hef/dbman/commit/e9645fb3fea39fae75e6e41fa1f30e7d4745f003))
* **ci:** speeding up incremental ci ([22146f8](https://github.com/hef/dbman/commit/22146f8cc2d5b838da04a5b033cf0d84f95a3e6a))
* **ci:** still trying to get "extra-files" to update chart.yaml ([fae52fd](https://github.com/hef/dbman/commit/fae52fd4fd644099b1065f8d9bacb1edbe0c4216))
* **ci:** switching default to main branch ([796b308](https://github.com/hef/dbman/commit/796b30835abddb245936079eef442643ddae7f62))
* **ci:** switching to manifest release for realz ([8cec224](https://github.com/hef/dbman/commit/8cec224575a0f30eb4ab13a17e0c62428f15869f))
* **ci:** tagged build support ([83050d4](https://github.com/hef/dbman/commit/83050d4d64428e7b3ef1704b1c8055392b91c755))
* **ci:** trying to get a build to finish in less than 6 hours ([16b2f3b](https://github.com/hef/dbman/commit/16b2f3bf54a37cf2bb7430cf2b01da79e2d9de49))
* **ci:** update chart version on release ([95db698](https://github.com/hef/dbman/commit/95db698d316a15a4adc11a98729cfe4d78fff264))
* **ci:** updating helm version ([3860e9e](https://github.com/hef/dbman/commit/3860e9e1cd72499f44140939d31d070adc6b93b0))
* **ci:** updating versions ([f1acf9a](https://github.com/hef/dbman/commit/f1acf9af3b6092f79fb2a1197523a001109ca87a))
* **ci:** use correct output for version ([2367c96](https://github.com/hef/dbman/commit/2367c961c8764417f538c29f3a3fb323f597b449))
* **ci:** use manifest release type ([d37ebfc](https://github.com/hef/dbman/commit/d37ebfcfd5a588720c4590debb2cfcd7b527c900))
* **ci:** using cross compiling in docker ([930a7ea](https://github.com/hef/dbman/commit/930a7ea0fdbe65ec3e9d55b0a877317164f02eb0))
* **clippy:** making errors impossible ([032dd88](https://github.com/hef/dbman/commit/032dd882f66c473e5cc0164c87b5d96b79ef00f4))
* **crd:** generating better crds ([f69c85f](https://github.com/hef/dbman/commit/f69c85ff8ea75ea211a0feb92509053f16c5857c))
* **dbc:** actually use the database password for a database server ([2490583](https://github.com/hef/dbman/commit/2490583cb414d318dbe703a328bd99522a2cadc1))
* **deps:** update rust crate actix-web to 4.4.1 ([db21b7d](https://github.com/hef/dbman/commit/db21b7d9ff35679b8435baad24f1c705a5c5dd00))
* **deps:** update rust crate anyhow to 1.0.76 ([85d798f](https://github.com/hef/dbman/commit/85d798f5fb2954fd4d63712a2c5d93792ea4f35d))
* **deps:** update rust crate anyhow to 1.0.77 ([fbd7301](https://github.com/hef/dbman/commit/fbd73013273ee30e7d996aa348210986917fd2e9))
* **deps:** update rust crate anyhow to 1.0.78 ([b4d5d66](https://github.com/hef/dbman/commit/b4d5d66d8d931253af1aa6575ac8f965e79a9c8d))
* **deps:** update rust crate anyhow to 1.0.79 ([82874c8](https://github.com/hef/dbman/commit/82874c8dd5882527e9c6769458a8b52e16443ace))
* **deps:** update rust crate clap to 4.4.1 ([166cc4e](https://github.com/hef/dbman/commit/166cc4e2d74493086b45a634b1368adc0f525ebb))
* **deps:** update rust crate clap to 4.4.11 ([db0321a](https://github.com/hef/dbman/commit/db0321a3dbd542de290cd322dad42cda766745ec))
* **deps:** update rust crate clap to 4.4.12 ([0f2f875](https://github.com/hef/dbman/commit/0f2f8753d08d956840b899d59555467ba8947e93))
* **deps:** update rust crate clap to 4.4.13 ([55108a7](https://github.com/hef/dbman/commit/55108a764e9049a87283d8dfb926276f9d610a2d))
* **deps:** update rust crate clap to 4.4.14 ([4bd10dc](https://github.com/hef/dbman/commit/4bd10dcc93cd7d6e0b051df02c82ad8adaec0235))
* **deps:** update rust crate clap to 4.4.15 ([40fbe03](https://github.com/hef/dbman/commit/40fbe0302470d9b1b8fb75ebd3e9ad4ea3efd3a1))
* **deps:** update rust crate clap to 4.4.16 ([09f0e54](https://github.com/hef/dbman/commit/09f0e541b60019ee831ac24c9b41d04a65922a1b))
* **deps:** update rust crate clap to 4.4.2 ([fcd76ae](https://github.com/hef/dbman/commit/fcd76ae728ee6366a7795419ddc662ce671566fd))
* **deps:** update rust crate clap to 4.4.3 ([853349c](https://github.com/hef/dbman/commit/853349c38f50d9d0b92a69a1074a589da8fc693c))
* **deps:** update rust crate clap to 4.4.5 ([db626a5](https://github.com/hef/dbman/commit/db626a5351ec9a57851b839b1f5a4330fd3dd6cd))
* **deps:** update rust crate clap to 4.4.6 ([9a46f59](https://github.com/hef/dbman/commit/9a46f5948474a59cc9eb559b068ccaf34658ab86))
* **deps:** update rust crate clap to 4.4.7 ([47cd0da](https://github.com/hef/dbman/commit/47cd0daef776303244bd3edabb9ebd0d5362bdbc))
* **deps:** update rust crate env_logger to 0.10.1 ([a289f70](https://github.com/hef/dbman/commit/a289f706f664f011c8a4a2ea9f88e5e56231be0a))
* **deps:** update rust crate futures to 0.3.30 ([1877580](https://github.com/hef/dbman/commit/1877580562713fd496e52be83ea40613a942b1b4))
* **deps:** update rust crate kube to 0.87.2 ([949c20f](https://github.com/hef/dbman/commit/949c20f1685f05b2988a80511d5958c9baac2164))
* **deps:** update rust crate schemars to 0.8.13 ([e10b25b](https://github.com/hef/dbman/commit/e10b25b2f68f27b5e0fd84f89b63c3c72ee23b41))
* **deps:** update rust crate schemars to 0.8.15 ([786212a](https://github.com/hef/dbman/commit/786212a53bedc6177f3f87535ef5f319f4bdb386))
* **deps:** update rust crate schemars to 0.8.16 ([221ce4b](https://github.com/hef/dbman/commit/221ce4be0ef626d6ec44f76e780ea83f65cc34e3))
* **deps:** update rust crate serde to 1.0.189 ([aa3ee50](https://github.com/hef/dbman/commit/aa3ee50eae938b34874b17405b5a04013bdceea3))
* **deps:** update rust crate serde to 1.0.190 ([351609d](https://github.com/hef/dbman/commit/351609de77db531bec8f78c0a4d1037bf3a7609a))
* **deps:** update rust crate serde to 1.0.193 ([f9ad09f](https://github.com/hef/dbman/commit/f9ad09ff8d3f3ce6c1c376fa6a704ee0c2547d48))
* **deps:** update rust crate serde to 1.0.194 ([31ee78f](https://github.com/hef/dbman/commit/31ee78f49fdf495165ba7ac0da674ac77292d2d5))
* **deps:** update rust crate serde to 1.0.195 ([2efbd0a](https://github.com/hef/dbman/commit/2efbd0a548e75539998e63efe9c78ccb3a35bf8a))
* **deps:** update rust crate serde_json to 1.0.106 ([2b1bef4](https://github.com/hef/dbman/commit/2b1bef4c3877caf50ba2bb38dd7a45b65eb9ede8))
* **deps:** update rust crate serde_json to 1.0.107 ([822f8e4](https://github.com/hef/dbman/commit/822f8e4704fa23e841b2fe249907841c40d44ba8))
* **deps:** update rust crate serde_json to 1.0.108 ([fedbe68](https://github.com/hef/dbman/commit/fedbe680c13c42e861e1b88c520f9a6a32323c42))
* **deps:** update rust crate serde_json to 1.0.109 ([b3276a7](https://github.com/hef/dbman/commit/b3276a7a87ec7d8eac599d779210de2306294691))
* **deps:** update rust crate serde_json to 1.0.110 ([b98de2b](https://github.com/hef/dbman/commit/b98de2b6b967a6f80e7b5ac58be5e5afd39e2751))
* **deps:** update rust crate serde_json to 1.0.111 ([f914cfa](https://github.com/hef/dbman/commit/f914cfacbf17e0350e2ba4f148c5dbd8d13c9bd4))
* **deps:** update rust crate serde_yaml to 0.9.29 ([354a60b](https://github.com/hef/dbman/commit/354a60bd5d0f95bc1d220878b9e7f2be440d86dc))
* **deps:** update rust crate serde_yaml to 0.9.30 ([5d7ad83](https://github.com/hef/dbman/commit/5d7ad8363b72decfff49ff0808607699fb5fe8b0))
* **deps:** update rust crate thiserror to 1.0.48 ([c54b28e](https://github.com/hef/dbman/commit/c54b28e07864224a842483fe9561deda9695fb17))
* **deps:** update rust crate thiserror to 1.0.49 ([9c72e5a](https://github.com/hef/dbman/commit/9c72e5ad6442754dd3bf1bc3b0f4fc2aa2aeea66))
* **deps:** update rust crate thiserror to 1.0.50 ([8f88f93](https://github.com/hef/dbman/commit/8f88f93054beced40d06a48b74734b5cee5d35ae))
* **deps:** update rust crate thiserror to 1.0.51 ([c9c0f82](https://github.com/hef/dbman/commit/c9c0f82dc849c4be74037ff0b84758e6fbb1f100))
* **deps:** update rust crate thiserror to 1.0.52 ([004a2f0](https://github.com/hef/dbman/commit/004a2f07dc5eb9d6dc11a0fb420fb9a4bac65d14))
* **deps:** update rust crate thiserror to 1.0.53 ([a724c8b](https://github.com/hef/dbman/commit/a724c8b5074a0d5470d85f5c8d447c147295e069))
* **deps:** update rust crate thiserror to 1.0.56 ([6d25f9c](https://github.com/hef/dbman/commit/6d25f9c5ab23c50faace887b3300174ecd221777))
* **deps:** update rust crate tokio to 1.33.0 ([db4b600](https://github.com/hef/dbman/commit/db4b600e9462b7a14e4a11b8af739a6e9975622d))
* **diag:** cleaning up code ([e765117](https://github.com/hef/dbman/commit/e7651175509527476bb24d9640c861109445ae28))
* **doc:** cleaning up readme ([c7b9b61](https://github.com/hef/dbman/commit/c7b9b6178ac26973cae7d5867ba9d0ef98ad0c48))
* **doc:** fixing type in readme ([90a0378](https://github.com/hef/dbman/commit/90a0378e54546edcd27a62aae132fbc53981a3f7))
* **docker:** fixing copy step to use whatever build dir we end up using ([8e0e258](https://github.com/hef/dbman/commit/8e0e258937153f6fcac7a931f49b9d1502bc67d4))
* **docker:** fixing environment typo ([a04497a](https://github.com/hef/dbman/commit/a04497a48d766b35d49cd33a8ae8d012684a0106))
* **docker:** switching dockerfile back to musl ([128b14c](https://github.com/hef/dbman/commit/128b14c9918dee432f55ad00e4bb2cf9126a8795))
* **doc:** updating readme with new help location ([9da88dc](https://github.com/hef/dbman/commit/9da88dc499dbad45e5b07a44d1de4f4d30b2c999))
* **helm:** add crds to helm chart ([a76346e](https://github.com/hef/dbman/commit/a76346e810024c449240e6d9936ae50b2e4592ca))
* **helm:** add events create permission ([7c45390](https://github.com/hef/dbman/commit/7c4539014eea21f7408c8f37405cf8add2584a83))
* **helm:** add patch permissions for databases/databaseServers ([e0a3e24](https://github.com/hef/dbman/commit/e0a3e249efab6d04d219329fc868602dbcafe006))
* **helm:** add role and role binding to service account ([66a6413](https://github.com/hef/dbman/commit/66a6413cbc7e06e2d3e0e9a6c078a2dcb2912a96))
* **helm:** adding datases/status to permissions ([3d764d8](https://github.com/hef/dbman/commit/3d764d852efc369e792b8a552c5120a8106bd3c4))
* **helm:** adding ready probe ([428f1b7](https://github.com/hef/dbman/commit/428f1b78e745c4517c372fd94a17b2b67e6c23fe))
* **helm:** fix cluster role binding name ([3b656ea](https://github.com/hef/dbman/commit/3b656ea6a6def9a9e29a4ebfb8a59b675fa0935f))
* **helm:** fix indent error ([64e1cf5](https://github.com/hef/dbman/commit/64e1cf553a0cf40ff63910d560cebf20f3db3082))
* **helm:** fix permission to retrieve secrets ([c41e54e](https://github.com/hef/dbman/commit/c41e54e6e18b487eedc78df305685912fce34ea6))
* **helm:** fix type "Kind" -&gt; "kind" ([3284425](https://github.com/hef/dbman/commit/328442510ef28d5c7790509f08215ec385266f60))
* **helm:** fixing events permission ([26d55fc](https://github.com/hef/dbman/commit/26d55fcd56d1776ed4cf1904e26df20eb91c1b6b))
* **helm:** fixing helm chart issue ([f850df9](https://github.com/hef/dbman/commit/f850df95c449f57a7d628e4182e5f09778eb3479))
* **helm:** making default helm chart render correctly ([8146574](https://github.com/hef/dbman/commit/814657483012a67f6e26c03b725b52c36abef830))
* **helm:** pluralizing resources in cluster role ([f44c194](https://github.com/hef/dbman/commit/f44c194a028025971720822188cb052104c08b83))
* **heritage:** check for heritage on role before dropping role ([63d72fc](https://github.com/hef/dbman/commit/63d72fcbc9d5d684ee84cfbf514105e6d45615c3))
* **heritage:** setting heritage works now ([040fb39](https://github.com/hef/dbman/commit/040fb39615a72333c74ddd245b54c2d2be479793))
* **log:** adding events ([1a89a98](https://github.com/hef/dbman/commit/1a89a98e3963df186b99b0318c2b0c0cdb6abf43))
* **log:** exclude /readyz from path ([f7d5bf0](https://github.com/hef/dbman/commit/f7d5bf0614efa1313ccc01351c7c8d1be18341e7))
* **main:** don't require owner ([372a46a](https://github.com/hef/dbman/commit/372a46a048e0d4b41cb301918f64707b8316d9c0))
* **test:** fix integration test ([4207eae](https://github.com/hef/dbman/commit/4207eaeadf14a6774f8a01a26d2b6e93974546a8))
* **test:** setup_cpng needs to be called before install crds ([f4f14a1](https://github.com/hef/dbman/commit/f4f14a1429e45bb5f897448efe7f51838d827ef3))
* **version:** setting up for release-please tags ([45e2b05](https://github.com/hef/dbman/commit/45e2b0529f1f1d1beefbd816fafed69f06236a21))

## [0.118.0](https://github.com/hef/dbman/compare/dbman-v0.117.0...dbman-v0.118.0) (2024-01-14)


### Features

* **ci:** adding helm push ([25c2b2e](https://github.com/hef/dbman/commit/25c2b2e38a5fc7ce8b776a7a856e0c3a9caa8155))
* **ci:** adding major and major.miner version labels ([b1e6ae8](https://github.com/hef/dbman/commit/b1e6ae89af10dfa6a0790dc8cfc63b52f59ade3e))
* **ci:** adding version tagging ([4adad92](https://github.com/hef/dbman/commit/4adad9233cc7409db69812e95f2450711b66899d))
* **ci:** significant refactoring of integration test ([4aab6cc](https://github.com/hef/dbman/commit/4aab6cca789347d42434abe4092c11b9076e7b61))
* **crd:** switching to database server defined in crd ([4928111](https://github.com/hef/dbman/commit/4928111ae0eee099047fd846b33357127eb0d0cb))
* create release on release ([8720a0a](https://github.com/hef/dbman/commit/8720a0a123e28f565fe427e03fa1fb795c9a7cb0))
* **error:** clearer error message for DatabaseServer missing database ([e5baefb](https://github.com/hef/dbman/commit/e5baefb35d08487962001ef17528e1c7c460b763))
* **helm:** add env setting ([95add47](https://github.com/hef/dbman/commit/95add4778879683ce04e15f0a951c16a885a0fc2))
* **helm:** moving helm chart ([cc65dfa](https://github.com/hef/dbman/commit/cc65dfadee88cbf0109f7fb4a33c657f8745d17f))
* **heritage:** applying heritage to roles too ([fd5185d](https://github.com/hef/dbman/commit/fd5185de39ba3ac4e64e3a0af09316db82c18a09))
* **heritage:** some sanity checks for ownership are being added ([607a6a3](https://github.com/hef/dbman/commit/607a6a314249d6444f3cfa49917ea0450c7e0187))
* **rust:** upgrade rust again ([599cb56](https://github.com/hef/dbman/commit/599cb5610fad5df88aa42a8325cd56f819073c01))
* **test:** adding an integration test ([885ee70](https://github.com/hef/dbman/commit/885ee7012ef8b90a71fc0fa96a598e46a953a47e))


### Bug Fixes

* **build:** fixing build error ([316d114](https://github.com/hef/dbman/commit/316d1143f8bd3fc53385b06130392443e1ddd1e7))
* **build:** random code that needs mods ([7a332b3](https://github.com/hef/dbman/commit/7a332b31c13cf2837d6d46523684985313bf7f7d))
* **cargo:** adding changes from rustls-tls change ([3735adc](https://github.com/hef/dbman/commit/3735adc51dcbc0f83591cf6d47b453a5364c3a4a))
* **ci:** actually setting build-mode as output ([f6350bc](https://github.com/hef/dbman/commit/f6350bc10a2af85ed023264e6ee78728c36e2e42))
* **ci:** Add concurrency settings to workflows ([55c05c7](https://github.com/hef/dbman/commit/55c05c7de325a2c01b60e21273c03d1663d6cc6e))
* **ci:** add kind to rust tests action ([03d6fb3](https://github.com/hef/dbman/commit/03d6fb36345283dbe2d35376953ce1849f32058d))
* **ci:** addinb `v` to helm-push version ([5f67352](https://github.com/hef/dbman/commit/5f67352a7ce9dcffd559c0ffc8784b8957937040))
* **ci:** adding chef to dockerfile ([c531fa4](https://github.com/hef/dbman/commit/c531fa43eb7affc0fc2371a091cb225ce7e0297d))
* **ci:** adjusting docker metadata ([c7cceec](https://github.com/hef/dbman/commit/c7cceec1efc1b8b130b4edbaa327c5fff4c5ab82))
* **ci:** adjusting metadata output to explicitly include tags ([0f9e55c](https://github.com/hef/dbman/commit/0f9e55c5d7cf37fef6759e46d7c54bceb149b80d))
* **ci:** allow release to write github releases ([c4190cc](https://github.com/hef/dbman/commit/c4190ccaab57e7b1ebd3bc29ba75ae732091631b))
* **ci:** bumping helm versions as part of upgrade ([a3b8f67](https://github.com/hef/dbman/commit/a3b8f6778969f96f069d50947fa5bb25eaa2ff2f))
* **ci:** changing bsord/helm-push to a real version ([8f4af00](https://github.com/hef/dbman/commit/8f4af0031ca850d09f1dcfa253926833465dccec))
* **ci:** changing build profile for tagged versions ([0e6e1bb](https://github.com/hef/dbman/commit/0e6e1bbf9f8fd9441018f04fb70d1d7096f4900e))
* **ci:** Cut off "new release spam" to stargazers ([0ff9f96](https://github.com/hef/dbman/commit/0ff9f960c491c65064db074d4569ca0e7720c6f8))
* **ci:** disabling extra optimizations in for-distribution for now ([4df1075](https://github.com/hef/dbman/commit/4df10752ae5bd1ea37e91cfcd84150b90937b345))
* **ci:** doing automerges into develop branch ([e843ca1](https://github.com/hef/dbman/commit/e843ca126db03667c3e910dffc18ed04c94dbec9))
* **ci:** exclude release action from running on renovate branches ([e89b476](https://github.com/hef/dbman/commit/e89b4766268ac783370649c462db2137f03ae8ab))
* **ci:** fix helm template verison to not include a `v` ([e205bab](https://github.com/hef/dbman/commit/e205babe36dd6e2ceafae35133132b7efa636c21))
* **ci:** fixing chart-folder locaiton ([e94cc08](https://github.com/hef/dbman/commit/e94cc08b07f3b0178964d8febc5d96f4492ce986))
* **ci:** fixing for-distributtion for tagged builds ([43d3c04](https://github.com/hef/dbman/commit/43d3c049a13272693991a3a8af2f953c054fea4a))
* **ci:** fixing github-token arg to go-sematnic-release ([d826e73](https://github.com/hef/dbman/commit/d826e73b4a8b3c2164d3066bed7323b7afad5ffc))
* **ci:** fixing type enabled -&gt; enable ([1534be3](https://github.com/hef/dbman/commit/1534be3e0d2c77c9e812d67fd0999bb45fe78dca))
* **ci:** fixing version parsing ([5410f52](https://github.com/hef/dbman/commit/5410f526d238d8816c4758f0dfe78db1da875876))
* **ci:** get tag after creating tag ([47ec639](https://github.com/hef/dbman/commit/47ec6399e9811471d4dd862f5b972f1f1a1f7b79))
* **ci:** make build finish ([fe8ee60](https://github.com/hef/dbman/commit/fe8ee60e724e09e365f0924c739ba4a1973c8f74))
* **ci:** make incremental builds a little faster ([0ccfefa](https://github.com/hef/dbman/commit/0ccfefa36acf4873e9ada266bcc8ea397bd97019))
* **ci:** make kind-action use default kind cluster_name ([d7ef779](https://github.com/hef/dbman/commit/d7ef779091dfd10390d03cf79e51b24871f3dc19))
* **ci:** making build complete in a reasonable amount of time ([a4baf13](https://github.com/hef/dbman/commit/a4baf13b4b1775d45ccf16a80e98d86126050fe4))
* **ci:** making docker build faster ([bd43be2](https://github.com/hef/dbman/commit/bd43be206f520d4f8ac09564ff2fca287f8330b8))
* **ci:** only create real release on push to main ([f710332](https://github.com/hef/dbman/commit/f7103327266c6975d7a48916e915d06f96057b09))
* **ci:** remove for-distribution profile ([3ac1d4f](https://github.com/hef/dbman/commit/3ac1d4f51feb1d7ba27d25d99332817d64fe0f30))
* **ci:** removing provider from semrel ([c919b3d](https://github.com/hef/dbman/commit/c919b3d1c210569c17ead783b2e1ec04491db41d))
* **ci:** renovate should use default branch, which is now develop ([3924103](https://github.com/hef/dbman/commit/39241039af81a926a58ec3058e0c15d1f113a986))
* **ci:** running rust test on all branches, and all PRs ([1eed4a0](https://github.com/hef/dbman/commit/1eed4a019c5c7d052fcdd941299b631c6c4da1b0))
* **ci:** setting all checked in versions to 0.0.0 ([3b4e431](https://github.com/hef/dbman/commit/3b4e431faf20a823c30e3599dade86ce420e710a))
* **ci:** setting lto to thin to ci can finish ([67befea](https://github.com/hef/dbman/commit/67befeaad33d14c6598b6774138946fadca2effc))
* **ci:** speeding up docker build ([e9645fb](https://github.com/hef/dbman/commit/e9645fb3fea39fae75e6e41fa1f30e7d4745f003))
* **ci:** speeding up incremental ci ([22146f8](https://github.com/hef/dbman/commit/22146f8cc2d5b838da04a5b033cf0d84f95a3e6a))
* **ci:** still trying to get "extra-files" to update chart.yaml ([fae52fd](https://github.com/hef/dbman/commit/fae52fd4fd644099b1065f8d9bacb1edbe0c4216))
* **ci:** switching default to main branch ([796b308](https://github.com/hef/dbman/commit/796b30835abddb245936079eef442643ddae7f62))
* **ci:** switching to manifest release for realz ([8cec224](https://github.com/hef/dbman/commit/8cec224575a0f30eb4ab13a17e0c62428f15869f))
* **ci:** tagged build support ([83050d4](https://github.com/hef/dbman/commit/83050d4d64428e7b3ef1704b1c8055392b91c755))
* **ci:** trying to get a build to finish in less than 6 hours ([16b2f3b](https://github.com/hef/dbman/commit/16b2f3bf54a37cf2bb7430cf2b01da79e2d9de49))
* **ci:** update chart version on release ([95db698](https://github.com/hef/dbman/commit/95db698d316a15a4adc11a98729cfe4d78fff264))
* **ci:** updating helm version ([3860e9e](https://github.com/hef/dbman/commit/3860e9e1cd72499f44140939d31d070adc6b93b0))
* **ci:** updating versions ([f1acf9a](https://github.com/hef/dbman/commit/f1acf9af3b6092f79fb2a1197523a001109ca87a))
* **ci:** use correct output for version ([2367c96](https://github.com/hef/dbman/commit/2367c961c8764417f538c29f3a3fb323f597b449))
* **ci:** use manifest release type ([d37ebfc](https://github.com/hef/dbman/commit/d37ebfcfd5a588720c4590debb2cfcd7b527c900))
* **ci:** using cross compiling in docker ([930a7ea](https://github.com/hef/dbman/commit/930a7ea0fdbe65ec3e9d55b0a877317164f02eb0))
* **clippy:** making errors impossible ([032dd88](https://github.com/hef/dbman/commit/032dd882f66c473e5cc0164c87b5d96b79ef00f4))
* **crd:** generating better crds ([f69c85f](https://github.com/hef/dbman/commit/f69c85ff8ea75ea211a0feb92509053f16c5857c))
* **dbc:** actually use the database password for a database server ([2490583](https://github.com/hef/dbman/commit/2490583cb414d318dbe703a328bd99522a2cadc1))
* **deps:** update rust crate actix-web to 4.4.1 ([db21b7d](https://github.com/hef/dbman/commit/db21b7d9ff35679b8435baad24f1c705a5c5dd00))
* **deps:** update rust crate anyhow to 1.0.76 ([85d798f](https://github.com/hef/dbman/commit/85d798f5fb2954fd4d63712a2c5d93792ea4f35d))
* **deps:** update rust crate anyhow to 1.0.77 ([fbd7301](https://github.com/hef/dbman/commit/fbd73013273ee30e7d996aa348210986917fd2e9))
* **deps:** update rust crate anyhow to 1.0.78 ([b4d5d66](https://github.com/hef/dbman/commit/b4d5d66d8d931253af1aa6575ac8f965e79a9c8d))
* **deps:** update rust crate anyhow to 1.0.79 ([82874c8](https://github.com/hef/dbman/commit/82874c8dd5882527e9c6769458a8b52e16443ace))
* **deps:** update rust crate clap to 4.4.1 ([166cc4e](https://github.com/hef/dbman/commit/166cc4e2d74493086b45a634b1368adc0f525ebb))
* **deps:** update rust crate clap to 4.4.11 ([db0321a](https://github.com/hef/dbman/commit/db0321a3dbd542de290cd322dad42cda766745ec))
* **deps:** update rust crate clap to 4.4.12 ([0f2f875](https://github.com/hef/dbman/commit/0f2f8753d08d956840b899d59555467ba8947e93))
* **deps:** update rust crate clap to 4.4.13 ([55108a7](https://github.com/hef/dbman/commit/55108a764e9049a87283d8dfb926276f9d610a2d))
* **deps:** update rust crate clap to 4.4.14 ([4bd10dc](https://github.com/hef/dbman/commit/4bd10dcc93cd7d6e0b051df02c82ad8adaec0235))
* **deps:** update rust crate clap to 4.4.15 ([40fbe03](https://github.com/hef/dbman/commit/40fbe0302470d9b1b8fb75ebd3e9ad4ea3efd3a1))
* **deps:** update rust crate clap to 4.4.16 ([09f0e54](https://github.com/hef/dbman/commit/09f0e541b60019ee831ac24c9b41d04a65922a1b))
* **deps:** update rust crate clap to 4.4.2 ([fcd76ae](https://github.com/hef/dbman/commit/fcd76ae728ee6366a7795419ddc662ce671566fd))
* **deps:** update rust crate clap to 4.4.3 ([853349c](https://github.com/hef/dbman/commit/853349c38f50d9d0b92a69a1074a589da8fc693c))
* **deps:** update rust crate clap to 4.4.5 ([db626a5](https://github.com/hef/dbman/commit/db626a5351ec9a57851b839b1f5a4330fd3dd6cd))
* **deps:** update rust crate clap to 4.4.6 ([9a46f59](https://github.com/hef/dbman/commit/9a46f5948474a59cc9eb559b068ccaf34658ab86))
* **deps:** update rust crate clap to 4.4.7 ([47cd0da](https://github.com/hef/dbman/commit/47cd0daef776303244bd3edabb9ebd0d5362bdbc))
* **deps:** update rust crate env_logger to 0.10.1 ([a289f70](https://github.com/hef/dbman/commit/a289f706f664f011c8a4a2ea9f88e5e56231be0a))
* **deps:** update rust crate futures to 0.3.30 ([1877580](https://github.com/hef/dbman/commit/1877580562713fd496e52be83ea40613a942b1b4))
* **deps:** update rust crate kube to 0.87.2 ([949c20f](https://github.com/hef/dbman/commit/949c20f1685f05b2988a80511d5958c9baac2164))
* **deps:** update rust crate schemars to 0.8.13 ([e10b25b](https://github.com/hef/dbman/commit/e10b25b2f68f27b5e0fd84f89b63c3c72ee23b41))
* **deps:** update rust crate schemars to 0.8.15 ([786212a](https://github.com/hef/dbman/commit/786212a53bedc6177f3f87535ef5f319f4bdb386))
* **deps:** update rust crate schemars to 0.8.16 ([221ce4b](https://github.com/hef/dbman/commit/221ce4be0ef626d6ec44f76e780ea83f65cc34e3))
* **deps:** update rust crate serde to 1.0.189 ([aa3ee50](https://github.com/hef/dbman/commit/aa3ee50eae938b34874b17405b5a04013bdceea3))
* **deps:** update rust crate serde to 1.0.190 ([351609d](https://github.com/hef/dbman/commit/351609de77db531bec8f78c0a4d1037bf3a7609a))
* **deps:** update rust crate serde to 1.0.193 ([f9ad09f](https://github.com/hef/dbman/commit/f9ad09ff8d3f3ce6c1c376fa6a704ee0c2547d48))
* **deps:** update rust crate serde to 1.0.194 ([31ee78f](https://github.com/hef/dbman/commit/31ee78f49fdf495165ba7ac0da674ac77292d2d5))
* **deps:** update rust crate serde to 1.0.195 ([2efbd0a](https://github.com/hef/dbman/commit/2efbd0a548e75539998e63efe9c78ccb3a35bf8a))
* **deps:** update rust crate serde_json to 1.0.106 ([2b1bef4](https://github.com/hef/dbman/commit/2b1bef4c3877caf50ba2bb38dd7a45b65eb9ede8))
* **deps:** update rust crate serde_json to 1.0.107 ([822f8e4](https://github.com/hef/dbman/commit/822f8e4704fa23e841b2fe249907841c40d44ba8))
* **deps:** update rust crate serde_json to 1.0.108 ([fedbe68](https://github.com/hef/dbman/commit/fedbe680c13c42e861e1b88c520f9a6a32323c42))
* **deps:** update rust crate serde_json to 1.0.109 ([b3276a7](https://github.com/hef/dbman/commit/b3276a7a87ec7d8eac599d779210de2306294691))
* **deps:** update rust crate serde_json to 1.0.110 ([b98de2b](https://github.com/hef/dbman/commit/b98de2b6b967a6f80e7b5ac58be5e5afd39e2751))
* **deps:** update rust crate serde_json to 1.0.111 ([f914cfa](https://github.com/hef/dbman/commit/f914cfacbf17e0350e2ba4f148c5dbd8d13c9bd4))
* **deps:** update rust crate serde_yaml to 0.9.29 ([354a60b](https://github.com/hef/dbman/commit/354a60bd5d0f95bc1d220878b9e7f2be440d86dc))
* **deps:** update rust crate serde_yaml to 0.9.30 ([5d7ad83](https://github.com/hef/dbman/commit/5d7ad8363b72decfff49ff0808607699fb5fe8b0))
* **deps:** update rust crate thiserror to 1.0.48 ([c54b28e](https://github.com/hef/dbman/commit/c54b28e07864224a842483fe9561deda9695fb17))
* **deps:** update rust crate thiserror to 1.0.49 ([9c72e5a](https://github.com/hef/dbman/commit/9c72e5ad6442754dd3bf1bc3b0f4fc2aa2aeea66))
* **deps:** update rust crate thiserror to 1.0.50 ([8f88f93](https://github.com/hef/dbman/commit/8f88f93054beced40d06a48b74734b5cee5d35ae))
* **deps:** update rust crate thiserror to 1.0.51 ([c9c0f82](https://github.com/hef/dbman/commit/c9c0f82dc849c4be74037ff0b84758e6fbb1f100))
* **deps:** update rust crate thiserror to 1.0.52 ([004a2f0](https://github.com/hef/dbman/commit/004a2f07dc5eb9d6dc11a0fb420fb9a4bac65d14))
* **deps:** update rust crate thiserror to 1.0.53 ([a724c8b](https://github.com/hef/dbman/commit/a724c8b5074a0d5470d85f5c8d447c147295e069))
* **deps:** update rust crate thiserror to 1.0.56 ([6d25f9c](https://github.com/hef/dbman/commit/6d25f9c5ab23c50faace887b3300174ecd221777))
* **deps:** update rust crate tokio to 1.33.0 ([db4b600](https://github.com/hef/dbman/commit/db4b600e9462b7a14e4a11b8af739a6e9975622d))
* **diag:** cleaning up code ([e765117](https://github.com/hef/dbman/commit/e7651175509527476bb24d9640c861109445ae28))
* **doc:** cleaning up readme ([c7b9b61](https://github.com/hef/dbman/commit/c7b9b6178ac26973cae7d5867ba9d0ef98ad0c48))
* **doc:** fixing type in readme ([90a0378](https://github.com/hef/dbman/commit/90a0378e54546edcd27a62aae132fbc53981a3f7))
* **docker:** fixing copy step to use whatever build dir we end up using ([8e0e258](https://github.com/hef/dbman/commit/8e0e258937153f6fcac7a931f49b9d1502bc67d4))
* **docker:** fixing environment typo ([a04497a](https://github.com/hef/dbman/commit/a04497a48d766b35d49cd33a8ae8d012684a0106))
* **docker:** switching dockerfile back to musl ([128b14c](https://github.com/hef/dbman/commit/128b14c9918dee432f55ad00e4bb2cf9126a8795))
* **doc:** updating readme with new help location ([9da88dc](https://github.com/hef/dbman/commit/9da88dc499dbad45e5b07a44d1de4f4d30b2c999))
* **helm:** add crds to helm chart ([a76346e](https://github.com/hef/dbman/commit/a76346e810024c449240e6d9936ae50b2e4592ca))
* **helm:** add events create permission ([7c45390](https://github.com/hef/dbman/commit/7c4539014eea21f7408c8f37405cf8add2584a83))
* **helm:** add patch permissions for databases/databaseServers ([e0a3e24](https://github.com/hef/dbman/commit/e0a3e249efab6d04d219329fc868602dbcafe006))
* **helm:** add role and role binding to service account ([66a6413](https://github.com/hef/dbman/commit/66a6413cbc7e06e2d3e0e9a6c078a2dcb2912a96))
* **helm:** adding datases/status to permissions ([3d764d8](https://github.com/hef/dbman/commit/3d764d852efc369e792b8a552c5120a8106bd3c4))
* **helm:** adding ready probe ([428f1b7](https://github.com/hef/dbman/commit/428f1b78e745c4517c372fd94a17b2b67e6c23fe))
* **helm:** fix cluster role binding name ([3b656ea](https://github.com/hef/dbman/commit/3b656ea6a6def9a9e29a4ebfb8a59b675fa0935f))
* **helm:** fix indent error ([64e1cf5](https://github.com/hef/dbman/commit/64e1cf553a0cf40ff63910d560cebf20f3db3082))
* **helm:** fix permission to retrieve secrets ([c41e54e](https://github.com/hef/dbman/commit/c41e54e6e18b487eedc78df305685912fce34ea6))
* **helm:** fix type "Kind" -&gt; "kind" ([3284425](https://github.com/hef/dbman/commit/328442510ef28d5c7790509f08215ec385266f60))
* **helm:** fixing events permission ([26d55fc](https://github.com/hef/dbman/commit/26d55fcd56d1776ed4cf1904e26df20eb91c1b6b))
* **helm:** fixing helm chart issue ([f850df9](https://github.com/hef/dbman/commit/f850df95c449f57a7d628e4182e5f09778eb3479))
* **helm:** making default helm chart render correctly ([8146574](https://github.com/hef/dbman/commit/814657483012a67f6e26c03b725b52c36abef830))
* **helm:** pluralizing resources in cluster role ([f44c194](https://github.com/hef/dbman/commit/f44c194a028025971720822188cb052104c08b83))
* **heritage:** check for heritage on role before dropping role ([63d72fc](https://github.com/hef/dbman/commit/63d72fcbc9d5d684ee84cfbf514105e6d45615c3))
* **heritage:** setting heritage works now ([040fb39](https://github.com/hef/dbman/commit/040fb39615a72333c74ddd245b54c2d2be479793))
* **log:** adding events ([1a89a98](https://github.com/hef/dbman/commit/1a89a98e3963df186b99b0318c2b0c0cdb6abf43))
* **log:** exclude /readyz from path ([f7d5bf0](https://github.com/hef/dbman/commit/f7d5bf0614efa1313ccc01351c7c8d1be18341e7))
* **main:** don't require owner ([372a46a](https://github.com/hef/dbman/commit/372a46a048e0d4b41cb301918f64707b8316d9c0))
* **test:** fix integration test ([4207eae](https://github.com/hef/dbman/commit/4207eaeadf14a6774f8a01a26d2b6e93974546a8))
* **test:** setup_cpng needs to be called before install crds ([f4f14a1](https://github.com/hef/dbman/commit/f4f14a1429e45bb5f897448efe7f51838d827ef3))
* **version:** setting up for release-please tags ([45e2b05](https://github.com/hef/dbman/commit/45e2b0529f1f1d1beefbd816fafed69f06236a21))
