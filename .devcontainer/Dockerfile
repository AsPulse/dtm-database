FROM mcr.microsoft.com/devcontainers/rust:0-1-bullseye@sha256:b723b13821b195ee9ee076b3182dff8e7f50c47d163d1dc188152b166252ed6e
USER vscode
RUN cargo install cargo-watch

# https://github.com/pnpm/pnpm/issues/3319#issuecomment-1165283772
RUN curl -fsSL https://get.pnpm.io/install.sh | SHELL=`which bash` bash -
