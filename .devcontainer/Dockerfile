FROM mcr.microsoft.com/devcontainers/rust:1-1-bullseye@sha256:6817e2f836956c5034933e52f305b6c1112b628d3fbad0e87f472a2839ad5fe0
USER vscode
RUN cargo install cargo-watch

# https://github.com/pnpm/pnpm/issues/3319#issuecomment-1165283772
RUN curl -fsSL https://get.pnpm.io/install.sh | SHELL=`which bash` bash -
