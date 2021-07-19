#!/usr/bin/env coffee

import Axios from '@rmw/axios'
import sleep from 'await-sleep'
import {existsSync} from 'fs'
import fs from 'fs/promises'
import {join} from 'path'
import chalk from 'chalk'
import {DIR_TXT} from './config'
request = Axios.Axios::request

Axios.Axios::request = (config)->
  console.log chalk.gray(config.method),chalk.blue(config.url)
  await sleep(600)
  request.apply(@,arguments)


axios = Axios.create({
  baseURL: "https://www.v2ex.com/"
  headers:
    'User-Agent': 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/90.0.4430.93 Safari/537.36'
    "accept": "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9",
    "accept-language": "zh-CN,zh;q=0.9,zh-TW;q=0.8,zh-HK;q=0.7,en;q=0.6",
    "cache-control": "max-age=0",
    "sec-ch-ua": "\" Not A;Brand\";v=\"99\", \"Chromium\";v=\"90\", \"Google Chrome\";v=\"90\"",
    "sec-ch-ua-mobile": "?0",
    "sec-fetch-dest": "document",
    "sec-fetch-mode": "navigate",
    "sec-fetch-site": "same-origin",
    "sec-fetch-user": "?1",
    "upgrade-insecure-requests": "1",
    "cookie": "A2=\"2|1:0|10:1619512653|2:A2|48:ODIxY2FkYzYtODU0YS00N2M0LWFhNzYtYmJiOWZiN2M1NDY4|60de3fc0a0e4d25ec61755c5aa1725ca916310213f761624e35ff0fb4f2e0543\"; V2EX_LANG=zhcn; PB3_SESSION=\"2|1:0|10:1625626915|11:PB3_SESSION|36:djJleDo1NC4xNzcuMTI3LjM3OjM1NzYzOTM3|d8cb2e54057f36b0469f4888c1f22dce5b19c12b496a6f71f89c081796778a67\"; V2EX_TAB=\"2|1:0|10:1625626915|8:V2EX_TAB|4:YWxs|ab8b3dfda1c4fbaf103897c255e79c9933f4b316de71beaed76cb280ef9cebec\""
})

fetch_index = (page)=>
  {data} = await axios.get("recent?p=#{page}")
  console.log data

do =>
  page = 1
  loop
    await fetch_index(page)
