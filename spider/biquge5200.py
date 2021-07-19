#!/usr/bin/env python3

from fire import Fire

@Fire
def main(filepath):
  li = list(open(filepath))
  with open(filepath,"w") as out:
    while li:
      if li[0].find("：") > 0:
        li.pop(0)
      else:
        break

    for i in li:
      i = i.strip("\t\n ")
      if i.endswith(" 正文") or "笔趣阁" in i or (i.startswith('第') and ('章' in i[:7])):
        continue
      out.write(i+"\n")



