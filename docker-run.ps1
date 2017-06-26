docker rm -f rust-examples
docker run --name rust-examples -v ${pwd}:/home/build rust-examples:latest