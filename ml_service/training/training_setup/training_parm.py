
from transformers import TrainingArguments


def load_training_args():
    lr = 2e-4
    batch_size = 8
    num_epochs = 2

    try:
        training_args = TrainingArguments(
            output_dir="output",
            learning_rate=2e-5,
            per_device_train_batch_size=8,
            per_device_eval_batch_size=8,
            num_train_epochs=3,
            evaluation_strategy="epoch",
            save_strategy="epoch",
            logging_dir="./logs",
            logging_steps=50,
            load_best_model_at_end=True,
            metric_for_best_model="f1",
            save_total_limit=2,
            fp16=True,  # if GPU
        )
        return (lr, batch_size, num_epochs, training_args)

    except Exception as e:
        raise ValueError(f"Error while loading training arguments: {str(e)}")
    