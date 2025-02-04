# Helper Scripts

## build.sh

build all the parts

## package.sh

make a tar the files need for server

## install.sh

Install

### TODO: Need

- [ ] Install libs (`openssl`)
```shell
sudo yum update
# TODO: get openssl 3 
sudo yum install openssl openssl-libs
sudo amazon-linux-extras enable mariadb10.5
sudo yum clean metadata
sudo yum install mariadb-libs
```
> mariadb-connector-c ?
- [ ] Setup server user (`status-reports`)
```shell
sudo useradd -m status-reports
```
- [ ] Get server
```shell
# TODO: download https://github.com/tztz8/EWU-CSCD488-490-Senior-Project/releases
# unzip
tar -xvzf bin.tar.gz
```
- [ ] Copy server into user (`/home/status-reports/server`)
```shell
echo "Make program folder"
sudo su - status-reports -c "mkdir -p /home/status-reports/server"
echo "Executables"
sudo cp target/release/backend /home/status-reports/server/
sudo cp target/release/dbcli /home/status-reports/server/
echo "SQL setup files"
sudo cp -r database/migrations /home/status-reports/server/
echo "File to allow run in the background"
sudo cp helper_scripts/srs-actix.service /home/status-reports/server/
sudo cp helper_scripts/run.sh /home/status-reports/server
echo "Static files"
sudo cp -r studentpage/dist /home/status-reports/server/studentpage/dist
sudo cp -r adminpage/dist /home/status-reports/server/adminpage/dist
sudo cp -r res /home/status-reports/server/
echo "Config files"
# TODO: Wait for IT to config server
sudo cp secret.config.toml /home/status-reports/server/
sudo cp server.config.toml /home/status-reports/server/
echo "Set file permissions"
sudo chown -R status-reports:status-reports /home/status-reports/server/
echo "Allow/link service"
sudo systemctl link /home/status-reports/server/srs-actix.service
```
- [ ] Copy SSL cert into user (`/home/status-reports/server`)
```shell
SSL_CERTIFICATE="/etc/letsencrypt/live/aws.tftinker.tech/fullchain.pem"
SSL_KEY="/etc/letsencrypt/live/aws.tftinker.tech/privkey.pem"

sudo cp $SSL_CERTIFICATE /home/status-reports/server/fullchain.pem
sudo chown status-reports:status-reports /home/status-reports/server/fullchain.pem
sudo su - status-reports -c "chmod 400 /home/status-reports/server/fullchain.pem"

sudo cp $SSL_KEY /home/status-reports/server/privkey.pem
sudo chown status-reports:status-reports /home/status-reports/server/privkey.pem
sudo su - status-reports -c "chmod 400 /home/status-reports/server/privkey.pem"
```
- [ ] Setup DB
Code missing
- [ ] Lock server user
```shell
sudo usermod -s /usr/sbin/nologin status-reports
```
- [ ] Make `srs-actix.service` file
```unit file (systemd)
[Unit]
Description=Status Repoting System Server in Rust actix
After=network.target
#After=mysqld.service # Uncomment when using mysql on same server
StartLimitIntervalSec=0

[Service]
Type=simple
#Restart=always
#RestartSec=1
#StartLimitBurst=5
#StartLimitIntervalSec=10
User=status-reports
ExecStart=/usr/bin/sh /home/status-reports/server/run.sh

[Install]
WantedBy=multi-user.target
```
- [ ] Start server
```shell
sudo systemctl start srs-actix
sudo systemctl status srs-actix
```

### TODO: Want

- [ ] Certbot (**Before** copy SSL cert)
```shell
sudo dnf update
sudo dnf upgrade -y
sudo dnf install python3 augeas-libs
echo -e "$Cyan Install certbot ... $Color_Off"
sudo python3 -m venv /opt/certbot/
sudo /opt/certbot/bin/pip install --upgrade pip
sudo /opt/certbot/bin/pip install certbot certbot
sudo ln -s /opt/certbot/bin/certbot /usr/bin/certbot
echo -e "$Cyan Getting ssl cert using certbot ... $Color_Off"
sudo certbot certonly --standalone
```
- [ ] Have user make config files (**Before** Start server)

## dev_*.sh

helper script for devlement