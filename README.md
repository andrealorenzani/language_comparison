# language_comparison
Language comparison for insertion sort

# Run on local
## NodeJS
on easy_node run
node app.js

## Java
on easy_spring run
./gradlew bootRun

## Rust
on easy_rus run
cargo run --release

# Create docker
## NodeJS
on easy_node run
docker build . -t andlorenzani/simple-node-server:0.0.1
to run it
docker run -p 49160:8080 -d andlorenzani/simple-node-server:0.0.1
to stop it
docker ps
docker stop <CONTAINER ID>

## Java
on easy_spring run
./gradlew jibDockerBuild
to run it
docker run -p 49161:8080 -d andlorenzani/simple-boot-server:0.0.1
to stop it
docker ps
docker stop <CONTAINER ID>

## Rust
on easy_spring run
cargo build --release
docker build . -t andlorenzani/simple-rust-server:0.0.1
to run it
docker run -p 49162:7878 -d andlorenzani/simple-rust-server:0.0.1
to stop it
docker ps
docker stop <CONTAINER ID>