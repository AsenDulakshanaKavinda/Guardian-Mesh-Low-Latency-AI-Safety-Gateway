from huggingface_hub import upload_folder

def push_to_huggingface_hub(repo_id: str, folder_path):
    try:
        print("uploading fine-tuned model to HF")
        upload_folder(repo_id=repo_id, folder_path=folder_path)
    except Exception as e:
        raise RuntimeError(f"Error while uploading ")