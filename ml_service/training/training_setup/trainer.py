
from transformers import Trainer


def trainer(model, training_args, tokenized_data, data_collator, compute_metrics) -> Trainer:
    try:
        print("Implementing the trainer...")
        trainer = Trainer(
            model=model,
            args=training_args,
            train_dataset=tokenized_data["train"],
            eval_dataset=tokenized_data["test"],
            data_collator=data_collator,
            compute_metrics=compute_metrics,
        )
        return trainer
    except Exception as e:
        raise RuntimeError(f"Error while implementing the trainer: {str(e)}")
