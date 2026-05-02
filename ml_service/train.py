from training.train.training_script import run_train_stript

id2label={
    0 : "benign",
    1 : "malicious"
}

label2id={
    "benign": 0,
    "malicious": 1
}


if __name__ == "__main__":
    run_train_stript(
        model_name="google-bert/bert-base-uncased",
        model_save_path="",
        dataset_name="neuralchemy/Prompt-injection-dataset",
        num_labels=2,
        id2label=id2label
        label2id=label2id
        repo_id="AsenDulakshana/test_gradian_mesh"
    )