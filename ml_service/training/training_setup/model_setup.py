
from transformers import AutoTokenizer, AutoModelForSequenceClassification
from transformers import pipeline


class TrainingModel:
    """
    This class is responsible for setting up the model for training.
     - It loads the base model and tokenizer from the specified path.
     - It allows setting specific parameters as trainable while freezing the rest.
     - It provides methods to get the trainable model and tokenizer for further use in training.
     - It also includes a method to upload the trained model and tokenizer to the Hugging Face Hub for easy sharing and deployment.
     - It also handles any exceptions that may occur during model loading and provides informative error messages.
    """

    def __init__(self, model_name: str, model_save_path: str, num_labels: int, id2label: dict, label2id: dict):
        self.model_name = model_name
        self.model_path = model_save_path

        self.id2label: dict = id2label
        self.label2id: dict = label2id
        self.num_labels: int = num_labels

        self.model = None
        self.tokenizer = None


    # load the base model and tokenizer from the specified path
    def load_base_model(self) -> None:
        if self.model is None:
            self.model = AutoModelForSequenceClassification.from_pretrained(
                model_name = self.model_name,
                num_labels = self.num_labels,
                id2label = self.id2label,
                label2id = self.label2id,
            )
        
        if self.tokenizer is None:
            self.tokenizer = AutoTokenizer.from_pretrained(
                model_name = self.model_name
            )


    # set trainable parameters
    def set_trainable_parameters(self, params: list) -> None:
        for name, param in self.model.named_parameters():
            if name in params:
                param.requires_grad = True
            else:
                param.requires_grad = False


    def get_trainable_model(self) -> AutoModelForSequenceClassification:
        if self.model is None:
            raise ValueError("Model not loaded. Please call load_base_model() first.")
        return self.model


    def get_tokenizer(self) -> AutoTokenizer:
        if self.tokenizer is None:
            raise ValueError("Tokenizer not loaded. Please call load_base_model() first.")
        return self.tokenizer
    
    
    def upload_model_hf(self) -> None:
        try:
            self.model.save_pretrained(self.model_path)
            self.tokenizer.save_pretrained(self.model_path)
            print(f"Model and tokenizer saved locally at {self.model_path}")

            self.model.push_to_hub(self.model_path)
            self.tokenizer.push_to_hub(self.model_path)
            print(f"Model and tokenizer uploaded to Hugging Face Hub at {self.model_path}")
        except Exception as e:
            print(f"Error uploading model to Hugging Face Hub: {e}")
            raise RuntimeError(f"Failed to upload model to Hugging Face Hub: {e}")
    
    

        