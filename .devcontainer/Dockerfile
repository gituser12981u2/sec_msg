FROM rust:latest

RUN apt-get update && apt-get install -y --no-install-recommends \
    git \
    gnupg \
    sudo \
    && rm -rf /var/lib/apt/lists/* \
    && echo "Base packages installed"

# Set up user
ARG USERNAME=vscode
ARG USER_UID=1000
ARG USER_GID=${USER_UID}
RUN groupadd --gid ${USER_GID} ${USERNAME} \
    && useradd -s /bin/bash --uid ${USER_UID} --gid ${USER_GID} -m ${USERNAME} \
    && echo ${USERNAME} ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/${USERNAME} \
    && chmod 0440 /etc/sudoers.d/${USERNAME}

# Give vscode user ownership of cargo directories
RUN mkdir -p /usr/local/cargo \
    && chown -R ${USERNAME}:${USERNAME} /usr/local/cargo

# Create .gnupg directory with correct permissions
RUN mkdir -p /home/${USERNAME}/.gnupg && \
    chown -R ${USERNAME}:${USERNAME} /home/${USERNAME}/.gnupg && \
    chmod 700 /home/${USERNAME}/.gnupg

# Copy git hooks and set permissions as root
COPY .git-hooks /tmp/.git-hooks
RUN chmod +x /tmp/.git-hooks/* && \
    chown -R ${USERNAME}:${USERNAME} /tmp/.git-hooks

# Copy script and set permissions
COPY check-git-config.sh setup-gpg.sh /usr/local/bin/
RUN chmod +x /usr/local/bin/check-git-config.sh /usr/local/bin/setup-gpg.sh

# Switch to vscode user for the rest of the setup
USER ${USERNAME}

# Set up rust environment
RUN rustup component add clippy rustfmt && \
    cargo install cargo-edit

# Set up git hooks
RUN git config --global core.hooksPath /tmp/.git-hooks

# Set up GPG agent
RUN echo "allow-loopback-pinentry" > ~/.gnupg/gpg-agent.conf && \
    echo "pinentry-mode loopback" > ~/.gnupg/gpg.conf

WORKDIR /workspace

# Set environment variables
ENV CARGO_HOME=/usr/local/cargo
ENV RUSTUP_HOME=/usr/local/rustup
ENV PATH="/usr/local/cargo/bin:${PATH}"
