FROM archlinux:base-devel

RUN pacman -Syyu --noconfirm \
    python \
    python-pip \
    gcc \
    gdb \
    git \
    rustup \
    vi \
    xclip

RUN rustup default stable

WORKDIR /workspace