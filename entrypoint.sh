#!/bin/bash

log_info() {
    echo "[ $(date '+%Y-%m-%d %H:%M:%S') ] [INFO] $1"
}

INIT="/var/init"
SSHD_PID=""

handle_stop() {
    log_info "Container stop requested. Stopping SSH server..."
    if [ -n "$SSHD_PID" ] && kill -0 "$SSHD_PID" 2>/dev/null; then
        kill "$SSHD_PID"
        wait "$SSHD_PID"
        log_info "SSH server stopped. Container will now exit."
    fi
    exit 0
}

trap handle_stop SIGTERM SIGINT

if [ ! -e $INIT ]; then
    touch $INIT
    log_info "Verify SSH Server"
    ssh-keygen -A
    log_info "Add Docker socket permissions"
    chmod 666 /var/run/docker.sock
    chgrp docker /var/run/docker.sock
    log_info "Start SSH Server"
    /usr/sbin/sshd -D &
    SSHD_PID=$!
    wait $SSHD_PID
    log_info "Stopped SSH Server"
else
    log_info "Add Docker socket permissions"
    chmod 666 /var/run/docker.sock
    chgrp docker /var/run/docker.sock
    log_info "Start SSH Server"
    /usr/sbin/sshd -D &
    SSHD_PID=$!
    wait $SSHD_PID
    log_info "Stopped SSH Server"
fi
