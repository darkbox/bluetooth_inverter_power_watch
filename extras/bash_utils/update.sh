#!/bin/bash

if [ "$(id -u)" -ne 0 ]; then echo "Please run as root." >&2; exit 1; fi
# From now on there is no need to prefix your commands with "sudo"

BIN_NAME='bt'
SERVICE_NAME='bt'
SERVICE_BIN_DIR='/bin/rgm/'

echo 'Stopping BT service...'
systemctl stop $SERVICE_NAME

echo 'Updating binary file...'
cp $BIN_NAME $SERVICE_BIN_DIR$BIN_NAME

echo 'Starting BT service...'
systemctl start $SERVICE_NAME

echo -e '\033[0;32mDONE!'

