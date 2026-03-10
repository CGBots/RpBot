#/bin/bash

#installation de mongo
sudo apt-get install gnupg curl
curl -fsSL https://www.mongodb.org/static/pgp/server-8.0.asc | \
   sudo gpg -o /usr/share/keyrings/mongodb-server-8.0.gpg \
   --dearmor
echo "deb [ arch=amd64,arm64 signed-by=/usr/share/keyrings/mongodb-server-8.0.gpg ] https://repo.mongodb.org/apt/ubuntu noble/mongodb-org/8.0 multiverse" | sudo tee /etc/apt/sources.list.d/mongodb-org-8.0.list
sudo apt-get update
sudo apt-get install -y mongodb-org
sudo rm -rf /tmp/mongodb-27017.sock
sudo systemctl start mongod
sudo systemctl enable mongod

#installation de rust POUR WSL
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

#installation de dépendances
sudo apt-get install gcc