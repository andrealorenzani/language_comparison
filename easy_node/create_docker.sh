npm i
docker build . -t andlorenzani/simple-node-server:0.0.1
echo "use: docker run -p 49160:8081 -d andlorenzani/simple-node-server:0.0.1"
echo "invoke: curl --location --request POST 'localhost:8081/' --header 'Content-Type:application/json' --data @curl.txt"