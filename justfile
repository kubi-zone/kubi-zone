@minimal:
    cargo +nightly update -Zdirect-minimal-versions
    cargo test --locked --all-targets
    cargo update

@crd version="default":
    cargo run -p crd-exporter --no-default-features --features {{ version }}
    git diff --quiet -- crds/

@hack:
    #!/usr/bin/env bash
    kubernetes_versions=$(cargo metadata --no-deps --format-version 1 | jq '[.packages[].features][] | to_entries | map(select(.value[] | startswith("k8s"))) | map(.key)' | jq -rs '. | flatten | join(",")')
    cargo hack --feature-powerset --exclude-features default --mutually-exclusive-features "$kubernetes_versions" --at-least-one-of "$kubernetes_versions" check

@docs:
    cargo metadata --format-version 1 --no-deps | jq '.packages[] | select(.targets[].kind | contains(["lib"])) | .name' | xargs -n1 cargo +nightly docs-rs -p

@test:
    cargo +stable test --locked --all-targets
    cargo +nightly test --locked --all-targets

@fmt:
    cargo fmt --check

@semver:
    cargo semver-checks --default-features

@publish:
    cargo metadata --format-version 1 --no-deps | jq '.packages[] | select(.targets[].kind | contains(["lib"])) | .name' | xargs -n1 cargo publish --dry-run -p

@all:
    just fmt
    just crd
    just test
    just docs
    just hack
    just minimal
    just semver
