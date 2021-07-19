#!/usr/bin/env coffee

import sleep from 'await-sleep'
import html2txt from './html2txt'
import he from 'he'
import readline from 'readline'
import {DIR_TXT} from './config'
import path from 'path'
import fs from 'fs/promises'
import {createReadStream,existsSync} from 'fs'

walk = (dir) ->
  for await d from await fs.opendir(dir)
    entry = path.join(dir, d.name)
    if d.isDirectory()
      `yield* walk(entry)`
    else if d.isFile()
      yield entry


tidy = (txt)=>
  for i from [
    '我刚'

  ]
    if txt.startsWith i
      return ''

  for i from [
    '转发//'
    '回复:'
  ]
    txt = txt.replaceAll i,''
  txt

main = =>
  for await json from walk path.join(DIR_TXT,"xueqiu")
    if not json.endsWith '.json'
      continue

    txt_fp = json[..-6]+'.txt'
    if existsSync(txt_fp)
      {mtimeMs} = await fs.stat(json)
      if (await fs.stat(txt_fp)).mtimeMs > mtimeMs
        continue

    console.log txt_fp

    exist = new Set()

    todo = []
    await new Promise (resolve)=>
      readline.createInterface({
        input: createReadStream(json)
      }).on(
        'close'
        =>
          await fs.writeFile txt_fp, todo.join('\n')
          resolve()
      ).on 'line', (line)=>
        [title,txt] = JSON.parse line
        title = he.decode tidy(title)
        txt = html2txt(txt or '').replaceAll('//','\n')

        li = []
        if title and (txt.indexOf(title)+1)
          if not exist.has title
            li.push title
            exist.add title

        for i in txt.split("\n")
          if ["投资者参考","作者个人观点，仅供", "转发","回复"].indexOf(i)+1
            continue
          i = tidy(i)
          if i
            if not exist.has i
              exist.add i
              li.push(i)

        if li.length
          todo.push li.join('\n')

do =>
  n = 0
  while ++n<999999
    await main()
    console.log "#{n} round done"
    await sleep 60000*10
