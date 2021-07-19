import he from 'he'
REPLACE = [
   '<p>','',
   '</p>','\n',
   '<br/>','\n',
   '<b>','',
   '</b>','',
   '\r\n','\n'
]

export default (html)=>
  len = REPLACE.length - 2
  txt = html
  loop
    txt = txt.replaceAll(REPLACE[len],REPLACE[len+1])
    if len
      len -= 2
    else
      break
  txt = he.decode txt.replace(/<.*>/g,'')
  li = []
  for i in txt.split('\n')
    i = i.trim()
    if i
      li.push i
  li.join '\n'


