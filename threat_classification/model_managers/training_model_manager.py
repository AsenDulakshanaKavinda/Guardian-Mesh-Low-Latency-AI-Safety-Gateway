from transformers import AutoModelForSequenceClassification, AutoTokenizer
from transformers import AutoTokenizer
import numpy as np
from pathlib import Path
from typing import Optional

from utils import log, handle_errors


class TrainingModelManager:
    def __init__(self, pretrained_model_name_or_path: str) -> tuple[
        AutoModelForSequenceClassification, 
        AutoTokenizer
    ]:
        self.pretrained_model_name_or_path = pretrained_model_name_or_path

    @handle_errors("model_loading", 6001)
    def model_loading(self):
        if not self.pretrained_model_name_or_path:
            log.warning("pretrained_model_name_or_path is missing")
            ValueError("pretrained_model_name_or_path is missing")

        log.info(f"Loading model: {self.pretrained_model_name_or_path}")
        model =  AutoModelForSequenceClassification.from_pretrained(
            pretrained_model_name_or_path = self.pretrained_model_name_or_path,
        )

        log.info(f"Loading tokenizer: {self.pretrained_model_name_or_path}")
        tokenizer = AutoTokenizer.from_pretrained(
            pretrained_model_name_or_path = self.pretrained_model_name_or_path
        )

        return (model, tokenizer)


    
    @handle_errors("save_model_to_DISK", 6001)
    def save_model_to_DISK(self, save_location: Path, model: AutoModelForSequenceClassification, tokenizer: AutoTokenizer) -> None:
        if not save_location:
            raise ValueError("Missing save location")

        if not model:
            raise ValueError("Model is missing")

        if not tokenizer:
            raise ValueError("Tokenizer is missing")

        log.info(f"saving model and tokenizer to: {str(save_location)}")
        model.save_pretrained(str(save_location))
        tokenizer.save_pretrained(str(save_location))



    @handle_errors("upload_model_to_HF", 6001)
    def upload_model_to_HF(self, repo: str, model: AutoModelForSequenceClassification, tokenizer: AutoTokenizer) -> None:
        if not repo:
            raise ValueError("Missing repo")  
        
        if not model:
            raise ValueError("Missing model") 
        
        if not tokenizer:
            raise ValueError("Missing tokenizer") 

        log.info(f"Uploading model and tokenizer to Hugging Face repo: {repo}")
        model.push_to_hub(repo)
        tokenizer.push_to_hub(repo)

    
        
        
