#!/usr/bin/env coffee

import FormData from 'form-data'
import Axios from '@rmw/axios'
import sleep from 'await-sleep'
import html2txt from './html2txt'
import {mkdirSync, existsSync} from 'fs'
import fs from 'fs/promises'
import {join} from 'path'
import chalk from 'chalk'
import {DIR_TXT} from './config'

request = Axios.Axios::request

DIR = join(DIR_TXT,"jianshu")

axios = Axios.create({
  headers:
    "x-pjax": "true"
    "User-Agent": "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/90.0.4430.93 Safari/537.35"
    "x-requested-with": "XMLHttpRequest"
})

extract_li = (html, begin, end)->
  len = begin.length
  end_len = 1+end.length
  p = 0

  loop
    p = html.indexOf begin, p
    if p < 0
      break
    p += len
    e = html.indexOf end,p
    if e < 0
      break
    yield html[p...e]
    p = end_len + e

extract = (html, begin, end)->
  p = html.indexOf begin
  if p < 0
    return
  p+=(1+begin.length)
  e = html.indexOf end,p
  if e < 0
    return
  html[p...e]


fetch_txt = (id)=>
  url = "https://www.jianshu.com/p/#{id}"

  {data} = await axios.get(url)

  h1 = extract data,'<h1 ','</h1>'
  h1 = h1[h1.indexOf('>')+1..]
  h1 = html2txt h1

  console.log chalk.blue(url), h1

  p = data.indexOf('<article')+8
  p = data.indexOf('>',p)

  [
    h1
    html2txt data[p+1...data.indexOf('</article>',p)]
    extract_li data,' href="/p/','"'
  ]

###
fetch_page = (page)->
  data = new FormData()
  data.append('page', page)
  {data} = await axios.post("https://www.jianshu.com/trending_notes", data)
  data = data[6...-1].split('}{"id":')
  for i in data
    i = '{"id":'+i+"}"
    yield JSON.parse(i)
###

slug_path = (slug)=>
  dir = join DIR, slug[0..1], slug[2..3]
  fp = join dir, slug+".txt"
  if existsSync fp
    return
  mkdirSync(dir, { recursive: true })
  fp

do =>
  page = 1

  exist = new Set()
  todo = new Set([
    parseInt("714c161c1f1c",16)
  ])

  while todo.size
    for slug from todo
      todo.delete slug
      exist.add slug

      {size} = todo
      if size % 100 == 0
        console.log "to fetch", size

      slug = slug.toString(16).padStart(12,"0")
      fp = slug_path slug
      if not fp
        continue

      try
        r = await fetch_txt slug
      catch err
        console.trace(err)
        continue

      [title, txt, li] = r
      await fs.writeFile fp, title + "\n" + txt

      for i from li
        i = parseInt(i,16)
        if not exist.has i
          todo.add i
