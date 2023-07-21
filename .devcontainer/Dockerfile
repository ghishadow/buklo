
ARG ALPINE_VERSION="3.18"
FROM mcr.microsoft.com/vscode/devcontainers/base:0-alpine-${ALPINE_VERSION}

USER root

# https://nvd.nist.gov/vuln/detail/CVE-2019-5021
RUN sed -i -e 's/^root::/root:!:/' /etc/shadow 

RUN apk update \
    && apk upgrade \
    && apk add openssl direnv git curl bash sudo xz \
    && rm -rf /var/cache/apk/*

# See here for how to integrate Nix into Alpine: https://github.com/redoracle/nixos/blob/master/Dockerfile
ARG NIX_VERSION="2.9.2"
ARG NIX_SHA256="e5fd4e993bc6c565aa0544eec2db6a977e496896f4133002436b1be1d3e9a8f6"

WORKDIR /
RUN wget https://releases.nixos.org/nix/nix-${NIX_VERSION}/nix-${NIX_VERSION}-x86_64-linux.tar.xz -O nix.tar.xz

RUN cat nix.tar.xz | xz -d | tar xp \
    && mv nix-*-linux nix-release \
    && rm nix.tar.xz

ARG USERNAME=vscode
RUN addgroup -g 30000 -S nixbld \
    && for i in $(seq 1 30); do adduser -S -D -h /var/empty -g "Nix build user $i" -u $((30000 + i)) -G nixbld nixbld$i ; done \
    && mkdir -m 0755 /etc/nix \
    && echo 'sandbox = false' > /etc/nix/nix.conf \
    && mkdir -m 0755 /nix \
    && chown -R ${USERNAME} /nix /etc/nix

USER ${USERNAME}
RUN USER=${USERNAME} sh nix-release/install

USER root 
RUN rm -r /nix-release \
    && ln -s /nix/var/nix/profiles/per-user/${USERNAME}/profile/etc/profile.d/nix.sh /etc/profile.d/

USER ${USERNAME}
RUN . $HOME/.nix-profile/etc/profile.d/nix.sh \
    && $HOME/.nix-profile/bin/nix-collect-garbage --delete-old \
    && $HOME/.nix-profile/bin/nix-store --optimise \
    && $HOME/.nix-profile/bin/nix-store --verify --check-contents

ONBUILD ENV \
    ENV=/etc/profile \
    USER=${USERNAME} \
    PATH=$HOME/.nix-profile/bin:/nix/var/nix/profiles/per-user/${USERNAME}/profile/bin:/nix/var/nix/profiles/per-user/${USERNAME}/profile/sbin:/bin:/sbin:/usr/bin:/usr/sbin \
    NIX_PATH=/nix/var/nix/profiles/per-user/${USERNAME}/channels

ENV \
    ENV=/etc/profile \
    USER=${USERNAME} \
    PATH=$HOME/.nix-profile/bin:/nix/var/nix/profiles/per-user/vscode/profile/bin:/nix/var/nix/profiles/per-user/vscode/profile/sbin:/bin:/sbin:/usr/bin:/usr/sbin \
    NIX_PATH=/nix/var/nix/profiles/per-user/vscode/channels
