import sys
import spacy


# slovak file
file_target = sys.argv[1]
with open(file_target, "r+") as ft:
    target_text = ft.read()

    nlp_sk = spacy.load('xx_sent_ud_sm')

    custom_doc_sk = nlp_sk(target_text)
    custom_doc_sk_sent = list(custom_doc_sk.sents)
    ft.seek(0)
    for sent in custom_doc_sk_sent:
        ft.write("%s\n" % sent)
    ft.truncate()
    

# english file
file_source = sys.argv[2]
with open(file_source, "r+") as fs:
    source_text = fs.read()

    nlp_en = spacy.load('en_core_web_sm')

    custom_doc_en = nlp_en(source_text)
    custom_doc_en_sent = list(custom_doc_en.sents)

    fs.seek(0)
    for sent in custom_doc_en_sent:
        fs.write("%s\n" % sent)
    fs.truncate()
