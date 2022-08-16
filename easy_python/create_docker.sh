docker build . -t andlorenzani/simple-python-server:0.0.1
echo "use: docker run -p 49163:49163 -d andlorenzani/simple-python-server:0.0.1"
echo "invoke: curl -i -X POST localhost:49163/test   -H 'Content-Type: application/json'   --data-binary '@curl.txt'"