#!/usr/bin/env python3

# 全网新闻数据(SogouCA)版本：2012 https://www.sogou.com/labs/resource/ca.php

from tqdm import tqdm
from fire import Fire
from os import makedirs
from os.path import join, basename
from config import ROOT
from glob import glob
from tzutil.extract import extract_li

@Fire
def main(filepath):
  outdir = join(ROOT,"txt/SogouCA-2012")
  print(outdir)
  makedirs(outdir,exist_ok=True)
  li = glob(filepath+"/*.txt")
  for i in tqdm(li):
    with open(join(outdir,  basename(i)), "w") as out:
      with open(i,"rb") as f:
        html = f.read().decode('gb18030','ignore')
        for title ,txt in zip(
            extract_li("<contenttitle>","</contenttitle>",html),
            extract_li("<content>","</content>",html)):
          li = []
          for line in txt.replace("","\n").split("\n"):
            if line.startswith('来源：'):
              continue
            li.append(line)
          txt = "\n".join(li)+"\n"
          out.write(title+txt)

