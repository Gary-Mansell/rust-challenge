docker rm -f rust-challenge
docker run --name rust-challenge -p 8000:8000 -v ${pwd}:/home/build rust-challenge:latest