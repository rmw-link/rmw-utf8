<!-- 本文件由 ./readme.make.md 自动生成，请不要直接修改此文件 -->

# rmw-utf8

Short text compression algorithm for utf-8 (optimized for Chinese , developed based on rust programming language).

面向utf-8的短文本压缩算法（为中文优化，基于rust编程语言开发）。

注意：rmw-utf8 只能压缩utf-8文本，不是通用的二进制压缩算法

## 如何使用

```rust
use rmw_utf8::{decode, encode};

fn main() {
  let txt = "测试一下";
  let compressed = encode(&txt.as_bytes());
  let decompressed = decode(&compressed[..]);
  assert!(txt == decompressed);
}
```

## 压缩率评测

```
"人民网络"
        orginal text bytes length :  12
    gz(level 6) compressed length :  33 , compress ratio 275.00% , cost 0.02ms
  lzma(level 6) compressed length :  68 , compress ratio 566.67% , cost 1.15ms
  zstd(level 6) compressed length :  21 , compress ratio 175.00% , cost 1.97ms
 zstd(level 19) compressed length :  21 , compress ratio 175.00% , cost 56.91ms
       rmw-utf8 compressed length :   4 , compress ratio  33.33% , cost 0.02ms

"互联网犹如末期的封建王朝，寡头们完成了土地兼并，垄断了人民创造的数据，坐收地租。"
        orginal text bytes length : 120
    gz(level 6) compressed length : 134 , compress ratio 111.67% , cost 0.04ms
  lzma(level 6) compressed length : 180 , compress ratio 150.00% , cost 11.50ms
  zstd(level 6) compressed length : 129 , compress ratio 107.50% , cost 0.85ms
 zstd(level 19) compressed length : 129 , compress ratio 107.50% , cost 19.73ms
       rmw-utf8 compressed length :  47 , compress ratio  39.17% , cost 0.09ms

"我不喜欢这样的世界。我想寻求改变。"
        orginal text bytes length :  51
    gz(level 6) compressed length :  74 , compress ratio 145.10% , cost 0.02ms
  lzma(level 6) compressed length : 108 , compress ratio 211.76% , cost 6.37ms
  zstd(level 6) compressed length :  60 , compress ratio 117.65% , cost 1.11ms
 zstd(level 19) compressed length :  60 , compress ratio 117.65% , cost 27.02ms
       rmw-utf8 compressed length :  11 , compress ratio  21.57% , cost 0.03ms

"打到数据霸权 · 网络土地革命"
        orginal text bytes length :  40
    gz(level 6) compressed length :  63 , compress ratio 157.50% , cost 0.02ms
  lzma(level 6) compressed length :  96 , compress ratio 240.00% , cost 10.43ms
  zstd(level 6) compressed length :  49 , compress ratio 122.50% , cost 1.17ms
 zstd(level 19) compressed length :  49 , compress ratio 122.50% , cost 26.00ms
       rmw-utf8 compressed length :  17 , compress ratio  42.50% , cost 0.15ms

"翻看2021年整个消费品牌赛道，“卖得贵”几乎成为了核心主线"
        orginal text bytes length :  82
    gz(level 6) compressed length : 105 , compress ratio 128.05% , cost 0.07ms
  lzma(level 6) compressed length : 140 , compress ratio 170.73% , cost 10.49ms
  zstd(level 6) compressed length :  91 , compress ratio 110.98% , cost 0.39ms
 zstd(level 19) compressed length :  91 , compress ratio 110.98% , cost 23.24ms
       rmw-utf8 compressed length :  27 , compress ratio  32.93% , cost 0.17ms

"拨开重庆魔幻的面纱，是网红第一城的逆袭史"
        orginal text bytes length :  60
    gz(level 6) compressed length :  83 , compress ratio 138.33% , cost 0.05ms
  lzma(level 6) compressed length : 116 , compress ratio 193.33% , cost 9.45ms
  zstd(level 6) compressed length :  69 , compress ratio 115.00% , cost 0.24ms
 zstd(level 19) compressed length :  69 , compress ratio 115.00% , cost 24.66ms
       rmw-utf8 compressed length :  26 , compress ratio  43.33% , cost 0.17ms

"从零设计可视化大屏搭建引擎"
        orginal text bytes length :  39
    gz(level 6) compressed length :  62 , compress ratio 158.97% , cost 0.04ms
  lzma(level 6) compressed length :  96 , compress ratio 246.15% , cost 8.64ms
  zstd(level 6) compressed length :  48 , compress ratio 123.08% , cost 1.77ms
 zstd(level 19) compressed length :  48 , compress ratio 123.08% , cost 25.32ms
       rmw-utf8 compressed length :  15 , compress ratio  38.46% , cost 0.11ms

"Harbor学习笔记：快速搭建Docker私有仓库"
        orginal text bytes length :  51
    gz(level 6) compressed length :  74 , compress ratio 145.10% , cost 0.05ms
  lzma(level 6) compressed length : 108 , compress ratio 211.76% , cost 7.63ms
  zstd(level 6) compressed length :  60 , compress ratio 117.65% , cost 0.25ms
 zstd(level 19) compressed length :  60 , compress ratio 117.65% , cost 25.95ms
       rmw-utf8 compressed length :  27 , compress ratio  52.94% , cost 0.13ms

"“师尊，何谓大道？”“大道……道可道，非常道。“请说人话？”“……我也不知道。”"
        orginal text bytes length : 120
    gz(level 6) compressed length : 116 , compress ratio  96.67% , cost 0.03ms
  lzma(level 6) compressed length : 152 , compress ratio 126.67% , cost 7.11ms
  zstd(level 6) compressed length : 118 , compress ratio  98.33% , cost 0.27ms
 zstd(level 19) compressed length : 113 , compress ratio  94.17% , cost 27.76ms
       rmw-utf8 compressed length :  39 , compress ratio  32.50% , cost 0.20ms

"一场灭世浩劫从天而降，没人能阻挡来自深空的威胁，地球上的所有生物，在瞬间烟消云散。"
        orginal text bytes length : 123
    gz(level 6) compressed length : 137 , compress ratio 111.38% , cost 0.04ms
  lzma(level 6) compressed length : 184 , compress ratio 149.59% , cost 6.83ms
  zstd(level 6) compressed length : 132 , compress ratio 107.32% , cost 0.48ms
 zstd(level 19) compressed length : 132 , compress ratio 107.32% , cost 30.80ms
       rmw-utf8 compressed length :  37 , compress ratio  30.08% , cost 0.08ms

"免责声明"
        orginal text bytes length :  12
    gz(level 6) compressed length :  33 , compress ratio 275.00% , cost 0.01ms
  lzma(level 6) compressed length :  68 , compress ratio 566.67% , cost 5.93ms
  zstd(level 6) compressed length :  21 , compress ratio 175.00% , cost 0.32ms
 zstd(level 19) compressed length :  21 , compress ratio 175.00% , cost 27.07ms
       rmw-utf8 compressed length :   6 , compress ratio  50.00% , cost 0.03ms

"由此给你带来的不便，我们深表歉意。"
        orginal text bytes length :  51
    gz(level 6) compressed length :  74 , compress ratio 145.10% , cost 0.02ms
  lzma(level 6) compressed length : 108 , compress ratio 211.76% , cost 6.79ms
  zstd(level 6) compressed length :  60 , compress ratio 117.65% , cost 0.23ms
 zstd(level 19) compressed length :  60 , compress ratio 117.65% , cost 28.06ms
       rmw-utf8 compressed length :  18 , compress ratio  35.29% , cost 0.05ms

"财经作家吴晓波坦言：『自己是精英主义者，并认为大部分人都是无用的，因为世界不需要很多人同时思考，吴晓波频道只服务于少数的几十万人就够了。』"
        orginal text bytes length : 207
    gz(level 6) compressed length : 195 , compress ratio  94.20% , cost 0.02ms
  lzma(level 6) compressed length : 248 , compress ratio 119.81% , cost 6.49ms
  zstd(level 6) compressed length : 203 , compress ratio  98.07% , cost 0.25ms
 zstd(level 19) compressed length : 203 , compress ratio  98.07% , cost 29.58ms
       rmw-utf8 compressed length :  67 , compress ratio  32.37% , cost 0.13ms

"所谓现代主义审美，用简短一句话概括即是“少而精”（Less is Better）。我们不难从家居、数码设备、家用电器和流行服饰瞥见这种趣味，年轻设计师更是多将其奉为行业准则。"
        orginal text bytes length : 233
    gz(level 6) compressed length : 235 , compress ratio 100.86% , cost 0.03ms
  lzma(level 6) compressed length : 280 , compress ratio 120.17% , cost 3.98ms
  zstd(level 6) compressed length : 229 , compress ratio  98.28% , cost 0.25ms
 zstd(level 19) compressed length : 229 , compress ratio  98.28% , cost 30.24ms
       rmw-utf8 compressed length :  95 , compress ratio  40.77% , cost 0.17ms

"华为ADS自动驾驶系统为什么甩特斯拉几条街？\n仅算力方面，华为拥有400Tops,特斯拉仅有140Tops，几乎是三倍。\n除此以外，我们看看核心部分雷达的配置。\n与华为深度合作的北汽蓝谷ARCFOX极狐HI版搭载华为ADS，使用了3颗激光雷达、6颗毫米波雷达、12颗超声波雷达、13颗摄像头。\n特斯拉的自动驾驶配置，由1颗毫米波雷达、12个超声波雷达和8个摄像头组成。\n高低立判，这能是一个级别的吗？\n特斯拉为什么不用激光雷达？因为太贵，最早一个成本要几万美元，现在技术进步也要几千美元一颗。华为的持续研发投入水平，能够使得成本降到几百美元。"
        orginal text bytes length : 723
    gz(level 6) compressed length : 499 , compress ratio  69.02% , cost 0.04ms
  lzma(level 6) compressed length : 552 , compress ratio  76.35% , cost 3.96ms
  zstd(level 6) compressed length : 532 , compress ratio  73.58% , cost 0.28ms
 zstd(level 19) compressed length : 527 , compress ratio  72.89% , cost 30.62ms
       rmw-utf8 compressed length : 277 , compress ratio  38.31% , cost 0.28ms

"# 美国经济复苏加速\u{3000}美联储会议记录的缩表讨论备受关注\n\n据路透社7月6日报道，美联储官员在6月15至6月16日的会议上就缩减每月1,200亿美元的购债计划展开辩论，此后大多数美联储决策者对经济表达了普遍乐观的看法，即从许多指标来看，经济正在摆脱全球新冠肺炎（COVID-19）疫情引发的衰退。\n\n一些决策者暗示，他们认为最近的就业增长和通胀超标意味着朝着美联储就业最大化和稳定物价的目标取得“实质性进一步进展”，这种情况将允许他们开始缩减资产购买。\n\n自美联储上次会议以来的数据可能支持这一结论。\n\n美国劳工部7月2日公布备受关注的就业报告显示，美国上月增加了85万个工作岗位，在连续两个月增长疲软之后，就业增速好于预期。\n\n美联储偏爱的通胀指标以1992年以来最快的年增速上升，并远超2%的目标水准。\n\n7月7日的会议记录可能会提供更多细节，说明就在最新经济数据公布前，美联储官员认为美联储距离实现目标还有多远。会议记录还可能显示出，对于最近的高通胀属于暂时现象，还是较持久的情况，决策者的分歧到底有多大。\n\n“会议记录肯定会很有意思，”富国银行资深分析师Sam Bullard说。“市场将关注有关讨论何时开始缩减刺激的细节。”\n\n一些美联储官员在6月的会议上提前对加息时间的预测。18位决策者中有13位目前认为2023年会加息，其中7位认为最快明年就会加息。\n\n路透调查的分析师认为，美联储将在8月或9月宣布缩减资产购买的策略，并在2022年初开始削减购债。\n\n美联储官员同意将缩减非常规购债计划做为加息前的第一步。2020年12月，他们为缩表设立了一个门槛：经济必须在实现就业最大化和通胀率持续保持2%方面取得“实质性的进一步进展”。\n\n美联储最关注的通胀指标在5月份同比上升3.4%，但就业人数与疫情爆发前相比减少了680万人，劳动力仍较疫前水平低340万。\n\n一些决策者，如达拉斯联邦储备银行总裁柯普朗警告说，在需要提高利率前，岗位空缺可能不会被完全填补。自疫情开始后已有超过250万55岁以上的美国人退休。\n\n凯投宏观（Capital Economics）高级经济学家Andrew Hunter说：“6月非农就业强劲增长，将使呼吁提前结束资产购买的美联储官员更有底气。”\n\n会议记录还可能揭示，关于联储打算如何缩减每月800亿美元公债和400亿美元抵押贷款支持证券（MBS）购买规模的初步讨论情况。\n\n在2014年的上一次此类行动中，缩表实际上是以自动驾驶模式进行的——即每次政策会议后公债和MBS都同样被削减-——即使官员们称它不是预设路线。\n\n不过，最近几周，一些美联储官员似乎与那些认为美联储没有必要在火热的房地产市场继续购买住房支持资产的批评者有同感，但他们同时警告称美联储更倾向于以最可预测的方式缩减购债规模。\n\n但纽约联邦储备银行总裁威廉姆斯（John Williams）指出，将购债从MBS转向公债，对抵押贷款利率仅会产生很小的影响。\n\n前美联储经济学家、经济咨询公司MacroPolicy Perspectives总裁Julia Coronado说：“我认为我们听到的事实...反映出他们还没有进行这种讨论。”\n\n她认为，负责债券购买计划的工作人员在7月会议给出更实质性的报告，将打消以更快速度削减MBS的论点。“一旦纽约联储的工作人员和领导层充分阐述了所有问题，他们就不会说这种傻话了。”"
        orginal text bytes length : 3764
    gz(level 6) compressed length :2134 , compress ratio  56.70% , cost 0.12ms
  lzma(level 6) compressed length :2152 , compress ratio  57.17% , cost 5.27ms
  zstd(level 6) compressed length :2304 , compress ratio  61.21% , cost 0.38ms
 zstd(level 19) compressed length :2091 , compress ratio  55.55% , cost 30.44ms
       rmw-utf8 compressed length :1308 , compress ratio  34.75% , cost 1.13ms

"# 算法，即剥削\n\n如果数据不私有化或未建立对算法的有效管控，算法即剥削。\n\n近期，《人物》团队推出一篇调查性文章《外卖骑手，困在系统里》。这篇文章揭示了“与生命赛跑”的外卖骑手所面临的困境。他们的时间、收入及生命安全，被强大的算法锁定。为了“准时送达”，骑手们经常在钢铁洪流中超速、逆行、穿红灯……\n\n这篇文章一度让舆论非常同情骑手，同时批判平台冷酷的算法和资本家对生命的漠视。但是，平台公司将皮球踢给用户，这事最后无疾而终，骑手依然在“极限赛跑”。\n\n除了道德批判，我们似乎别无办法。舆论也没有深入挖掘“骑手被困系统里”的本质问题。本文从经济学的角度，使用反垄断理论、价格歧视及消费者剩余理论，解释和揭示“算法剥削”。\n\n本文逻辑：\n\n一、数据，即权力\n\n二、歧视，即垄断\n\n三、算法，即剥削\n\n数据，即权力\n这篇文章提到一个关键词：时间失踪。\n\n“2016年，3公里送餐距离的最长时限是1小时，2017年，变成了45分钟，2018年，又缩短了7分钟，定格在38分钟——据相关数据显示，2019年，中国全行业外卖订单单均配送时长比3年前减少了10分钟。【1】”\n\n从平台方来说，“吞掉时间”是算法带来的技术革命。外卖平台实时收集海量的配送数据，人工智能算法通过深度学习，优化派单，压缩时间，提升配送效率。这就是美团的“超脑”、饿了么的“方舟”的力量。\n\n对于平台公司来说，时间就是金钱。\n\n“根据美团公布的数据显示，2019年第三季度，美团外卖的订单量达到25亿，每单收入比2018年同时期增加了0.04元，而与此同时，每单成本则同比节省了0.12元——这也帮助美团在2019年Q3，多赚了整整4亿元。【1】”\n\n但是，对于骑手来说，“效率就是生命”。\n\n骑手们的收入被系统的算法支配着。骑手的收入取决于接单量、准时率、差评率、投诉率。其中，准时率是最重要的。因为差评和投诉主要原因是超时，如果超时，系统会自动扣提成，接单量再大也是徒劳。“准时率低于98%一单扣一毛钱，低于97%一单扣两毛钱。”\n\n过去几年，配送里程增加，配送时间却在减少。\n\n“美团研究院在今年6月发布的中国外卖产业发展报告中称，2019年骑手日均配送里程相比2018年增长约5.5%，日均配送里程大于50公里的骑手比例从2018年的13.8%增至2019年的18.2%【2】。”\n\n为了与时间赛跑，骑手不得不超速，甚至闯红灯、逆行。这导致骑手的交通事故率上升。“现实数据有力地佐证了这一判断——2017年上半年，上海市公安局交警总队数据显示，在上海，平均每2.5天就有1名外卖骑手伤亡。【1】”\n\n系统规划的时间是最短的，有时没有考虑路况、雨天、单行道、红灯等现实问题。这就迫使骑手拿生命派单。\n\n文章发出后，这一缺乏话语权的群体备受社会的关注。\n\n有人拿出约翰·罗尔斯的正义理论批评平台算法“不计偶然性”特别违反正义原则。系统算法计算的是最理想的极限时间，却忽略了现实诸多偶然因素，如电梯拥挤、雨天堵车、电动车故障等。【3】\n\n有人拿出尼克·西弗的“算法文化”，认为平台的算法，除了包括理性程序外，还要包含制度、交叉环境等，并建议研究者应该从人类学地探索算法【1】。\n\n有人拿出港交所的“ESG信息披露”，指出美团等上市公司需要披露包括环境、社会责任和公司治理的信息。在这里，骑手的生存环境属于社会责任的范畴。上市公司必须“不遵守就解释”“过去三年每年因工亡故的人士及比率”。期望这一披露制度倒逼平台重视骑手的交通事故风险，并给予更多的保护【2】。\n\n但是，平台却将皮球踢给了消费者，推出新功能，增加“愿意等待系统”。言下之意，顾客是上帝，不是我们要求骑手快，而是你们（消费者）要求骑手快。\n\n虽然评论区对这一踢皮球行为很气愤，但是除了骂资本家无良外，也没有别的好办法。随着热度退却，骑手每天与死神赛跑的状况并未改变。\n\n这到底是什么问题？\n\n算法优化配送，提升经济效率，这是技术进步，利好于消费者（尤其在疫情下），利好于平台。理论上来说还利好于骑手，节省了不必要的路程。但是，人们总觉得其中有问题，又说不出哪里出了问题，最后只能从情绪道德上谴责资本家剥削，从社会责任上呼吁资本家手下留情。\n\n其实，这不仅仅是道德问题，更是法律问题。\n\n从经济学的角度，算法支配骑手是一种垄断行为。这种技术性垄断，很可能构成平台滥用数据优势，以及价格歧视中的大数据杀熟。\n\n在反垄断法的框架下，滥用市场支配地位是三类垄断行为中的一类。在大数据时代，平台可能滥用大数据的支配优势。\n\n数据是一种资源，也是一种权力。\n\n数据本是用户的一项私人资源，数据所有权也就是一项私人权力。但是，现在大型平台没有采用分布式系统，私人数据被中心化的数据库垄断。因此，私人的数据所有权被剥夺，科技公司便产生了所谓的大数据支配优势。科技公司往往在不告知用户的前提下采集、占有并使用私人数据。\n\n数据为何成为科技公司一项“关键权力”？\n\n用户在平台上留下的任何结构性的和非结构性的数据，经过科技公司的数学模型分析后，变得具有预测性。隐秘在用户深处的欲望、需求、情绪、情感可能被算法洞悉，科技公司可借此推送信息，引导消费，改变甚至控制人们的思想及行为。\n\n2018年，Facebook陷入“数据泄露丑闻”。在听证会上，有议员质问扎克伯格：“Facebook在窃听用户说的话？”扎克伯格婉转地回答：“我们允许用户上传分享自己拍摄的视频，这些视频的确有声音，我们也的确会记录那些声音，并且利用对这些声音的分析来提供更好的服务。”\n\n滥用大数据支配优势的极端情况是大数据杀熟。\n\n所谓大数据杀熟，是一种差异化定价行为。比如，某电商平台上同样一件商品，老用户和新用户客户端上所显示的价格不同。又如，当你急于在某个网络平台上购买飞机票时，票价却莫名其妙地上涨了。\n\n亚马逊是大数据杀熟的“始作俑者”。2000 年，亚马逊针对同一张 DVD 碟片施行不同的价格政策，新用户看到的价格是 22.74 美元，如果是算法认定有购买意愿的老用户，价格会显示为 26.24 美元。如果删除 Cookie，价格马上又回落。很快这种策略被用户发现并投诉，亚马逊 CEO 贝索斯公开道歉，说这仅仅是一场实验，也承诺不再进行价格歧视。\n\n在美国，大数据杀熟为何被禁止？\n\n歧视，即垄断\n大数据杀熟其实是一种歧视性定价行为。而价格歧视是一种被打击的垄断行为。美国经济学家贝克尔在1955年的博士论文《歧视经济学》最早发现了歧视行为。他认为，歧视行为将付出代价，造成经济效率损失【4】。\n\n价格歧视是指厂商在同一时期对同一商品向不同客户索取不同的价格。厂商可借此获得差别定价的垄断利润。而消费者在这一行为中受到了针对性定价的歧视。\n\n当然，差别定价是一种商业竞争策略，并非所有的价格歧视都是违法的。\n\n美国1936年出台的《罗宾逊帕特曼法》是一部针对价格歧视的法律。这部法律规定，确定价格歧视违法需要满足两个条件：一是同一个商品针对不同消费者采用不同售价；二是这种行为对竞争构成破坏或给消费者造成损害。\n\n但是，现实中，确定价格歧视依然相当困难。\n\n英国经济学家庇古在1920年《福利经济学》按照价格歧视程度，分为一级价格歧视、二级价格歧视、三级价格歧视【5】。\n\n三级价格歧视，对不同群体的客户索取不同的价格，但群体内的价格是一致的。民航、电影院、跨国公司经常采取这类差异化定价策略。\n\n二级价格歧视，针对不同购买量索取不同的价格。数量折扣，多买更优惠，也是常用的定价策略。\n\n但是，一级价格歧视是不被允许的。一级价格歧视，也叫完全价格歧视，同一商品针对每一个不同的买家都采用不同的价格。大数据杀熟就是属于一级价格歧视。\n\n价格歧视，通常用在消费领域，这与骑手有何关系？\n\n这里需要引入两个概念：信息不对称和消费者剩余。\n\n所谓信息不对称，是指交易中的各人拥有的信息不同。在交易中，掌握充分信息的一方对信息贫乏一方构成议价优势。价格歧视程度，与厂商所掌握的信息量直接相关。在一级价格歧视中，厂商占据了绝对的信息优势，对每一个买家的信息了如指掌，从而可以做出差异化定价，最大限度地榨取消费者剩余。\n\n消费者剩余是庇古的老师马歇尔在1890年《经济学原理》中提出来的概念【6】。所谓消费者剩余，是指消费者在购买一定数量的某种商品时愿意支付的最高总价格和实际支付的总价格之间的差额。\n\n比如，这部手机标价3000元，你的心理价位是3500元，那么多出来的500元就是消费者剩余。与消费者剩余相对的概念是生产者剩余，两者可以称为“交易剩余”，属于一种“心理剩余”。\n\n有人认为，“交易剩余”不过是心理效应，不会造成财富损失。这是理解是错误的。交易双方只有交易剩余大于零时，才会选择交易。这就是我们通常说的“划得来”。交易剩余是真实财富，交易之所以发生以及交易带来的财富，正是“交易剩余”。\n\n而价格歧视所获取的超额利润，也正是“交易剩余”。正因为企业攫取了消费者的“交易剩余”，我们才认定其获得了超额垄断利润。在一级价格歧视中，企业掌握了每一个买家的信息，对每一个买家都索取了其愿意支付的最高价格，从而赚取了所有买家的全部消费者剩余。\n\n根据哈耶克的信息分散理论，企业不可能掌握每一个买家的所有完整信息。但是，在“大数据杀熟”中，平台公司的算法越出色，对每一个买家的信息分析越充分，就可能榨取更多的消费者剩余。换言之，算法极限追求消费者剩余。\n\n通常，价格歧视、大数据杀熟发生在消费领域，这与骑手被困在系统里有何关系？\n\n骑手与消费者一样，其私人数据都被平台系统掌控，平台可能存在滥用大数据支配优势。\n\n正如上文所讲的，平台的算法根据骑手、买家、卖家等信息，极限压缩配送时间。极限压缩配送时间是经济效率的体现，也可能压榨骑手的交易剩余。\n\n如何理解？\n在消费中，消费者剩余是消费者在交易中获得的“差额”财富。在劳动力市场中，劳动者的“交易剩余”不容易发现。由于马歇尔的“消费者剩余”理论没有推广到劳动力市场，所以，我们在理解这个问题需要兜几个弯。\n\n我们将马歇尔的“消费者剩余”推广到劳动力市场。不管是消费品市场还是劳动力市场，都存在“消费者剩余”和“生产者剩余”，我们统一使用“交易剩余”更好理解。\n\n在消费品交易中，买卖双方一手交钱一手交货，钱货两清，交易终止。交易双方在交易发生时便已经计算出了各自的“交易剩余”。但是，劳动力市场的交易要复杂得多，资方雇佣劳动，劳方给资方提供的是劳务，不容易衡量，却需要时间交付，具有相当的复杂性和不确定性。\n\n不考虑供求关系的情况下，劳方最开始处于信息优势方，资方处于劣势方。因为在雇佣劳动时，资方投入的沉没成本，如厂房、设备都是看得到的，而且劳方还可能随意滥用、支配、牵制这些设备和资源，给资方造成损失，错失商机。\n\n而劳方则只是带个“人”来，沉没成本低，且信息不透明。资方很难衡量员工到底有多大能耐，是否使出全部力气和智慧，是否偷懒敲竹杠，是否“寻租”。同时，劳动者的资源掌握在自己身上和脑袋（人力资本）中，人力资本与劳动者一般是不分离的，且容易带走。\n\n所以，理论上，劳方“剥削”资方。劳方“偷懒”，“敲竹杠”，“寻租”，便是在赚取更多的“交易剩余”。如果劳方不计加班费地干活，资方便赚取了更多的“交易剩余”。值得注意的是，劳资市场中的交易剩余、价格歧视都在过程之中，不容易被发现。\n\n资方为了改变信息不对称的地位，避免被雇员“磨洋工”、“敲竹杠”，选择与劳方签署雇佣合同，制定严厉的规则，以约束劳方行为。强化管理是为了阻止工人“剥削”资方，让劳动力的利益向资本的利益看齐。这就是“为什么是资方雇佣劳方”的逻辑。\n\n在18、19世纪，劳资双方的矛盾极为尖锐。在泰罗制引入工厂之前，工人的效率极低。资方采取粗暴，甚至违法的方式对付工人。工人则组建工会，甚至加入暴力团体，对抗、群殴资方。\n\n到了19世纪末，泰罗制引入大型工厂，资方找到了“科学管理”办法。劳资双方的矛盾有所缓和，计件工资的引入使得工人与资本家的目标趋于一致。\n\n但是，流水线的发明及推广，让劳资双方的地位发生了逆转。\n\n算法，即剥削\n1913福特汽车发明了第一条流水线后，劳资力量发生了逆转。泰罗制利用了分工理论研究工人的“动作”，流水线以机械化的方式落实了泰罗制。在流水线上，工人的身体、动作、时间、精力被重复的机器绑架。最早斯密在《国富论》中担忧，分工可能导致知识退化。到了流水线时代，工人已经被“机械化”【7】\n\n谁也无法否认，基于分工的泰罗制及流水线带来的工业效率革命。但是，这一工业效率背后潜藏着另一种“剥削”，那就是资方对劳方“交易剩余”的剥夺。工人只能利用大型工会，与资本家博弈，并在信息上、力量上形成相对均势。\n\n我们知道，马氏创造了剩余价值理论批判资本家的剥削行为。巴斯夏、米塞斯等经济学家，与马氏对抗，捍卫自由主义，被认为是资本家的代言人。但是，这两派势力都未能揭示问题的根本。\n\n管理学大师德鲁克年轻时经历过劳资矛盾所引发的极权主义斗争（《经济人的末日》，1937）。他在后来的《公司的概念》中批判了泰罗制及流水线对劳方的“剥夺”。他认为，这种方式违背了人的特性，抹杀了人的动机、兴趣、感受，以及综合、平衡、控制、判断等优势【8】。\n\n但是，不管是马氏、米氏还是德氏都没能揭示劳方的“交易剩余”被资方剥夺。人们忽略了斯密在《国富论》中的另一种担忧：知识积累带来规模递增，规模递增引发市场集中【7】。不管是流水线还是系统算法，正是利用了技术的垄断优势，帮助资方建立了信息优势和议价优势。\n\n在流水线上，任何工人都必须在某个时间比如1秒钟，完成一道工序，消灭了“磨洋工”。流水线上的时间，相当于给所有工人确定了强制性的价格。不管工人是否愿意，这个价格剥夺了工人的“交易剩余”。如果资方发现工人的动作越来越娴熟，还有时间，即交易剩余可以榨取，便会调高流水线的速度。\n\n如今，骑手变成了被算法支配的流水线上的工人。骑手们的工作时间完全被系统支配，当算法深度学习后发现还可以压缩时间，那么骑手们的“交易剩余”又被削减。\n\n理论上，算法还可以对每一个骑手实施“大数据杀熟”。针对每个骑手的数据确定不同的配送时间，相当于给每个歧视实施不同的“歧视性价格”，以完全剥夺所有骑手的“交易剩余”（不确定是否存在）。这就构成了一级价格歧视。\n\n资方居于信息优势方，骑手处于信息贫乏方，在算法面前没有任何议价能力。\n\n从经济学的角度来说，交易双方利用各自的信息，与对方进行价格博弈，本是一种正常的合理的竞争行为。正是这种竞争行为才促进技术进步及效率提升。但是，在自由市场中，其中一方获取了信息垄断优势，比如平台掌控了私人数据，对另一方实施价格歧视，最大限度地榨取“交易剩余”，那么价格将扭曲，经济效率也会下降。骑手的部分收益被平台攫取，财富长期向平台倾向，打乱自由市场的分配机制。\n\n可能有人会提出，自由竞争会解决这个问题。不愿意干骑手，可以去工厂上班。正是因为工厂上班工资太低，才更多人做骑手。\n\n真实的逻辑是，流水线“压榨”了工人的“交易剩余”。工人跑去送外卖，然后被算法压榨了“交易剩余”。工人和骑手的收入被压低，也会拉动整个劳工市场的工资水平。甚至，还可能对其它行业构成不正当竞争。比如，平台压榨了骑手的“交易剩余”，降低了外卖配送成本，还提高了配送效率，方便面企业却因此遭了殃。\n\n算法可以支配骑手，也可以支配我们每一个人。\n\n在大数据时代，平台理论上可以掌控每个买家的信息，对下压榨每个买家的全部“消费者剩余”；也可以掌控每个骑手的信息，对上压榨每个骑手的全部“交易剩余”。平台两头都可获得超额的垄断利润，导致财富向互联网巨头集中。\n\n比较中国与美国的互联网生态，我们会发现一个明显的不同。中国互联网形成了两大系，这两大系都热衷于终端的横向扩张，势力范围囊括零售、医疗、消费金融、网络支付、出行、住房、媒体、旅游、商业服务、物流。\n\n由于美国的反垄断法是条高压线，Facebook、谷歌、微软、亚马逊等公司不敢过度横向扩张，只能往纵深领域发展，如操作系统、人工智能、大数据、云计算、无人驾驶、通用芯片、导航系统、编程语言、机器人、基础科学等。\n\n中国互联网公司极少进入这些领域，而这些纵深领域才是核心技术所在。不可否认，中国互联网巨头在大数据、云计算及算法领域投入巨大。但是，这些领域所获得的成果，目的是为了在终端获取最大的“交易剩余”。在终端领域的扩张，中国互联网巨头获得了巨大资本红利，大量冠以大数据、云计算概念的终端消费公司上市套现。\n\n终端横向扩张的互联网生态至少造成三大问题：一是资本、人才无法进入纵深领域，核心技术创新不足；二是我们的生活被一个个强大的算法支配、包围和锁定；三是平台算法上下通吃，攫取了全社会的“交易剩余”，制造了财富集中及贫富分化，甚至出现有效需求不足。\n\n价格歧视理论揭示了被大数据、云计算“伪装”的算法剥削。\n\n中国的价格法规定，价格歧视是不正当价格行为。同时，即将在10月1日实施的《在线旅游经营服务管理暂行规定》禁止了大数据杀熟行为：在线旅游经营者不得滥用大数据分析等技术手段，基于旅游者消费记录、旅游偏好等设置不公平的交易条件，侵犯旅游者合法权益。\n\n近些年，美国及欧洲掀起了数据民主化运动。欧盟议会于2016年通过《通用数据保护条例》。该条例规定，任何收集、传输、保留或处理涉及到欧盟所有成员国内的个人信息的机构组织均受约束。这条例明确了个人数据权是公民的一项基本权利，应该得到尊重和保护。\n\n2019年7月8日，英国信息监管局发表声明说，英国航空公司因为违反《一般数据保护条例》被罚1.8339亿英镑（约合15.8亿元人民币）。\n\n虽然个人拥有保护数据的责任，但是根据汉德公式，最低成本的办法是限制大公司、大平台。1942年，美国诉罗尔拖公司一案中，法官汉德创意性地提出了一个公式来判案。汉德公式的意思是，预防未来事故成本小的一方应该受到限制。私人数据“被迫”存于平台之中，如果要每一个用户都保护好自己的数据隐私不被滥用显然不现实。\n\n要杜绝大数据杀熟，必须解决个人数据私有化问题。分布式信仰者试图通过点对点技术、加密算法等构建去中心化数据库。极客们不仅面临赛道拥堵、自治宪法等技术性挑战，还面临奥尔森所述的权力挑战。\n\n反科技“狂人”希尔多·卡辛斯基曾在《工业社会及其未来》一文发出警告：“工业化时代的人类，如果不是直接被高智能化的机器控制，就是被机器背后的少数精英所控制。”\n\n如果数据不私有化或未建立对算法的有效管控（注意前提），算法即剥削。\n\n参考文献：\n\n【1】外卖骑手，困在系统里，赖祐萱，人物；\n\n【2】外卖骑手困局背后 企业利润与ESG之间如何平衡，黄婉仪，21世纪经济报道；\n\n【3】我们正掉入外卖陷阱，竺晶莹，虎嗅；\n\n【4】歧视经济学，贝克尔，商务印书馆；\n\n【5】福利经济学，庇古，商务印书馆；\n\n【6】经济学原理，马歇尔，商务印书馆；\n\n【7】国富论，亚当·斯密，中央编译出版社；\n\n【8】公司的概念，德鲁克，机械工业出版社。"
        orginal text bytes length : 22467
    gz(level 6) compressed length :9889 , compress ratio  44.02% , cost 1.00ms
  lzma(level 6) compressed length :9240 , compress ratio  41.13% , cost 17.85ms
  zstd(level 6) compressed length :10374 , compress ratio  46.17% , cost 0.64ms
 zstd(level 19) compressed length :9225 , compress ratio  41.06% , cost 36.77ms
       rmw-utf8 compressed length :7457 , compress ratio  33.19% , cost 4.60ms

"There is no optimization for English. You can train yourself a dictionary optimized for a specific language. Or try shoco or smaz first."
        orginal text bytes length : 136
    gz(level 6) compressed length : 121 , compress ratio  88.97% , cost 0.02ms
  lzma(level 6) compressed length : 184 , compress ratio 135.29% , cost 6.00ms
  zstd(level 6) compressed length : 109 , compress ratio  80.15% , cost 1.83ms
 zstd(level 19) compressed length : 110 , compress ratio  80.88% , cost 27.91ms
       rmw-utf8 compressed length :  74 , compress ratio  54.41% , cost 0.08ms

压缩率平均值
lzma(level 6)	199.72%
gz(level 6)	130.09%
zstd(level 6)	108.06%
zstd(level 19)	107.23%
rmw-utf8	38.11%

```

测试机器为 MacBook Pro 2015 ( 2.2 GHz Intel Core i7 )

测试代码见 [compress_test](https://github.com/rmw-link/rmw-utf8/tree/master/compress_test)

## 使用注意

压缩会把文本中的`\r\n`和`\r`都替换为`\n`，也就是说，压缩和解压的文本未必完全等同。

###  历史故事

`\r`是回车，`\n`是换行，前者使光标到行首，后者使光标下移一格。

很久很久以前，在计算机还没诞生，有一种机器叫做电传打字机（Teletype Model），每秒钟可以打10个字符。

它有个问题，就是换行要0.2秒。要是在这0.2秒里面，又有新的字符传过来，那么这个字符将丢失。

于是，研发人员想了个办法，就是在每行后面加两个表示结束的字符。

一个叫做“回车”，告诉打字机把打印头定位在左边界；另一个叫做“换行”，告诉打字机把纸向下移一行。

这就是“换行”和“回车”的来历。

后来，计算机发明了，这两个概念也就被般到了计算机上。那时，存储器很贵，一些科学家认为在每行结尾加两个字符太浪费了，加一个就可以。

于是，世界裂开了。

Unix/Linux系统，每行结尾只有“<换行>”，即`\n`；Windows系统，默认是“<回车><换行>”，即`\r\n`；Mac系统，默认是“<回车>”也就是`\r`。

时过境迁，现代的文本编辑器都支持`\n`作为结尾字符了，所以`\r`也就没什么存在的必要了。

## 训练自定义字典

针对不同语言、不同类型的文本，可以训练一套自己的压缩字典以增强压缩效果。

```
git clone --depth=1 https://github.com/rmw-link/rmw-utf8.git
cd rmw-utf8
# 在txt目录下放你的准备训练语料，格式为utf8编码的txt文件
cd train
./train.sh
```

## 缺失的特性

没做流式压缩（毕竟我的场景主要是短文本）。

有需要的人可以自己再封装一个流式压缩。

比如每1MB压缩下，压缩后每段开头再记录下压缩后的内容的字节数。


## 创作随笔

互联网犹如末期的封建王朝，寡头们完成了土地兼并，垄断了人民创造的数据，坐收地租。

我不喜欢这样的世界。我想寻求改变。

我想重新构建一个全新的互联网 ：让每个人拥有自己的数据。让人民的数据归属于人民。

于是，我开始了新的个人项目 —— [人民网络](https://rmw.link)。

因为打算从底层重构整个互联网架构，所以，我计划一切都自己从底层搭建。

首先，我封装了sanakirja数据库 —— [sdb](https://docs.rs/crate/sdb) （这个数据库会崩溃问题还有点多，不推荐现在使用）。

根据sanakirja作者所说，[sanakirja比sled要快10倍](https://www.reddit.com/r/rust/comments/lp5jez/sanakirja_10_pure_rust_transactional_ondisk/)。

> Sanakirja is at least 10 times faster than Sled in my (sequential) benchmarks, and even 20%-50% faster than LMDB (the fastest C equivalent) in the same benchmarks. Also, I started it when there was no real alternative (Sled didn't exist at the time).

我没有实测性能，选sanakirja的原因主要是因为他原生支持rust的数据类型，用起来方便。

而sled或者lmdb，都只支持不定长字符串作为键值，这其中就多了额外的空间开销以及序列化和反序列化的成本。

sanakirja用mmap把存储的文件都映射到了内存，并且每个条目的最大大小为1020字节。

所以比较适合做索引数据库，大段内容可以考虑另开日志存储（比如使用[commitlog](https://lib.rs/crates/commitlog)），然后在sanakirja中记录对应日志存储编号即可。

用来存些短文本，问题也不大（比如140字以内的微博、日常聊天、文件名等等）。

是的，我就打算如此使用。

因为所有内容都被mmap映射到内存（类似lmdb的方案），虽说可以用虚拟内存并不受到物理内存的限制，但普通用户电脑上虚拟内存设置也不大，所以还是节省着用比较好。

所以我希望可以能对以中文为主体的短文本内容做些压缩。

## 如何压缩中文短文本？（十几到一两百个字、主要为中文的文本）

通用压缩算法并不适合短文本的场景，比如我测试了当世最强的压缩算法[zstd](https://github.com/facebook/zstd)，经常是把42个字节压缩成为62个字节（是的，没压缩反而放大了），即使训练了字典也不太行（我没搞明白怎么让zstd以3个字节为单位来建立字典，我cat了下zstd的字典，里面都是短句子）。

有一些面向短文本压缩算法，比如[shoco](https://ed-von-schleck.github.io/shoco/)、[smaz](https://github.com/antirez/smaz)。但这都只对类英文的语言有压缩效果，对短中文依然是放大（他们字典都只有几百个字符，不够用，所以即使重新训练字典也不可行）。

另一种压缩方案就是改文字的编码。

如果有unicode编码有所了解，那么就自然明白utf-8编码方案中一个中文字符需要三个字节的存储空间（其实挺浪费的）。

gb18030中一个中文两个字节，一下子就省了33%的空间。但是gb18030并不涵盖所有的unicode字符（只是utf8的子集），没法用。

有些标准化的unicode压缩编码，比如[scsu](https://github.com/dop251/scsu)（[SqlServer用的就是这个](https://docs.microsoft.com/en-us/sql/relational-databases/data-compression/unicode-compression-implementation?view=sql-server-ver15)）、[utf-c](https://github.com/deNULL/utf-c)。

我[测试了一下](https://denull.github.io/utf-c)，差不多一个中文两个字节，再外加一个额外字节（比如4个中文大概是2*4+1=9个字节）。

![](https://raw.githubusercontent.com/gcxfd/img/gh-pages/ffxMd3.jpg)

最关键的是，我搜索了全网，没有找到这两种编码的rust实现。

自己写这些编码的rust实现也不是不可以，但是这都需要对unicode各种语言的码表区间有比较深入的理解，学习成本高，躺平、不想学、搞不定。

于是我想，能不能做个更通用更优的编码压缩方案。

unicode字符数是固定已知的，unicode-13.0.0方案有143859个字符([参见这里](https://github.com/rmw-link/utf8_compress/blob/master/all_char.py))。

完全可以统计下每个字的出现频率，然后用霍夫曼编码来压缩。

于是，用了一些中文语料，开始统计字频率。

语料如下：

* [维基百科中文语料](https://jdhao.github.io/2019/01/10/two_chinese_corpus)
* [FictionDown网络小说爬虫](https://github.com/ma6254/FictionDown)(release版本会重复不断抓取失效的章节，所以需要用master版本 `go get github.com/ma6254/FictionDown@master`)
* [微博爬虫](https://github.com/gcxfd/weibo-crawler)
* [BT网络的DHT爬虫](https://github.com/gcxfd/bt-spider)
* [几个随手写的爬虫，见代码spider目录](https://github.com/rmw-link/utf8_compress/tree/master/spider)

小试牛刀，效果不错，三个中文可以压到5个字节，已经超出gb18030的压缩效果了。

我进一步遐想，是不是可以把常用的词也加到霍夫曼的字典中去，进一步优化压缩效果。

于是，就用分词+ngram[参见train目录下的训练算法](https://github.com/rmw-link/rmw-utf8/tree/master/train)做了一个带常用的词的字典（压缩后500多KB）。

试了试效果，碾压市面上一切压缩算法。

酷。

编码问题到此为止。

继续写人民网络去。


