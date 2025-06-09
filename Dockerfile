##### Builder
FROM rust:1.85.1-slim AS builder
ARG UID=65203
ARG GID=65203

RUN apt-get update && apt-get install musl-tools -y

RUN adduser                 \
    --disabled-password     \
    --gecos ""              \
    --home "/nonexistent"   \
    --shell "/sbin/nologin" \
    --no-create-home        \
    --uid "${UID}"          \
    --uid "${GID}"          \
    "kubizone"

RUN mkdir -p /usr/src/kubizone
COPY . /usr/src/kubizone/

WORKDIR /usr/src/kubizone/

# Build it and copy the resulting binary into 
# /usr/local/bin since cache directories become
# inaccessible at the end of the running command.
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/src/kubizone/target  \
    cargo build --release  &&                           \
    cp -r /usr/src/kubizone/target/release/kubizone /usr/local/bin/kubizone

FROM scratch AS kubizone
LABEL org.opencontainers.image.source=https://github.com/kubi-zone/kubi-zone
ARG UID=65203
ARG GID=65203
COPY --from=builder --chown=${UID}:${GID} --chmod=0440 /etc/passwd /etc/passwd
COPY --from=builder --chown=${UID}:${GID} --chmod=0440 /etc/group /etc/group
COPY --from=builder --chown=${UID}:${GID} --chmod=0550 /usr/local/bin/kubizone /app/kubizone
USER ${UID}:${GID}

ENTRYPOINT ["/app/kubizone"]
CMD ["help"]
