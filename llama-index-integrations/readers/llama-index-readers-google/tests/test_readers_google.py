from llama_index.core.readers.base import BasePydanticReader
from llama_index.readers.google import GoogleDocsReader, GoogleSheetsReader


def test_class():
    names_of_base_classes = [b.__name__ for b in GoogleDocsReader.__mro__]
    assert BasePydanticReader.__name__ in names_of_base_classes

    names_of_base_classes = [b.__name__ for b in GoogleSheetsReader.__mro__]
    assert BasePydanticReader.__name__ in names_of_base_classes
