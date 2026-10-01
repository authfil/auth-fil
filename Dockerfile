# syntax=docker/dockerfile:1
#
# Authfil development and CI image. One toolchain for every language the
# adapters target, so the conformance suite runs the same everywhere.
#
#   dev  - Rust + Node + Python toolchains, used by compose for local work
#   ci   - dev plus a copy of the source, runs `make ci` during the build

ARG RUST_VERSION=1.98
ARG NODE_VERSION=24
ARG DEBIAN_RELEASE=trixie

FROM node:${NODE_VERSION}-${DEBIAN_RELEASE}-slim AS node

FROM rust:${RUST_VERSION}-${DEBIAN_RELEASE} AS tools
RUN cargo install --locked cargo-deny

FROM rust:${RUST_VERSION}-${DEBIAN_RELEASE} AS dev

ARG USER_UID=1000
ARG USER_GID=1000

RUN apt-get update \
    && apt-get install -y --no-install-recommends python3 python3-venv \
    && rm -rf /var/lib/apt/lists/*

COPY --from=node /usr/local/bin/node /usr/local/bin/node
COPY --from=node /usr/local/lib/node_modules /usr/local/lib/node_modules
RUN ln -s ../lib/node_modules/npm/bin/npm-cli.js /usr/local/bin/npm \
    && ln -s ../lib/node_modules/npm/bin/npx-cli.js /usr/local/bin/npx

COPY --from=ghcr.io/astral-sh/uv:latest /uv /usr/local/bin/uv
COPY --from=tools /usr/local/cargo/bin/cargo-deny /usr/local/cargo/bin/cargo-deny

RUN rustup component add rustfmt clippy

# Non-root user matching the host UID, so files written to the bind mount
# are not owned by root. Volume mount points are created here so compose's
# named volumes inherit this ownership.
RUN groupadd --gid ${USER_GID} dev \
    && useradd --uid ${USER_UID} --gid ${USER_GID} --create-home dev \
    && mkdir -p /usr/local/cargo/registry /workspace/target \
    && chown -R dev:dev /usr/local/cargo /workspace

USER dev
ENV VIRTUAL_ENV=/home/dev/.venv \
    PATH=/home/dev/.venv/bin:$PATH
RUN uv venv "$VIRTUAL_ENV" && uv pip install maturin

WORKDIR /workspace
CMD ["bash"]

FROM dev AS ci
COPY --chown=dev:dev . .
RUN make ci
