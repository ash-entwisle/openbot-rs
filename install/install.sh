# Make OpenBot Directory
echo "Make OpenBot Directory"
mkdir -p $(pwd)/openbot
cd $(pwd)/openbot

# Get Config File
echo "Download Config File"
curl -fsSL https://raw.githubusercontent.com/ash-entwisle/openbot-rs/main/install/config.sh -o config.sh

# Get Template Env File
echo "Download Template Env File"
curl -fsSL https://raw.githubusercontent.com/ash-entwisle/openbot-rs/main/install/temp.env -o .env

# Get Docker Compose 
echo "Download Docker Compose File"
curl -fsSL https://raw.githubusercontent.com/ash-entwisle/openbot-rs/main/install/docker-compose.yaml -o docker-compose.yaml

# Get License File
echo "Download License File"
curl -fsSL https://raw.githubusercontent.com/ash-entwisle/openbot-rs/main/LICENSE -o LICENSE

# Get Readme File
echo "Download Readme File"
curl -fsSL https://raw.githubusercontent.com/ash-entwisle/openbot-rs/main/README.md -o README.md

# Done !!
echo "Done !!"
echo "Please edit the .env file and then docker-compose.yaml file"
echo "Then run docker-compose up -d to start the bot"

