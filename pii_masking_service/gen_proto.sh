python3 -m grpc_tools.protoc \
    -I./proto \
    --python_out=./gen \
    --grpc_python_out=./gen \
    ./proto/ner_proto.proto