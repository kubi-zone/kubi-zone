@minimal-versions:
    cargo +nightly update -Zdirect-minimal-versions
    cargo test --locked --all-targets
    cargo update

@dump-crds version="default":
    cargo metadata --format-version 1 --no-deps | jq -r '.packages[] | select(.name | endswith("-crds")) | .name' \
    | xargs -n1 cargo run --no-default-features --features {{ version }} --example dump -p

@hack:
    #!/usr/bin/env bash
    kubernetes_versions=$(cargo metadata --no-deps | jq '[.packages[].features][] | to_entries | map(select(.value[] | startswith("k8s"))) | map(.key)' | jq -rs '. | flatten | join(",")')
    cargo hack --feature-powerset --exclude-features default --mutually-exclusive-features "$kubernetes_versions" --at-least-one-of "$kubernetes_versions" check

@docs-rs:
    cargo metadata --format-version 1 --no-deps | jq '.packages[].name' | xargs -n1 cargo +nightly docs-rs -p

@test:
    cargo +stable test --locked --all-targets
    cargo +nightly test --locked --all-targets

@fmt:
    cargo fmt --check

@all:
    just minimal-versions
    just docs-rs
    just hack
    just test
