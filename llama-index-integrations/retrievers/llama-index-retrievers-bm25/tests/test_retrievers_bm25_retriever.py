from llama_index.core.base_retriever import BaseRetriever
from llama_index.retrievers.bm25.base import BM25Retriever


def test_class():
    names_of_base_classes = [b.__name__ for b in BM25Retriever.__mro__]
    assert BaseRetriever.__name__ in names_of_base_classes