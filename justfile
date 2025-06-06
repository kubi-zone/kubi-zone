@fetch-ci-changes:
    git remote set-url ci https://github.com/jonhoo/rust-ci-conf.git 2>/dev/null || git remote add ci https://github.com/jonhoo/rust-ci-conf.git
    git fetch ci
    git rebase ci/main

@minimal-versions:
    cargo +nightly update -Zdirect-minimal-versions
    cargo test --locked --features latest --all-targets
    cargo update

@dump-crds version="latest":
    cargo metadata --no-deps | jq -r '.packages[] | select(.name | endswith("-crds")) | .name' \
    | xargs -n1 cargo run --features {{ version }} --example dump -p

@hack:
    bash -c "$(yq '.jobs.hack.steps[] | select(.name == "cargo hack").run' .github/workflows/check.yml)"

@test:
    just minimal-versions
    just hack