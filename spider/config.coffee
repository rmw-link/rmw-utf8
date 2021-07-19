import {dirname,join} from 'path'
import {thisdir} from '@rmw/thisfile'

export DIR_ROOT = dirname thisdir(`import.meta`)
export DIR_TXT = join DIR_ROOT, "txt"
