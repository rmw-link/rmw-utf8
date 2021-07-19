#!/usr/bin/env python3

from tangled_up_in_unicode.u13_0_0_data.unicode_data_to_name_start import unicode_data_to_name_start

with open('./train/src/utf8.char',"wb") as f:
  for i in unicode_data_to_name_start.keys():
    txt = chr(i)
    f.write(txt.encode('utf-8'))
  print(len(unicode_data_to_name_start))
