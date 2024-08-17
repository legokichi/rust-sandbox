import './style.css';
import xs from 'xstream';
import { Stream } from 'xstream';
import * as Cycle from '@cycle/run'
import { makeOpenLayersDriver } from './oldriver';
import type { OlCommand, OlSource, OlCommandAddWaypoint, Coordinate } from './oldriver';
import { makeGeoLocationDriver } from './geolocationdriver';
import type { GeoCommand, GeoSource } from './geolocationdriver';

interface Sources {
  OL: OlSource;
  GEO: GeoSource;
}

interface Sinks {
  OL: Stream<OlCommand>;
  GEO: Stream<GeoCommand>;
}

function main(o: Sources): Sinks {
  const {
    OL: { clickGps$, clickMap$, clickAddWaypoint$ },
    GEO: { pos$ },
  } = o;
  const posChanged$ = pos$.fold((prev, curr) => {
    return [prev[1], curr];
  }, [{
    longtiude: 0,
    latitude: 0,
    accuracy: 0
  }, {
    longtiude: 0,
    latitude: 0,
    accuracy: 0
  }]).filter(([a, b]) => a.accuracy !== b.accuracy || a.latitude !== b.latitude || a.longtiude !== b.longtiude).map(([_a, b]) => b);
  const ol$1: Stream<OlCommand> = xs.combine(clickGps$, posChanged$).map(
    ([_click, pos]): OlCommand => {
      return {
        type: "focus",
        longitude: pos.longtiude,
        latitude: pos.latitude,
      };
    }
  );
  const ol$2: Stream<OlCommand> = pos$.map(({
    longtiude,
    latitude,
    accuracy
  }): OlCommand => {
    return {
      type: "updateCurrentPosition",
      longtiude,
      latitude,
      accuracy
    };
  });
  const ol$3: Stream<OlCommand> = clickAddWaypoint$.map(({
    longitude,
    latitude
  }): OlCommand => {
    return {
      type: "addWaypoint",
      longitude,
      latitude,
    };
  });
  const ol$4 = clickMap$.filter((_) => false).map((_) => { throw new Error("unreachable") });
  // EPSG:4326 (WGS 84 の EPSGコード) 座標系
  const waypoints: { [key: string]: Coordinate } = {
    "富士山": [138.7313889, 35.3622222],
    "秩父公園橋": [139.0719155531588, 36.002780508652975],
    "武之鼻橋": [139.0725162723582, 36.003201622651204],
    "せせらぎ荘の瀬": [139.0765789104678, 36.00723361904889],
    "ゴルフ場の瀬": [139.07697137565765, 36.01068857129275],
    "下水処理場の瀬": [139.07821077059012, 36.013009368087765],
    "秩父橋前のザラ瀬": [139.0838604619643, 36.01634060666794],
    "秩父橋前の隠れ岩": [139.08574555926472, 36.01751730197438],
    "秩父橋": [139.086117278947, 36.01785811396044],
    "旧秩父橋": [139.08650512930004, 36.01862531457782],
    "宮崎城跡前の瀬": [139.08889855209182, 36.02143244420989],
    "秩父太平洋セメント前のザラ瀬": [139.08909762763685, 36.02599732396122],
    "秩父太平洋セメント前の水制": [139.0877188250745, 36.02797501259279],
    "秩父太平洋セメントベルトコンベヤーの橋": [139.0858824694279, 36.02897258934499],
    "秩父太平洋セメント前の瀬": [139.08902304435563, 36.031320758803716],
    "送電線": [139.09226412930903, 36.03668446479628],
    "二股": [139.09367281644546, 36.040082273715484],
    "瀬": [139.0958552454708, 36.042222927153645],
    "諏訪城跡の瀬": [139.09948522336654, 36.042882528765844],
    "横瀬川流入": [139.09976136985387, 36.04440005756092],
    "地層のザラ瀬": [139.09935668758064, 36.04485715632296],
    "和銅大橋": [139.09906482029547, 36.045662765285755],
    "氷雨塚の瀬": [139.0987702435825, 36.04666107627774],
    "消波ブロック": [139.0990103603088, 36.05404306242967],
    "消波ブロックの瀬": [139.09844546520998, 36.05491127632135],
    "蒔田川前の地層ドロップ": [139.0955381337743, 36.05775360989348],
    "新皆野橋前の瀬": [139.09420292816912, 36.05990598040812],
    "新皆野橋（秩父やまなみ大橋）": [139.0937991035614, 36.06041177186199],
    "薬師堂下古墳の瀬": [139.0915281206684, 36.06325478563795],
    "皆野橋": [139.08683343011208, 36.066221192261494],
    "皆野橋の瀬": [139.08682814676, 36.06704496612902],
    "赤平川流入": [139.08633610146802, 36.068195303924625],
    "皆野駅前のザラ瀬": [139.0911909064733, 36.07224207747889],
    "皆野駅前のザラ瀬2": [139.0952014667693, 36.07460900018707],
    "栗谷瀬橋前の地層ドロップ": [139.0958315536906, 36.07710793544136],
    "栗谷瀬橋": [139.09648892709652, 36.07970773070484],
    "栗谷瀬橋下EP": [139.09910095952117, 36.08020459406606],
    "ウォーターパーク長瀞前のザラ瀬": [139.1029328438854, 36.08120521841272],
    "親鼻橋下EP（ライン下りAコース乗船場）": [139.11096796836785, 36.08218670597326],
    "親鼻橋": [139.1093862083123, 36.08201746611897],
    "親鼻鉄橋": [139.1143966034843, 36.08395693744913],
    "鉄橋下の瀬(1級)": [139.11503232956505, 36.084094960385954],
    "ステミの瀬(1.5級)": [139.11513116562165, 36.084365380354825],
    "セイゴの瀬(2級)": [139.11785814011483, 36.08581321213056],
    "コタキの瀬(2.5級)": [139.1172523238773, 36.08965099170375],
    "ライン下り発着場": [139.1152709330081, 36.09537404754023],
    "ピンポールの瀬(1級)": [139.11441676067753, 36.09919316977167],
    "白鳥荘二股の瀬(1.5級)": [139.11454012184967, 36.09972346220377],
    "金石水管橋": [139.11306485062124, 36.10517131775538],
    "キャンプ場前の瀬(1級)": [139.11316964362655, 36.10580907020025],
    "サイコロ岩": [139.11615904936005, 36.110159118014536],
    "高砂橋": [139.1167424927241, 36.11070389221439],
    "高砂橋の瀬(ドロップ)": [139.11693855014067, 36.11106753802916],
    "ライン下りゴール":[ 139.1186464790973, 36.11320768728517],
    "クツナシの瀬(1.5級)": [139.11926020969383, 36.11326318552129],
    "洗濯機の瀬(1級)": [139.11879485006364, 36.1144341517747],
    "ハーブ研究所の瀬": [139.11557811545947, 36.116718327617065],
    "ドビーの瀬(2級)": [139.11384175981132, 36.12104181534758],
    "宮沢の瀬":[ 139.11372611314187, 36.12342728651372],
    "岩田前EP": [139.11606475098313, 36.12473134411758],
    "岩田の瀬": [139.11706029437553, 36.12509438509453],
    "高橋の瀬": [139.11843891377535, 36.126238334622315],
    "最後の瀬": [139.12054374582158, 36.12927824388774],
    "消波ブロック2": [139.12094938205328, 36.12969096410883],
    "護岸の壁(ポーテージEP)": [139.12184727494045, 36.130187934893314],
    "国体コースの瀬": [139.1227239383733, 36.1301792098962],
    "左シュート右コース": [139.12307109432462, 36.1303786445835],
    "岩超えEP": [139.12327136351757, 36.13067374114304],
    "白鳥橋":[139.12635673431276, 36.13231132085089],
    "下郷EP":[139.13578485635776, 36.132373526147504],
    "寄居橋": [139.15642000228442, 36.125776760332926],
    "玉淀ダムのEP": [139.16127969112873, 36.117017779899484],
    "玉淀ダム": [139.16953024811244, 36.116160102616234],

    "常陸大子駅": [140.35080250269525, 36.770774326721806],
    "湯の里公園EP": [140.35742028244655, 36.76667757086997],
    "久慈川橋": [140.35813015837178, 36.762228431857224],
    "仮設橋PTG": [140.36966126454843, 36.76128870135851],
    "北田気大橋": [140.37006262045577, 36.76180190486879],
    "久野瀬橋（沈下橋）": [140.3783319930691, 36.76089391358181],
    "第六久慈川橋梁": [140.37919813174506, 36.7577207966265],
    "南田気大橋": [140.37662866812434, 36.75633516390502],
    "新昭和橋": [140.37670943379254, 36.75171239521721],
    "第五久慈川橋梁": [140.3796813213811, 36.75023381239245],
    "岩絡みの瀬": [140.38270137159387, 36.74486007191567],
    "下津原橋": [140.38243301294884, 36.74459249053717],
    "下津原橋下EP": [140.38200226378663, 36.74499555620106],
    "シャモの瀬(1.5)": [140.3750527069436, 36.73711161147121],
    "鰐ヶ淵橋": [140.3875716004426, 36.737070841048535],
    "瀬(1.5)": [140.38815405170078, 36.73704490865313],
    "奥久慈橋": [140.38770034647536, 36.73084565900146],
    "瀬2": [140.38805873334738, 36.73022838415318],
    "第四久慈川橋梁": [140.37781142501007, 36.73015776261283],
    "鉄橋下消波ブロック左岸ポーテージ": [140.37789722254547, 36.73043758359897],
    "瀬4": [140.37155335146826, 36.72481867353004],
    "上小川橋": [140.37274741438972, 36.72011377353866],
    "消波ブロック3":[140.3795158212913, 36.71775590298124],
    "第三久慈川橋梁": [140.38454913405548, 36.716295337227635],
    "消波ブロック4":[140.38439424486737, 36.71642633980228],
    "宮平橋": [140.3851586634755, 36.7169938866642],
    "消波ブロック5":[140.3888511573862, 36.714533737051525],
    "御城橋": [140.38701080503512, 36.71409471436456],
    "第二久慈川橋梁": [140.38660359788128, 36.71403313622072],
    "鉄橋下の瀬1": [140.38566898130327, 36.71427812035678],
    "仮設橋3PTG": [140.38364497056943, 36.71186910171957],
    "消波ブロック6": [140.38178855105178, 36.709356481032344],
    "消波ブロック7":[140.38310507726555, 36.69993083819344],
    "西金大橋": [140.38913742535934, 36.6923698465521],
    "大内野橋": [140.38873160599792, 36.684856256473466],
    "瀬(1)": [140.38843126445389, 36.68494379878156],
    "第一久慈川橋梁": [140.38747602314749, 36.67922582795006],
    "下小川橋": [140.3880044292847, 36.67758848782043],
    "橋下消波ブロック右岸ポーテージ": [140.38804273702493, 36.677555953140256],
    "左岸EP": [140.3884544639313, 36.6662684945092],
    "平山橋（沈下橋）": [140.3879894614594, 36.6658553575746],
    "堰": [140.38825731374612, 36.66230703709776],
  };
  const commands: Array<OlCommandAddWaypoint> = Object.keys(waypoints).map((key) => {
    return {
      type: "addWaypoint",
      longitude: waypoints[key][0],
      latitude: waypoints[key][1],
    };
  });
  const ol$5 = xs.fromArray(commands);
  return {
    OL: xs.merge(ol$1, ol$2, ol$3, ol$4, ol$5),
    GEO: xs.never(),
  };
}



Cycle.run(main as any, {
  GEO: makeGeoLocationDriver(),
  OL: makeOpenLayersDriver(),
} as any);


