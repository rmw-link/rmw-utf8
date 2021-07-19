import { readFileSync } from 'fs';
import { pathToFileURL, fileURLToPath } from 'url';
import 'source-map-support/register'

const baseURL = pathToFileURL(process.cwd() + '/').href;

export default (ignore, resolve, transformSource)=>{
  const IGNORE = new Set('yaml mjs txt'.split(' '));
  for(var i of ignore)IGNORE.add(i);
  return {
  resolve:function (url, context, defaultResolve) {
    const {
      parentURL = baseURL
    } = context;
    
    if (url.startsWith("~/")) {
      var n = parentURL.slice(baseURL.length+4, parentURL.lastIndexOf("/")).split("/").length;
      var li = [];
      while(n--){
        li.push('..');
      }
      li.push(url.slice(2));
      url = li.join("/");
    }

    var ext=url.slice(url.lastIndexOf('.')+1);
    
    if (ext!="js"  && !IGNORE.has(ext) && url.startsWith(".")){
      url = url+parentURL.slice(parentURL.lastIndexOf("."))
    }
    
    var r = resolve(url, parentURL);
    if(r)return r; 

    return defaultResolve(url, context, defaultResolve);
  },
  getFormat:function (url, context, defaultGetFormat) {
    var ext=url.slice(url.lastIndexOf('.')+1);
    if (IGNORE.has(ext)) {
      return {
        format: 'module'
      };
    }

    return defaultGetFormat(url, context, defaultGetFormat);
  },
  transformSource: function (source, context, defaultTransformSource) {
    const {
      url,
      format
    } = context;

    if (url.endsWith('.txt')){
      var s = JSON.stringify(readFileSync(fileURLToPath(url),"utf8").trimEnd());
      return {
        source:"export default "+ s
      }
    }
    var r=transformSource(url, source);
    if(r)return r;

    return defaultTransformSource(source, context, defaultTransformSource);
  }
  }
}
