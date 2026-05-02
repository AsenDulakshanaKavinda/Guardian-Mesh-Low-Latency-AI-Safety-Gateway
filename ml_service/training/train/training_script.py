from training.training_setup.dataset_setup import TrainingDataset
from training.training_setup.model_setup import TrainingModel
from training.training_setup.training_parm import load_training_args
from training.training_setup.trainer import trainer
from training.training_setup.push_to_hf import push_to_huggingface_hub

def run_train_stript(
        model_name: str, 
        model_save_path: str, 
        dataset_name: str, 
        num_labels: int, 
        id2label: dict, 
        label2id: dict,
        repo_id: str,

):
    # 1. model
    training_model = TrainingModel(
        model_name=model_name,
        model_save_path=model_save_path,
        num_labels=num_labels,
        id2label=id2label,
        label2id=label2id,
    )
    training_model.load_base_model()
    training_model.set_trainable_parameters([])
    trainable_model = training_model.get_trainable_model()
    tokenizer = training_model.get_tokenizer()

    # 2. dataset
    training_dataset = TrainingDataset(dataset_name=dataset_name)
    training_dataset.load_raw_dataset()
    training_dataset.preprocess(tokenizer=tokenizer)
    tokenized_dataset = training_dataset.get_dataset()
    data_collector = training_dataset.get_data_collector(tokenizer=tokenizer)

    # 3. train args
    lr, batch_size, num_epochs, training_args = load_training_args()

    # 4. trainer
    model_trainer = trainer(model=trainable_model, training_args=training_args, tokenized_data=tokenized_dataset, data_collator=data_collector)

    # 5. training
    model_trainer.train()

    # 6. save model
    trainable_model.save_pretrained(model_save_path)
    tokenizer.save_pretrained(model_save_path)

    # 7. push to HF
    push_to_huggingface_hub(repo_id=repo_id, folder_path=model_save_path)


