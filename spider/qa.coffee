#!/usr/bin/env coffee

# https://github.com/brightmart/nlp_chinese_corpus
# 4.社区问答json版(webtext2019zh) ：大规模高质量数据集

import {DIR_TXT} from './config.coffee'
import readline from 'readline'
import {createReadStream, writeFileSync} from 'fs'
import html2txt from './html2txt'

do =>
  exist = new Set()

  txt = []
  n = 0
  await new Promise (resolve)=>
    readline.createInterface({
      input: createReadStream("/Users/z/Downloads/webtext2019zh/web_text_zh_train.json")
    }).on(
      'close'
      =>
        writeFileSync "#{DIR_TXT}/qa/#{n}.txt", txt.join("\n")
        resolve()
    ).on 'line', (line)=>
      {title, qid, desc, content} = JSON.parse(line)
      if exist.has(qid)
        desc = ""
      else
        exist.add qid
        if desc == "镜像问题" or desc.startsWith("<")
          desc = ""
      txt.push "#{html2txt title}\n#{html2txt desc}\n#{html2txt content}\n"
      if txt.length > 20000
        writeFileSync "#{DIR_TXT}/qa/#{n}.txt", txt.join("\n")
        n += 1
        console.log n
        txt = []
