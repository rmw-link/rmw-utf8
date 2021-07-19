#!/usr/bin/env python3

from os import makedirs
from os.path import dirname, join
from tqdm import tqdm
import re
from fire import Fire
import bz2
from gensim.corpora.wikicorpus import extract_pages, filter_wiki
from config import ROOT

RE_S = re.compile(r':*{\|[\s\S]*?\|}')
RE_GALLERY = re.compile(
    r'<gallery>[\s\S]*?</gallery>'
    )
RE_BRACE = re.compile(
    r'(.){{([^{}\n]*?\|[^{}\n]*?)}}'
    )


def wiki_replace(d):
  s = d[1]
  s = RE_S.sub('', s)
  s = RE_GALLERY.sub('', s)
  s = RE_BRACE.sub('\\1[[\\2]]', s)
  s = filter_wiki(s)
  s = re.sub(r'\* *\n|\'{2,}', '', s)
  s = re.sub('\n+', '\n', s)
  s = re.sub('\n[:;]|\n +', '\n', s)
  li = []
  for i in s.split("\n"):
    li.append(i.strip("\r= *"))
  return d[0].strip(), "\n".join(li)


RE_EN = re.compile('^[a-zA-Z]+:')


def main(xml_bz2_filepath=None):
  if xml_bz2_filepath is None:
    print(main.__doc__)
    return
  outpath = join(ROOT, "txt/wiki")
  makedirs(outpath, exist_ok=True)

  wiki = extract_pages(bz2.open(xml_bz2_filepath))
  w = tqdm(wiki, desc='已获取0篇文章')
  size = 0
  i = 0
  f = open(f"{outpath}/{i}.txt", 'w')

  for d in w:
    if not RE_EN.findall(
        d[0]
        ) and d[0] and not d[1].startswith("#"):
      title, txt = wiki_replace(d)
      out = title + "\n" + txt + "\n\n"
      size += len(out)
      i+=1
      f.write(out)
      if size > 1024*1024*32:
        size = 0
        w.set_description('已获取%s篇文章' % i)
        f.close()
        f = open(f"{outpath}/{i//200}.txt", 'w')
  f.close()


Fire(main)
