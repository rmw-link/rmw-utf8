#!/usr/bin/env python3

# 歌词 https://github.com/liuhuanyong/MusicLyricChatbot/tree/master/data

from tqdm import tqdm
from json import loads
from fire import Fire
from os import makedirs
from os.path import join, basename
from config import ROOT

@Fire
def main(filepath):
  outdir = join(ROOT,"txt/MusicLyricChatbot")
  print(outdir)
  makedirs(outdir,exist_ok=True)
  f = open(join(outdir,"0.txt"),"w")
  with open(filepath) as infile:
    for pos, line in enumerate(tqdm(tuple(infile)),1):
      line = line.rstrip()
      if not line:
        continue
      line = loads(line)
      f.write(line['song']+"\n")
      f.write("\n".join(line['geci'])+"\n")
      if pos % 1000 == 0:
        f.close()
        f = open(join(outdir,f"{pos//1000}.txt"),"w")
    f.close()

