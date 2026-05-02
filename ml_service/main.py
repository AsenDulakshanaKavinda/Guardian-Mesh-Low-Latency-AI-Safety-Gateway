from utils import get_logger, config
import grpc
from concurrent import futures

import ml_server_proto_pb2
import ml_server_proto_pb2_grpc

log = get_logger(__file__)


    
class ClassificationServer(ml_server_proto_pb2_grpc.PromptClassificationServiceServicer):
    def __init__(self):
        pass

    def ClassifyPrompts(self, request, context):
        return super().ClassifyPrompts(request, context)



def server():
    try:
        server = grpc.server(futures.ThreadPoolExecutor(max_workers=10))

        ml_server_proto_pb2_grpc.add_PromptClassificationServiceServicer_to_server(
            servicer=ClassificationServer(), server=server
        )

        server.add_insecure_port('[::]:50052')
        server.start()

        log.info("Server started on port 50052")
        server.wait_for_termination()


    except Exception as e:
        log.error("Error while starting the server.")
        raise RuntimeError(f"Error while starting the server: {str(e)}")



def main():
    log.info("Hello from ml-service!")
    print(config)

if __name__ == "__main__":
    main()
