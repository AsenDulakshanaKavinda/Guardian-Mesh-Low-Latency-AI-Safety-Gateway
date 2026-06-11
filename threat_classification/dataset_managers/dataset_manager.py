from transformers import AutoTokenizer
from datasets.dataset_dict import DatasetDict
from datasets import load_dataset, load_from_disk
from pathlib import Path
from typing import Optional
from utils import log, handle_errors



class DatasetManager:
    def __init__(self):
        self.repo_name = None # load from the config
        self.raw_model_path = None # load from the config
        self.trained_model_path = None # load from the config
        

    @handle_errors("dataset_load_from_HF", 5001)
    def dataset_load_from_HF(self, dataset_id: str, name: Optional[str] = None) -> DatasetDict:
        if not dataset_id:
            log.warning("No dataset id provided to load.")
            raise ValueError("No dataset id provided to load.")
        
        log.info(f"Loading dataset from HF: {dataset_id}")
        return load_dataset(path=dataset_id, name=name)
    

    @handle_errors("dataset_load_from_DISK", 5001)
    def dataset_load_from_DISK(self, dataset_path: str) -> DatasetDict:
        if not dataset_path:
            log.warning("No dataset path provided to load.")
            raise ValueError("No dataset path provided to load.")
        
        log.info(f"Loading dataset from DISK: {dataset_path}")
        return load_from_disk(dataset_path=dataset_path)
    

    @handle_errors("upload_dataset_to_HF", 5001)
    def upload_dataset_to_HF(self, dataset: DatasetDict, repo_name: str) -> None:
        if not dataset:
            log.warning("Missing dataset to upload to HF")
            raise ValueError("Missing dataset to upload to HF")
        
        if not repo_name:
            log.warning("Missing repo name to upload to HF")
            raise ValueError("Missing repo name to upload to HF")
        
        log.info("Uploading dataset to HF")
        dataset.push_to_hub(repo_name)


    @handle_errors("save_dataset_to_DISK", 5001)
    def save_dataset_to_DISK(self, dataset: DatasetDict, dataset_path: Path) -> None:
        if not dataset:
            log.warning("Missing dataset to save to DISK")
            raise ValueError("Missing dataset to save to DISK")

        if not dataset_path:
            log.warning("Missing dataset path to save to DISK")
            raise ValueError("Missing dataset path to save to DISK")
        
        log.info("Saving dataset to DISK")
        dataset.save_to_disk(str(dataset_path))


    @handle_errors("preprocess_dataset", 5001)
    def preprocess_dataset(self, tokenizer: AutoTokenizer, examples) -> DatasetDict:
        if not tokenizer:
            log.warning("Missing tokenizer")
            raise ValueError("Missing tokenizer")  
        
        log.info("Encoding dataset")
        return tokenizer(examples['text'], truncation=True, padding="max_length", max_length=128)
    










"""
tokenized_datasets = dataset.map(
        preprocess_fn,
        batched=True,                   # CRITICAL: Processes data in batches (much faster)
        remove_columns=["text"]         # Optional: Drops raw text to save memory
    )
"""

"""
    def dataset_load_from_HF(self, dataset_id: str, name: str = None) -> DatasetDict:
        if not dataset_id:
            log.warning("No dataset id provided to load.")
            raise ValueError("No dataset id provided to load.")

        try:
            log.info(f"Loading dataset from HF: {dataset_id}")
            return load_dataset(path=dataset_id, name=name)
            
        except Exception as e:
            log.error(f"Error while loading dataset from HF: {str(e)}")
            raise ThreatClassificationException(
                message=f"Error while loading dataset from HF: {str(e)}",
                component="dataset_load_from_HF",
                error_code=50001
            )
        

    def dataset_load_from_DISK(self, dataset_path: str, name: str = None) -> DatasetDict:
        if not dataset_path:
            log.warning("No dataset path provided to load.")
            raise ValueError("No dataset path provided to load.")

        try:
            log.info(f"Loading dataset from DISK: {dataset_path}")
            return load_from_disk(dataset_path=dataset_path)
            
        except Exception as e:
            log.error(f"Error while loading dataset from DISK: {str(e)}")
            raise ThreatClassificationException(
                message=f"Error while loading dataset from DISK: {str(e)}",
                component="dataset_load_from_DISK",
                error_code=50001
            )

            
    def preprocess_dataset(self, tokenizer: AutoTokenizer) -> DatasetDict:
        pass


    def upload_dataset_to_HF(self, dataset: DatasetDict, repo_name: str) -> None:
        if not dataset:
            log.warning("Missing dataset to upload to HF")
            raise ValueError("Missing dataset to upload to HF")

        if not repo_name:
            log.warning("Missing repo name to upload to HF")
            raise ValueError("Missing repo name to upload to HF")
        

        try:
            log.info("Uploading dataset to HF")
            dataset.push_to_hub(repo_name)
        except Exception as e:
            raise ThreatClassificationException(
                message=f"",
                component="upload_dataset_to_HF",
                error_code=50001
            )
        

    def save_dataset_to_DISK(self, dataset: DatasetDict, dataset_path: Path) -> None:
        if not dataset:
            log.warning("Missing dataset to save to DISK")
            raise ValueError("Missing dataset to save to DISK")

        if not dataset_path:
            log.warning("Missing dataset path to save to DISK")
            raise ValueError("Missing dataset path to save to DISK")
        

        try:
            log.info("Saving dataset to DISK")
            dataset.save_to_disk(dataset_path)
        except Exception as e:
            raise ThreatClassificationException(
                message=f"Error while saving dataset to DISK",
                component="save_dataset_to_DISK",
                error_code=50001
            )

    """




