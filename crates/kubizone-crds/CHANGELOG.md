## 0.14.0

### Changed
* Updated `kubizone-common` to `v0.15.0`


## 0.12.4

### Added
* Implement `From<ZoneEntry>` for `kubizone_common::RecordIdent`.


## 0.12.3

### Changed
* Updated to kubizone_common v0.13.2

### Added
* Implement `TryFrom<Record>` for the new `kubizone_common::RecordIdent`.


## 0.12.2

### Fixed
* Improper parsing of ZoneRefs from strings


## 0.12.1

### Changed
* Updated kube-rs to v0.92.0


## 0.12.0

### Changed
* Introduced `DomainExt` trait which covers functionality common between Zones and Records, such as fetching computed Fully Qualified Domain Name, parsing parent zone labels.

### Added
* Implemented `From<str/String/&str>` for ZoneRef.


## 0.11.3

### Fixed
* Extend the dev/prod schema split to parent zone labels as well


## 0.11.2

### Added
* Kubernetes API version feature selector.


## 0.11.1

### Added
* Introduce 'dev' feature for toggling between `kubi.zone` and `dev.kubi.zone` api groups.
* Add 'dump' example for writing out the CRDs to yaml files: `cargo run [--features dev] --example dump`

### Fixed
* Updated k8s-openapi to v0.22.0
* Updated kube-rs to v0.91.0