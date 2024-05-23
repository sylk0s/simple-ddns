docker build -t simple-ddns .
docker run -d -e TOKEN=$(cat token) -e ZONE_NAME=sylkos.xyz -e RECORD_NAME=home.sylkos.xyz --name simple_ddns simple-ddns