FROM mcr.microsoft.com/devcontainers/rust:2-1-bullseye

RUN sudo apt-get update && \ 
    sudo apt-get upgrade && \
    curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash && \
    cargo install diesel_cli && \
    apt-get clean