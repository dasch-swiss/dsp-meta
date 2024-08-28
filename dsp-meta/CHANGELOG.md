# Changelog

## [2.1.0](https://github.com/dasch-swiss/dsp-meta/compare/dsp-meta-v2.0.3...dsp-meta-v2.1.0) (2024-08-28)


### ⚠ BREAKING CHANGES

* Backport schema changes (DEV-3953) ([#169](https://github.com/dasch-swiss/dsp-meta/issues/169))

### Enhancements

* Add draft json schema validation and serde model ([#144](https://github.com/dasch-swiss/dsp-meta/issues/144)) ([df7948b](https://github.com/dasch-swiss/dsp-meta/commit/df7948b1fc5a411f064dbaa9657875405a551706))
* Add grant domain model and deserialization ([#121](https://github.com/dasch-swiss/dsp-meta/issues/121)) ([f4a6162](https://github.com/dasch-swiss/dsp-meta/commit/f4a61623b3b640f8a8b1bd85969ea01ab9b3b056))
* Add project serialization to rdf (ongoing) ([#62](https://github.com/dasch-swiss/dsp-meta/issues/62)) ([2e7d33c](https://github.com/dasch-swiss/dsp-meta/commit/2e7d33cebf7633f63dcfbd904913faf547363d68))
* Add robots.txt and sitemap.xml (DEV-3931) ([#152](https://github.com/dasch-swiss/dsp-meta/issues/152)) ([6dceb9e](https://github.com/dasch-swiss/dsp-meta/commit/6dceb9e32776f1678bfb10c144b87d1bba25c0ea))
* Add serialization to JSON (ongoing) ([#69](https://github.com/dasch-swiss/dsp-meta/issues/69)) ([fc9481f](https://github.com/dasch-swiss/dsp-meta/commit/fc9481f6abc0ab80bcf3c9e3c634789785fa089d))
* Backport schema changes (DEV-3953) ([#169](https://github.com/dasch-swiss/dsp-meta/issues/169)) ([04ed3c4](https://github.com/dasch-swiss/dsp-meta/commit/04ed3c4a15f24eb1dbc8909d3a212c4570ca0a40))
* **dsp-domain:** Add ontology domain model ([#61](https://github.com/dasch-swiss/dsp-meta/issues/61)) ([c80cd83](https://github.com/dasch-swiss/dsp-meta/commit/c80cd83a5b9b749a00c84a5da5a1b4af606dbdc9))
* **dsp-meta:** Load data at startup from disk and server through api ([#70](https://github.com/dasch-swiss/dsp-meta/issues/70)) ([f82fb5a](https://github.com/dasch-swiss/dsp-meta/commit/f82fb5a1e11db731cea14c9b62458a23abb1cdbf))
* Extend dataset domain model and a bit of deserialization (ongoing) ([#113](https://github.com/dasch-swiss/dsp-meta/issues/113)) ([070a729](https://github.com/dasch-swiss/dsp-meta/commit/070a7290a65a266e8d9c7284b7af3733c64cd490))
* Extend dataset domain model and deserialization (ongoing) ([#115](https://github.com/dasch-swiss/dsp-meta/issues/115)) ([4264c72](https://github.com/dasch-swiss/dsp-meta/commit/4264c721cd80c889175331263b591a94c999b465))
* Make service work with json (DEV-3930) ([#146](https://github.com/dasch-swiss/dsp-meta/issues/146)) ([184c710](https://github.com/dasch-swiss/dsp-meta/commit/184c71031595885389cd045375213b81ac09b00b))


### Maintenance

* Add nix ([#123](https://github.com/dasch-swiss/dsp-meta/issues/123)) ([6044adc](https://github.com/dasch-swiss/dsp-meta/commit/6044adce0a692ffa1cfb705e252f408546b45897))
* Add version for each crate ([#162](https://github.com/dasch-swiss/dsp-meta/issues/162)) ([825a282](https://github.com/dasch-swiss/dsp-meta/commit/825a282bd0828eba40cec6e2038176bf0c6ab768))
* Attempt to fix release-please ([#186](https://github.com/dasch-swiss/dsp-meta/issues/186)) ([ab7ca21](https://github.com/dasch-swiss/dsp-meta/commit/ab7ca21a31993c2f4e51447ce25232084771fcf8))
* Bump axum to 0.7 ([#97](https://github.com/dasch-swiss/dsp-meta/issues/97)) ([a12d77b](https://github.com/dasch-swiss/dsp-meta/commit/a12d77b5f156afa9ba4adbbac58e040a4e7e5711))
* Change towards single version ([#80](https://github.com/dasch-swiss/dsp-meta/issues/80)) ([564736a](https://github.com/dasch-swiss/dsp-meta/commit/564736a398750aed47bdad704aa715a4a4252f62))
* Change towards single version ([#82](https://github.com/dasch-swiss/dsp-meta/issues/82)) ([2f41c23](https://github.com/dasch-swiss/dsp-meta/commit/2f41c236d40792165255f3d21addf60d6c87424f))
* Cleanup logging ([#94](https://github.com/dasch-swiss/dsp-meta/issues/94)) ([ff5f9d4](https://github.com/dasch-swiss/dsp-meta/commit/ff5f9d438e96aed9deb2e8d891e286217aa5d608))
* **deps:** Bump assert_cmd from 2.0.15 to 2.0.16 ([#184](https://github.com/dasch-swiss/dsp-meta/issues/184)) ([b8dbfa5](https://github.com/dasch-swiss/dsp-meta/commit/b8dbfa5ca727e272fd8f6ac9bfe9dd5543aa62bf))
* **deps:** Bump sophia from 0.7.2 to 0.8.0 ([#118](https://github.com/dasch-swiss/dsp-meta/issues/118)) ([63a3d1b](https://github.com/dasch-swiss/dsp-meta/commit/63a3d1bb2dd4adb39b62f7f28b26c8216ade37b6))
* **deps:** Bump thiserror from 1.0.50 to 1.0.56 ([#109](https://github.com/dasch-swiss/dsp-meta/issues/109)) ([8c6ad74](https://github.com/dasch-swiss/dsp-meta/commit/8c6ad740fe228423ecc3fa07f265cbdf55dd1da8))
* Force release 2.0.3 ([c622a06](https://github.com/dasch-swiss/dsp-meta/commit/c622a06c8f4e04d99d8f0b23cf46e7d2c0209805))
* Force release 2.1.0 ([fc6a297](https://github.com/dasch-swiss/dsp-meta/commit/fc6a297805dda80e656ec923a0ccc37348813830))
* In schema make job title and affiliation optional for person (DEV-3933) ([#157](https://github.com/dasch-swiss/dsp-meta/issues/157)) ([eb9526b](https://github.com/dasch-swiss/dsp-meta/commit/eb9526ba8f7c4c118b8f3d70f44e8709f637160e))
* Release 2.0.0 ([#154](https://github.com/dasch-swiss/dsp-meta/issues/154)) ([3d5d1ca](https://github.com/dasch-swiss/dsp-meta/commit/3d5d1ca63d85737ce21b3cd4c6eee0525f9146b0))
* Release 2.0.0 ([#155](https://github.com/dasch-swiss/dsp-meta/issues/155)) ([fef0913](https://github.com/dasch-swiss/dsp-meta/commit/fef0913bfc2fd49c8057d31a4bad4a44ec1c5cc3))
* Release main ([#163](https://github.com/dasch-swiss/dsp-meta/issues/163)) ([2cd272d](https://github.com/dasch-swiss/dsp-meta/commit/2cd272d02ab0c378dc661869f01cccca0a36ffdf))
* Release main ([#164](https://github.com/dasch-swiss/dsp-meta/issues/164)) ([41b71e8](https://github.com/dasch-swiss/dsp-meta/commit/41b71e8d5e5a0a8703de0b154c3e8ae0f698b9d4))
* Release main ([#75](https://github.com/dasch-swiss/dsp-meta/issues/75)) ([03cfae7](https://github.com/dasch-swiss/dsp-meta/commit/03cfae719eb5363677eacc7c9d488c014ea5ff3d))
* Release main ([#83](https://github.com/dasch-swiss/dsp-meta/issues/83)) ([dae1280](https://github.com/dasch-swiss/dsp-meta/commit/dae12802411e6d4ec87d1c5016f788f0300c4e26))
* Update data and activate test validating finished projects ([#161](https://github.com/dasch-swiss/dsp-meta/issues/161)) ([b45825d](https://github.com/dasch-swiss/dsp-meta/commit/b45825dc0fd35c762932a54a1ea0c110e8422374))

## [0.1.0](https://github.com/dasch-swiss/dsp-meta/compare/dsp-meta-v0.1.4...dsp-meta-v0.1.0) (2024-07-31)


### Miscellaneous Chores

* **dsp-meta:** Synchronize dsp-meta versions

## [0.1.4](https://github.com/dasch-swiss/dsp-meta/compare/dsp-meta-v0.1.3...dsp-meta-v0.1.4) (2024-07-30)


### Features

* Add and serve old frontend ([#92](https://github.com/dasch-swiss/dsp-meta/issues/92)) ([00fd892](https://github.com/dasch-swiss/dsp-meta/commit/00fd892d4d1553e29c274d83c6bdc06b0e249253))
* Add and serve old frontend ([#93](https://github.com/dasch-swiss/dsp-meta/issues/93)) ([3058175](https://github.com/dasch-swiss/dsp-meta/commit/30581753c734a26acf262619709280d35fe97d5b))
* Add draft json schema validation and serde model ([#144](https://github.com/dasch-swiss/dsp-meta/issues/144)) ([df7948b](https://github.com/dasch-swiss/dsp-meta/commit/df7948b1fc5a411f064dbaa9657875405a551706))
* Add grant domain model and deserialization ([#121](https://github.com/dasch-swiss/dsp-meta/issues/121)) ([f4a6162](https://github.com/dasch-swiss/dsp-meta/commit/f4a61623b3b640f8a8b1bd85969ea01ab9b3b056))
* Add robots.txt and sitemap.xml (DEV-3931) ([#152](https://github.com/dasch-swiss/dsp-meta/issues/152)) ([6dceb9e](https://github.com/dasch-swiss/dsp-meta/commit/6dceb9e32776f1678bfb10c144b87d1bba25c0ea))
* Expose validation command ([#88](https://github.com/dasch-swiss/dsp-meta/issues/88)) ([aa6dd91](https://github.com/dasch-swiss/dsp-meta/commit/aa6dd91f6b654b5770521735387c93d52fa50f66))
* Extend dataset domain model ([#85](https://github.com/dasch-swiss/dsp-meta/issues/85)) ([f564ff6](https://github.com/dasch-swiss/dsp-meta/commit/f564ff6a5dedd2a4a4e464c568b7acb2eba15033))
* Extend dataset domain model and a bit of deserialization (ongoing) ([#113](https://github.com/dasch-swiss/dsp-meta/issues/113)) ([070a729](https://github.com/dasch-swiss/dsp-meta/commit/070a7290a65a266e8d9c7284b7af3733c64cd490))
* Extend dataset domain model and deserialization (ongoing) ([#115](https://github.com/dasch-swiss/dsp-meta/issues/115)) ([4264c72](https://github.com/dasch-swiss/dsp-meta/commit/4264c721cd80c889175331263b591a94c999b465))
* Make service work with json (DEV-3930) ([#146](https://github.com/dasch-swiss/dsp-meta/issues/146)) ([184c710](https://github.com/dasch-swiss/dsp-meta/commit/184c71031595885389cd045375213b81ac09b00b))

## [0.1.3](https://github.com/dasch-swiss/dsp-meta/compare/dsp-meta-v0.1.2...dsp-meta-v0.1.3) (2023-12-04)


### Features

* Add project serialization to rdf (ongoing) ([#62](https://github.com/dasch-swiss/dsp-meta/issues/62)) ([2e7d33c](https://github.com/dasch-swiss/dsp-meta/commit/2e7d33cebf7633f63dcfbd904913faf547363d68))
* Add serialization to JSON (ongoing) ([#69](https://github.com/dasch-swiss/dsp-meta/issues/69)) ([fc9481f](https://github.com/dasch-swiss/dsp-meta/commit/fc9481f6abc0ab80bcf3c9e3c634789785fa089d))
* **dsp-domain:** Add ontology domain model ([#61](https://github.com/dasch-swiss/dsp-meta/issues/61)) ([c80cd83](https://github.com/dasch-swiss/dsp-meta/commit/c80cd83a5b9b749a00c84a5da5a1b4af606dbdc9))
* **dsp-meta:** Load data at startup from disk and server through api ([#70](https://github.com/dasch-swiss/dsp-meta/issues/70)) ([f82fb5a](https://github.com/dasch-swiss/dsp-meta/commit/f82fb5a1e11db731cea14c9b62458a23abb1cdbf))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * dsp-domain bumped from 0.1.2 to 0.1.3

## [0.0.2](https://github.com/dasch-swiss/dsp-meta/compare/dsp-meta-v0.0.1...dsp-meta-v0.0.2) (2023-12-04)


### Features

* Add project serialization to rdf (ongoing) ([#62](https://github.com/dasch-swiss/dsp-meta/issues/62)) ([2e7d33c](https://github.com/dasch-swiss/dsp-meta/commit/2e7d33cebf7633f63dcfbd904913faf547363d68))
* Add serialization to JSON (ongoing) ([#69](https://github.com/dasch-swiss/dsp-meta/issues/69)) ([fc9481f](https://github.com/dasch-swiss/dsp-meta/commit/fc9481f6abc0ab80bcf3c9e3c634789785fa089d))
* **dsp-domain:** Add ontology domain model ([#61](https://github.com/dasch-swiss/dsp-meta/issues/61)) ([c80cd83](https://github.com/dasch-swiss/dsp-meta/commit/c80cd83a5b9b749a00c84a5da5a1b4af606dbdc9))
* **dsp-meta:** Load data at startup from disk and server through api ([#70](https://github.com/dasch-swiss/dsp-meta/issues/70)) ([f82fb5a](https://github.com/dasch-swiss/dsp-meta/commit/f82fb5a1e11db731cea14c9b62458a23abb1cdbf))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * dsp-domain bumped from 0 to 0.0.2
