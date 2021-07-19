#!/usr/bin/env python3

import twint
import html
import traceback
from config import ROOT
from os.path import join, exists
from os import makedirs
import re

HTTP = re.compile(r'\s*https?:\/\/[^\s]+', flags=re.MULTILINE)
TWITTER_USERNAME_RE = re.compile(r'@([A-Za-z0-9_]+)')

TXT = join(ROOT,"txt/tweet")
makedirs(TXT,exist_ok=True)

EXIST = set()

def fetch_user(username, limit):
  EXIST.add(username)
  c = twint.Config()

  c.Username = username
  c.Store_object = True
  c.Limit = limit
  twint.run.Search(c)
  tweets = twint.output.tweets_list

  user_li = []
  li = []
  for i in tweets:
    if i.type == "tweet":
      tweet = html.unescape(i.tweet)

      for user in TWITTER_USERNAME_RE.findall(tweet):
        if user not in EXIST:
          user_li.append(user)

      tweet = TWITTER_USERNAME_RE.sub("", tweet)
      tweet = HTTP.sub("\nhttps://\n", tweet)
      for i in tweet.split("\n"):
        i = i.strip()
        if i:
          li.append(i)

  return user_li, "\n".join(li)

from fire import Fire

@Fire
def main(first="sagacity"):
  tofetch = [first]

  while 1:

    if not tofetch:
      return

    username = tofetch.pop(0)

    try:
      EXIST.add(username)
      out = join(TXT,username+".txt")
      if exists(out):
        continue
      user_li, txt = fetch_user(username,20)
      if txt.find("的") < 0:
        continue
      user_li, txt = fetch_user(username,30000)
      with open(out,"w") as f:
        f.write(txt)
      for i in user_li:
        print(i)
        if len(tofetch) < 1024:
          tofetch.append(i)
    except Exception:
      traceback.print_exc()

