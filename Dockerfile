FROM ubuntu:jammy

# -- Configuration -- #

# Install SSH Server
RUN apt update -y && apt install openssh-server -y

# Configure SSH-Server
RUN mkdir /var/run/sshd
COPY /etc/ssh/sshd_config /etc/ssh/

# Install Sudo
RUN apt update -y && apt install sudo -y

# Load entrypoint
COPY entrypoint.sh /
RUN chmod +x /entrypoint.sh

# Sail-CLI
COPY /cli/sail/ /cli/
RUN chmod +x /cli/sail


# -- Post -- #

# Remove temp
RUN apt clean 
RUN rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*


# -- Setup -- #
EXPOSE 22
WORKDIR /cli
ENTRYPOINT ["/entrypoint.sh"]