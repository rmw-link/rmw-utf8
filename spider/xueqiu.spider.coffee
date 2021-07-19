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
  baseURL: "https://xueqiu.com/"
  headers:
    'User-Agent': 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/90.0.4430.93 Safari/537.36'
    Cookie: 'device_id=24700f9f1986800ab4fcc880530dd0ed; s=dg1jgs0yqb; acw_tc=2760820916252511337467025e89488c277d29aa152bde50b6fb63ff4bdae7; remember=1; xq_a_token=2a57a766a2acd5154bc4072726eab978d75caf79; xqat=2a57a766a2acd5154bc4072726eab978d75caf79; xq_id_token=eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJ1aWQiOjI4OTcxMjE4MjAsImlzcyI6InVjIiwiZXhwIjoxNjI3ODQzNDIyLCJjdG0iOjE2MjUyNTE0MjIxMTcsImNpZCI6ImQ5ZDBuNEFadXAifQ.UtH1opK5VvfcvjsDB9FAqLAMKIABYt6gFRBpAxzHKKilPMJibm4UqnJkIwQyOu5wJomKI9fP2uvOg9SP7TqPRJZGRcw3s7dxiBwDhqiG57Dpt_qbk8qU6g0rVmIlgTzeZ0OkDlKVt1gbUHFX5FcYi2agt4dTaroXvRYF_eW2NfmgNsXKhO2qMkEU88SLW000D2bZWNUWxetKb6ayq6R2E1gaVbiC68kOoH8w9IRvamBW8RpvOVcLeos3WmfsqU5eeC0VXiTf96HUAftw7i71K1WlkfqSD43uWMarorzvtZxgbA5qYr6vSnP8CDTgXHUaN1JvPeawkZEeLDYlHZTNbg; xq_r_token=653b628bed7ee8993797571c0d06ae3c3c8fdc4a; xq_is_login=1; u=2897121820; is_overseas=0'
})


fetch_hot = (max_id)->
  url = "statuses/hot/listV2.json?since_id=-1&size=15"
  if max_id
    url += "&max_id=#{max_id}"
  {data} = await axios.get url
  return data


_fetch_user = (user_id, page)=>
  n = 0
  while ++n < 5
    url = "v4/statuses/user_timeline.json?page=#{page}&user_id=#{user_id}"
    {data} = await axios.get url
    if data?.statuses
      return data
    else
      console.log data
  data


dump = (statuses, out)=>
  if not statuses
    return
  for {id,type,title,text,created_at, reply_count, retweet_count,retweeted_status} in statuses
    li = [
      title, text, id, (type-0) or 0, parseInt(created_at/1000).toString(36), reply_count, retweet_count
    ]
    if retweeted_status
      {user_id,id} = retweeted_status
      li = li.concat [user_id,id]
    await out.write JSON.stringify(li)+"\n"


fetch_user = (user)=>
  user_id = user.id+""

  dir = join(DIR_TXT, "xueqiu",user_id[-2..],user_id[-4..-3])
  await fs.mkdir(dir,recursive:true)

  fp = join(dir,"#{user_id}.json")
  if existsSync(fp)
    return

  console.log user.screen_name, user.description

  out = await fs.open(fp,"w")
  out.write JSON.stringify([user.screen_name,user.description])+"\n"

  {page, maxPage, statuses} = await _fetch_user(user_id, 1)
  if statuses
    if maxPage > 1
      now = Math.min(1000,maxPage)
      while now > 1
        {statuses} = await _fetch_user(user_id, now)
        await dump(statuses, out)
        now -= 1
    await dump(statuses, out)
  await out.close()


do =>
  next_max_id = 0
  loop
    data = await fetch_hot(next_max_id)
    {next_max_id,items} = data
    if not next_max_id
      break
    for {original_status} in items
      {user,user_id,id, type, description,title} = original_status
      await fetch_user(user)


