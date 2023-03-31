# -- SAIL-CLI -- #
FROM rust:1.68.2 AS sailcli

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