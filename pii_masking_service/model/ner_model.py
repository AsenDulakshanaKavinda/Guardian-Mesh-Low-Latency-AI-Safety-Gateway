
import spacy


def load_nlp_model(model_name="en_core_web_sm"):
    # This function loads the specified spaCy model. 
    # If the model is not found, it attempts to download it and then load it again.
    try:
        print(f"Loading: {model_name} model")
        # use only PII/Entities, disable the rest to save CPU cycles
        return spacy.load(model_name, disable=["lemmatizer", "attribute_ruler", "parser"])
    except OSError:
        print(f"Model {model_name} not found. Downloading...")
        from spacy.cli import download
        download(model_name)

        return spacy.load(model_name)
    
ner_model = load_nlp_model()