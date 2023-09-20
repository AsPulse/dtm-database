FROM mcr.microsoft.com/devcontainers/rust:0-1-bullseye
USER vscode
RUN cargo install cargo-watch

# https://github.com/pnpm/pnpm/issues/3319#issuecomment-1165283772
RUN curl -fsSL https://get.pnpm.io/install.sh | SHELL=`which bash` bash -
