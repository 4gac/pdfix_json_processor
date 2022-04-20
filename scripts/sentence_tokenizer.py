import sys
from nltk.tokenize import sent_tokenize
import nltk.data

tokenizer = nltk.data.load("tokenizers/punkt/english.pickle")

file_source = sys.argv[1]
with open(file_source, "r+") as f:
    source_data = f.read()
    # tokens = sent_tokenize(source_data)
    tokens = tokenizer.tokenize(source_data)
    # pysbd sentence tokenizer
    #seg = pysbd.Segmenter(language="en", clean=False)
    #tokens = seg.segment(source_data)
    f.seek(0)
    f.write(
        "\n". join(tokens)
    )
    f.truncate()

tokenizer2 = nltk.data.load("tokenizers/punkt/polish.pickle")

file_target = sys.argv[2]
with open(file_target, "r+") as f:
    target_data = f.read()
    # tokens = sent_tokenize(target_data)
    tokens = tokenizer2.tokenize(target_data)
    # pysbd sentence tokenizer
    #  seg = pysbd.Segmenter(language="en", clean=False)
    # tokens = seg.segment(target_data)
    f.seek(0)
    f.write(
        "\n".join(tokens),
    )
    f.truncate()
