from gen import ner_proto_pb2_grpc
from gen import ner_proto_pb2
from model.ner_model import ner_model

import grpc
from concurrent import futures


class NERServiceServicer(ner_proto_pb2_grpc.NERService):

    def __init__(self):
        self.nlp = ner_model

    def ExtractEntities(self, request, content):
        prompt = self.nlp(request.prompt_str) 
        entities = []
        for ent in prompt.ents:
            entity_msg = ner_proto_pb2.Entity(
                labal = ent.label,
                start = ent.start,
                end = ent.end,
            )
            entities.append(entity_msg)



    
def serve():
    server = grpc.server(futures.ThreadPoolExecutor(max_workers=10))
    ner_proto_pb2_grpc.add_NERServiceServicer_to_server(NERServiceServicer(), server)
    listen_addr = "localhost:10001"
    server.add_insecure_port(listen_addr)
    print(f"Starting server on {listen_addr}")
    server.start()
    server.wait_for_termination()

if __name__ == "__main__":

    serve()