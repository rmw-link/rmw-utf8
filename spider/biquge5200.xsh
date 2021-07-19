#!/usr/bin/env xonsh

from os.path import dirname,abspath,join,exists
import requests
from tqdm import tqdm

def book_li(url,exist):
  html = requests.get(url,headers={
    "user-agent": "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/90.0.4430.93 Safari/537.36"
    }).content.decode('gb18030','ignore')

  book_li = []
  for i in html.split('<a href="http://'):
    if i.startswith('www.b520.org/'):
      i = i.split('<')[0]
      url, book = i.split('">',2)
      if not book:
        continue
      url = url[13:-1]
      if url and url.replace('_','').isdigit():
        if url not in exist:
          exist.add(url)
          book_li.append((url, book))
  return book_li


PWD=dirname(abspath(__file__))
ROOT = join(dirname(PWD),"txt/biquge5200.cc")

def main():
  cd @(ROOT)
  exist = set()
  parse = join(PWD,"biquge5200.py")
  for index_url in [
    "http://www.b520.org/chuanyuexiaoshuo/",
    "http://www.b520.org/dushixiaoshuo/",
    "http://www.b520.org/kehuanxiaoshuo/",
    "http://www.b520.org/wangyouxiaoshuo/",
    "http://www.b520.org/xiaoshuodaquan/",
    "http://www.b520.org/xiuzhenxiaoshuo/",
    "http://www.b520.org/xuanhuanxiaoshuo/",
  ]:
    for url,name in tqdm(book_li(index_url,exist)):
      outname = name+".txt"
      print(outname, exists(join(ROOT, outname)))
      if exists(join(ROOT, outname)):
        continue
      rm -rf *.FictionDown
      url = f"https://www.biquge5200.cc/{url}/"
      print(name, url)
      FictionDown --url @(url) d
      FictionDown -i ./*.FictionDown conv -f txt
      @(parse) @('"'+outname+'"')


main()
