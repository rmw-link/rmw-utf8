import CoffeeScript from 'coffeescript';
import Loader from './loader.mjs'

const COFFEE = "coffee", DOT_COFFEE = "."+COFFEE;

export const {resolve,getFormat,transformSource} = Loader(
  [
    COFFEE
  ],
  (url, parentURL)=>{
    if (url.endsWith(DOT_COFFEE)) {
      return {
        url: new URL(url, parentURL).href
      };
    }
    return;
  },
  (url, source)=>{
    if (url.endsWith(DOT_COFFEE)) {
      return {
        source: CoffeeScript.compile(source.toString('utf8'), {
          inlineMap:true,
          bare: true,
          filename: url
        })
      };
    }
  }
)
