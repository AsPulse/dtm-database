FROM mcr.microsoft.com/devcontainers/rust:1-1-bullseye@sha256:066878f166fdc8ee717ecb3f94d4155dac857a7465a39af85509314f5abc1bbf
USER vscode
RUN cargo install cargo-watch

# https://github.com/pnpm/pnpm/issues/3319#issuecomment-1165283772
RUN curl -fsSL https://get.pnpm.io/install.sh | SHELL=`which bash` bash -
