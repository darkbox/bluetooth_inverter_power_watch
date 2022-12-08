#!/bin/bash
sudo mkdir -p /bin/rgm
sudo cp bt /bin/rgm/bt
sudo cp .env /bin/rgm/.env
sudo cp bt.service /etc/systemd/system/bt.service
sudo systemctl daemon-reload
sudo systemctl enable bt.service
systemctl list-unit-files --type=service --all | grep bt.service
echo "Done\n"