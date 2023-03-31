#!/bin/bash

INIT="/var/init"
if [ ! -e $INIT ]; then
        touch $INIT
        echo "-- Verify SSH Server --"
        ssh-keygen -A
        echo "-- Start SSH Server --"
        /usr/sbin/sshd -D
        echo "-- Stopped SSH Server --"
else
        echo "-- Start SSH Server --"
        /usr/sbin/sshd -D
        echo "-- Stopped SSH Server --"
fi
