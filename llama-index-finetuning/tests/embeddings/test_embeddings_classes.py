from llama_index.finetuning.types import BaseEmbeddingFinetuneEngine
from llama_index.finetuning.embeddings import (
    SentenceTransformersFinetuneEngine,
    EmbeddingAdapterFinetuneEngine,
)


def test_class():
    names_of_base_classes = [b.__name__ for b in EmbeddingAdapterFinetuneEngine.__mro__]
    assert BaseEmbeddingFinetuneEngine.__name__ in names_of_base_classes

    names_of_base_classes = [
        b.__name__ for b in SentenceTransformersFinetuneEngine.__mro__
    ]
    assert BaseEmbeddingFinetuneEngine.__name__ in names_of_base_classes
