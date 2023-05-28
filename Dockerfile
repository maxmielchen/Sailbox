# -- SAIL-CLI -- #
FROM rustlang/rust:nightly AS sailcli

# Mirror source
COPY /sail /sail

# Build source
WORKDIR /sail
RUN cargo build --release

# Place Unix Executable
RUN cp /sail/target/release/sail /usr/local/sbin/



# -- SAILBOX -- #
FROM ubuntu:jammy

# -- Configuration -- #

# Install SSH Server
RUN apt update -y && apt install openssh-server -y

# Configure SSH-Server
RUN mkdir /var/run/sshd
COPY /etc/ssh/sshd_config /etc/ssh/

# Install Sudo
RUN apt update -y && apt install sudo -y

# Install Docker
RUN apt update -y
RUN apt install ca-certificates curl gnupg -y
RUN install -m 0755 -d /etc/apt/keyrings
RUN curl -fsSL https://download.docker.com/linux/ubuntu/gpg | gpg --dearmor -o /etc/apt/keyrings/docker.gpg
RUN chmod a+r /etc/apt/keyrings/docker.gpg
RUN echo \
  "deb [arch="$(dpkg --print-architecture)" signed-by=/etc/apt/keyrings/docker.gpg] https://download.docker.com/linux/ubuntu \
  "$(. /etc/os-release && echo "$VERSION_CODENAME")" stable" | \
  tee /etc/apt/sources.list.d/docker.list > /dev/null
RUN apt update -y
RUN apt install docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin -y

# Install Packages
RUN apt update -y
RUN apt install git -y
RUN apt install vim -y
RUN apt install neovim -y
RUN apt install nano -y
RUN apt install curl -y

# Load entrypoint
COPY entrypoint.sh /
RUN chmod +x /entrypoint.sh

# Sail-CLI
COPY --from=sailcli /usr/local/sbin/sail /usr/local/sbin/
RUN chmod +x /usr/local/sbin/sail


# -- Post -- #

# Remove temporary
RUN apt clean 
RUN rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*


# -- Setup -- #
EXPOSE 22
ENTRYPOINT ["/entrypoint.sh"]
