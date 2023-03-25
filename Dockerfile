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
COPY /usr/local/sbin/ /usr/local/sbin/
RUN chmod +x /usr/local/sbin/sail

# -- Packages
RUN apt update -y
RUN apt install git -y
RUN apt install vim -y
RUN apt install neovim -y
RUN apt install nano -y
RUN apt install curl -y


# -- Post -- #

# Remove temp
RUN apt clean 
RUN rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*


# -- Setup -- #
EXPOSE 22
ENTRYPOINT ["/entrypoint.sh"]