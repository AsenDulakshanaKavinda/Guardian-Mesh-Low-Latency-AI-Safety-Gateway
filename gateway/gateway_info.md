## Overview

The gateway have the following components:
1. ML Service: Responsible for classifying the incoming request either the user prompt is involved in prompt injection attack or not. It will return the classification result to the gateway.

2. NRE Service: Responsible for extracting the named entities from the user prompt. It will return the extracted named entities to the gateway.

3. Prompt Rewriting Service: Responsible for rewriting the user prompt based on the extracted named entities. It will return the rewritten prompt to the gateway.

4. Invocation Service: Responsible for invoking the LLM with the rewritten prompt and returning the response to the gateway.


## Workflow. 
1. ML Service - The gateway receives the user prompt and sends it to the ML Service for classification. User `GRPC request` is sent to the ML Service, and the ML Service will return the classification result to the gateway.

2. NRE Service - If the classification result indicates that the user prompt is not involved in a prompt injection attack, the gateway will send the user prompt to the NRE Service for named entity recognition. User `GRPC request` is sent to the NRE Service, and the NRE Service will return the extracted named entities to the gateway.

3. Prompt Rewriting - The gateway use the extracted named entities to rewrite the user prompt. The gateway rewrites the user prompt based on the extracted named entities and the original user prompt. The rewritten prompt is then sent to the Invocation Service for invoking the LLM.

4. Invocation Service - The gateway sends the rewritten prompt to the Invocation Service for invoking the LLM and returns the response to the user. 