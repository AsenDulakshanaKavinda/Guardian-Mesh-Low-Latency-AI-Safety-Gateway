from datasets import load_dataset
from transformers import AutoTokenizer
from transformers import DataCollatorWithPadding

class TrainingDataset:
    """
    This class is responsible for loading and preprocessing the dataset for training.
     - It loads the raw dataset using the Hugging Face Datasets library.
     - It provides a method to get the loaded dataset for further processing.
     - It also handles any exceptions that may occur during dataset loading and provides informative error messages.
    """

    def __init__(self, dataset_name, dataset_version = "core"):
        self.dataset_name = dataset_name
        self.dataset_version = dataset_version
        self.dataset_dict = None


    def load_raw_dataset(self) -> None:
        """ Load the raw dataset using the Hugging Face Datasets library. """

        try:
            self.dataset_dict = load_dataset(
                self.dataset_name,
                self.dataset_version 
            )
        except Exception as e:
            raise RuntimeError(f"Error while loading the dataset: {str(e)}")
        

    def preprocess(self, tokenizer: AutoTokenizer, text_col):
        """ Preprocess the dataset by tokenizing the text data. """
        def tokenize_fn(example):
            return tokenizer(
                example[text_col],
                truncation = True,
                padding = "max_length"
            )
        
        self.dataset = self.dataset.map(tokenize_fn, batched=True)
        

    def get_dataset(self) -> dict:
        """ Get the loaded dataset for further processing. """

        if not self.dataset_dict:
            raise ValueError("Dataset not loaded. Please call load_raw_dataset() first.")
        return self.dataset_dict
    

    def get_data_collector(self, tokenizer: AutoTokenizer) -> DataCollatorWithPadding:
        """ Get the data collator for padding the inputs during training. """
        return DataCollatorWithPadding(tokenizer=tokenizer)
