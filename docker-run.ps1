docker rm -f rust-challenge
docker run --name rust-challenge -v ${pwd}:/home/build rust-challenge:latest