PRAGMA foreign_keys = ON;

CREATE TABLE rivers (
   river_id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
   name TEXT NOT NULL,
   created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
   updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);


-- CREATE TABLE river_paths (
--     ancestor INTEGER NOT NULL,
--     descendant INTEGER NOT NULL,
--     PRIMARY KEY (ancestor, descendant),
--     FOREIGN KEY (ancestor) REFERENCES rivers(river_id),
--     FOREIGN KEY (descendant) REFERENCES rivers(river_id)
-- ) WITHOUT ROWID;
-- INSERT INTO river_paths (ancestor, descendant) VALUES (1, 1);
-- INSERT INTO river_paths (ancestor, descendant) VALUES (2, 2);
-- INSERT INTO river_paths (ancestor, descendant) VALUES (3, 3);
-- INSERT INTO river_paths (ancestor, descendant) VALUES (4, 4);
-- INSERT INTO river_paths (ancestor, descendant) VALUES (5, 5);
-- INSERT INTO river_paths (ancestor, descendant) VALUES (6, 6);

CREATE TABLE river_waypoints (
  river_waypoint_id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  river_id INTEGER NOT NULL,
  name TEXT NOT NULL,
  description TEXT NOT NULL DEFAULT '',
  longitude REAL NOT NULL,
  latitude REAL NOT NULL,
  elevation REAL,
  distance REAL NOT NULL,
  created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
  updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
  FOREIGN KEY (river_id) references rivers(river_id) ON DELETE CASCADE
);

INSERT INTO rivers (name) VALUES ('無');

INSERT INTO rivers (name) VALUES ('荒川長瀞');
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (1, 1.0, '秩父公園橋', 139.0719155531588, 36.002780508652975);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (1, 2.0, '武之鼻橋', 139.0725162723582, 36.003201622651204);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (1, 3.0, 'せせらぎ荘の瀬', 139.0765789104678, 36.00723361904889);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (1, 4.0, 'ゴルフ場の瀬', 139.07697137565765, 36.01068857129275);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (1, 5.0, '下水処理場の瀬', 139.07821077059012, 36.013009368087765);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (1, 6.0, '秩父橋前のザラ瀬', 139.0838604619643, 36.01634060666794);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (1, 7.0, '秩父橋前の隠れ岩', 139.08574555926472, 36.01751730197438);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (1, 8.0, '秩父橋', 139.086117278947, 36.01785811396044);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (1, 9.0, '旧秩父橋', 139.08650512930004, 36.01862531457782);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (1, 10.0, '宮崎城跡前の瀬', 139.08889855209182, 36.02143244420989);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (1, 11.0, '秩父太平洋セメント前のザラ瀬', 139.08909762763685, 36.02599732396122);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (1, 11.0, '秩父太平洋セメント前の水制', 139.0877188250745, 36.02797501259279);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (1, 13.0, '秩父太平洋セメントベルトコンベヤーの橋', 139.0858824694279, 36.02897258934499);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (1, 14.0, '秩父太平洋セメント前の瀬', 139.08902304435563, 36.031320758803716);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (1, 15.0, '送電線', 139.09226412930903, 36.03668446479628);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (1, 16.0, '二股', 139.09367281644546, 36.040082273715484);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (1, 17.0, '瀬', 139.0958552454708, 36.042222927153645);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (1, 18.0, '諏訪城跡の瀬', 139.09948522336654, 36.042882528765844);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (1, 19.0, '横瀬川流入', 139.09976136985387, 36.04440005756092);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (1, 20.0, '地層のザラ瀬', 139.09935668758064, 36.04485715632296);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (1, 21.0, '和銅大橋', 139.09906482029547, 36.045662765285755);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (1, 22.0, '氷雨塚の瀬', 139.0987702435825, 36.04666107627774);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (1, 23.0, '消波ブロック', 139.0990103603088, 36.05404306242967);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (1, 24.0, '消波ブロックの瀬', 139.09844546520998, 36.05491127632135);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (1, 25.0, '蒔田川前の地層ドロップ', 139.0955381337743, 36.05775360989348);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (1, 26.0, '新皆野橋前の瀬', 139.09420292816912, 36.05990598040812);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (1, 27.0, '新皆野橋（秩父やまなみ大橋）', 139.0937991035614, 36.06041177186199);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (1, 28.0, '薬師堂下古墳の瀬', 139.0915281206684, 36.06325478563795);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (1, 29.0, '皆野橋', 139.08683343011208, 36.066221192261494);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (1, 30.0, '皆野橋の瀬', 139.08682814676, 36.06704496612902);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (1, 31.0, '赤平川流入', 139.08633610146802, 36.068195303924625);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (1, 32.0, '皆野駅前のザラ瀬', 139.0911909064733, 36.07224207747889);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (1, 33.0, '皆野駅前のザラ瀬2', 139.0952014667693, 36.07460900018707);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (1, 34.0, '栗谷瀬橋前の地層ドロップ', 139.0958315536906, 36.07710793544136);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (1, 35.0, '栗谷瀬橋', 139.09648892709652, 36.07970773070484);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (1, 36.0, '栗谷瀬橋下EP', 139.09910095952117, 36.08020459406606);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (1, 37.0, 'ウォーターパーク長瀞前のザラ瀬', 139.1029328438854, 36.08120521841272);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (1, 38.0, '親鼻橋下EP（ライン下りAコース乗船場）', 139.11096796836785, 36.08218670597326);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (1, 39.0, '親鼻橋', 139.1093862083123, 36.08201746611897);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (1, 40.0, '親鼻鉄橋', 139.1143966034843, 36.08395693744913);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (1, 41.0, '鉄橋下の瀬(1級)', 139.11503232956505, 36.084094960385954);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (1, 42.0, 'ステミの瀬(1.5級)', 139.11513116562165, 36.084365380354825);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (1, 43.0, 'セイゴの瀬(2級)', 139.11785814011483, 36.08581321213056);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (1, 44.0, 'コタキの瀬(2.5級)', 139.1172523238773, 36.08965099170375);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (1, 45.0, 'ライン下り発着場', 139.1152709330081, 36.09537404754023);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (1, 46.0, 'ピンポールの瀬(1級)', 139.11441676067753, 36.09919316977167);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (1, 47.0, '白鳥荘二股の瀬(1.5級)', 139.11454012184967, 36.09972346220377);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (1, 48.0, '金石水管橋', 139.11306485062124, 36.10517131775538);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (1, 49.0, 'キャンプ場前の瀬(1級)', 139.11316964362655, 36.10580907020025);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (1, 50.0, 'サイコロ岩', 139.11615904936005, 36.110159118014536);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (1, 51.0, '高砂橋', 139.1167424927241, 36.11070389221439);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (1, 52.0, '高砂橋の瀬(ドロップ)', 139.11693855014067, 36.11106753802916);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (1, 53.0, 'ライン下りゴール', 139.1186464790973, 36.11320768728517);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (1, 54.0, 'クツナシの瀬(1.5級)', 139.11926020969383, 36.11326318552129);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (1, 55.0, '洗濯機の瀬(1級)', 139.11879485006364, 36.1144341517747);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (1, 56.0, 'ハーブ研究所の瀬', 139.11557811545947, 36.116718327617065);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (1, 57.0, 'ドビーの瀬(2級)', 139.11384175981132, 36.12104181534758);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (1, 58.0, '宮沢の瀬', 139.11372611314187, 36.12342728651372);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (1, 59.0, '岩田前EP', 139.11606475098313, 36.12473134411758);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (1, 60.0, '岩田の瀬', 139.11706029437553, 36.12509438509453);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (1, 61.0, '高橋の瀬', 139.11843891377535, 36.126238334622315);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (1, 62.0, '最後の瀬', 139.12054374582158, 36.12927824388774);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (1, 63.0, '消波ブロック2', 139.12094938205328, 36.12969096410883);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (1, 64.0, '護岸の壁(ポーテージEP)', 139.12184727494045, 36.130187934893314);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (1, 65.0, '国体コースの瀬', 139.1227239383733, 36.1301792098962);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (1, 66.0, '左シュート右コース', 139.12307109432462, 36.1303786445835);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (1, 67.0, '岩超えEP', 139.12327136351757, 36.13067374114304);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (1, 68.0, '白鳥橋', 139.12635673431276, 36.13231132085089);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (1, 69.0, '下郷EP', 139.13578485635776, 36.132373526147504);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (1, 70.0, '寄居橋', 139.15642000228442, 36.125776760332926);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (1, 71.0, '玉淀ダムのEP', 139.16127969112873, 36.117017779899484);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (1, 72.0, '玉淀ダム', 139.16953024811244, 36.116160102616234);

INSERT INTO rivers (name) VALUES ('多摩川');
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (2, 1.0, '白丸駅', 139.11484235039475, 35.81197300620613);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (2, 2.0, '数馬峡橋', 139.11509811594544, 35.80979171198217);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (2, 3.0, '白丸湖EP', 139.1163506934866, 35.81072660234875);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (2, 4.0, '寒山寺 駐車場', 139.19349356696478, 35.803138785806055);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (2, 5.0, '御岳駅', 139.18244153331304, 35.80143116199427);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (2, 6.0, '御岳橋', 139.18297566550882, 35.8007385662186);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (2, 7.0, '御岳EP', 139.18541866232138, 35.80160407389222);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (2, 8.0, '鵜の瀬橋', 139.18844944967992, 35.804899355018506);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (2, 9.0, '楓橋', 139.19410920654443, 35.80372655974858);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (2, 10.0, '軍畑大橋', 139.20781173508183, 35.80522764257336);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (2, 11.0, '奥多摩橋', 139.21562497429582, 35.80081643252839);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (2, 12.0, '好文橋', 139.22136883783207, 35.79633162894858);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (2, 13.0, '神代橋', 139.22766577508216, 35.789454976943176);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (2, 14.0, '和田橋', 139.2315517872428, 35.78164251380883);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (2, 15.0, '万年橋', 139.24920211499773, 35.7850013528697);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (2, 16.0, '柳淵橋', 139.25435967856814, 35.7857529907149);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (2, 17.0, '釜の淵公園EP', 139.25334577149258, 35.78578140172303);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (2, 18.0, '釜の淵公園大柳駐車場', 139.25344139573681, 35.78564120889584);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (2, 19.0, '釜の淵公園駐車場', 139.25569029811456, 35.784451978337216);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (2, 20.0, '青梅駅', 139.25834360426683, 35.79027062559453);

INSERT INTO rivers (name) VALUES ('久慈川');
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (3, 1.0, '常陸大子駅', 140.35080250269525, 36.770774326721806);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (3, 2.0, '湯の里公園EP', 140.35742028244655, 36.76667757086997);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (3, 3.0, '久慈川橋', 140.35813015837178, 36.762228431857224);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (3, 4.0, '仮設橋PTG', 140.36966126454843, 36.76128870135851);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (3, 5.0, '北田気大橋', 140.37006262045577, 36.76180190486879);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (3, 6.0, '久野瀬橋（沈下橋）', 140.3783319930691, 36.76089391358181);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (3, 7.0, '第六久慈川橋梁', 140.37919813174506, 36.7577207966265);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (3, 8.0, '南田気大橋', 140.37662866812434, 36.75633516390502);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (3, 9.0, '新昭和橋', 140.37670943379254, 36.75171239521721);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (3, 10.0, '第五久慈川橋梁', 140.3796813213811, 36.75023381239245);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (3, 11.0, '岩絡みの瀬', 140.38270137159387, 36.74486007191567);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (3, 12.0, '下津原橋', 140.38243301294884, 36.74459249053717);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (3, 13.0, '下津原橋下EP', 140.38200226378663, 36.74499555620106);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (3, 14.0, 'シャモの瀬(1.5)', 140.3750527069436, 36.73711161147121);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (3, 15.0, '鰐ヶ淵橋', 140.3875716004426, 36.737070841048535);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (3, 16.0, '瀬(1.5)', 140.38815405170078, 36.73704490865313);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (3, 17.0, '奥久慈橋', 140.38770034647536, 36.73084565900146);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (3, 18.0, '瀬2', 140.38805873334738, 36.73022838415318);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (3, 19.0, '第四久慈川橋梁', 140.37781142501007, 36.73015776261283);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (3, 20.0, '鉄橋下消波ブロック左岸ポーテージ', 140.37789722254547, 36.73043758359897);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (3, 21.0, '瀬4', 140.37155335146826, 36.72481867353004);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (3, 22.0, '上小川橋', 140.37274741438972, 36.72011377353866);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (3, 23.0, '消波ブロック3', 140.3795158212913, 36.71775590298124);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (3, 24.0, '第三久慈川橋梁', 140.38454913405548, 36.716295337227635);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (3, 25.0, '消波ブロック4', 140.38439424486737, 36.71642633980228);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (3, 26.0, '宮平橋', 140.3851586634755, 36.7169938866642);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (3, 27.0, '消波ブロック5', 140.3888511573862, 36.714533737051525);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (3, 28.0, '御城橋', 140.38701080503512, 36.71409471436456);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (3, 29.0, '第二久慈川橋梁', 140.38660359788128, 36.71403313622072);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (3, 30.0, '鉄橋下の瀬1', 140.38566898130327, 36.71427812035678);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (3, 31.0, '仮設橋3PTG', 140.38364497056943, 36.71186910171957);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (3, 32.0, '消波ブロック6', 140.38178855105178, 36.709356481032344);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (3, 33.0, '消波ブロック7', 140.38310507726555, 36.69993083819344);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (3, 34.0, '西金大橋', 140.38913742535934, 36.6923698465521);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (3, 35.0, '大内野橋', 140.38873160599792, 36.684856256473466);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (3, 36.0, '瀬(1)', 140.38843126445389, 36.68494379878156);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (3, 37.0, '第一久慈川橋梁', 140.38747602314749, 36.67922582795006);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (3, 38.0, '下小川橋', 140.3880044292847, 36.67758848782043);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (3, 39.0, '橋下消波ブロック右岸ポーテージ', 140.38804273702493, 36.677555953140256);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (3, 40.0, '左岸EP', 140.3884544639313, 36.6662684945092);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (3, 41.0, '平山橋（沈下橋）', 140.3879894614594, 36.6658553575746);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (3, 42.0, '堰', 140.38825731374612, 36.66230703709776);

INSERT INTO rivers (name) VALUES ('利根川');
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (4, 1.0, '奥利根湖ゲート', 139.05839374130173, 36.86386954837418);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (4, 2.0, '矢木沢ダムEP', 139.05189725730443, 36.91221208964946);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (4, 3.0, '矢木沢ダム駐車場', 139.05307807306556, 36.911018264155814);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (4, 4.0, '奥利根湖事務所', 139.05227413894397, 36.91139461047338);



INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (4, 1.0, '湯檜曽駅ゆびそ', 138.98646812422595, 36.80322412356702);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (4, 2.0, 'スタート', 138.9884753879484, 36.80375698348929);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (4, 3.0, '幸知橋こうちばし', 138.98735164626947, 36.79961110116844);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (4, 4.0, 'ダイナマイトの瀬', 138.99059569321594, 36.7967844876507);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (4, 5.0, 'スリーシスターズ＆ザ・ウォールの瀬', 138.99011974357992, 36.79578160890378);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (4, 6.0, '水明荘プットイン', 138.97708703494513, 36.78540756603442);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (4, 7.0, 'スリーウェイズの瀬', 138.9770931868941, 36.78455185334815);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (4, 8.0, '第八利根川橋梁', 138.98283086560878, 36.790323062983006);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (4, 9.0, '大鹿橋', 138.9766704935036, 36.784031959986635);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (4, 10.0, '第七利根川橋梁', 138.97646159887333, 36.78362823967291);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (4, 11.0, '第六利根川橋梁', 138.9735632252436, 36.782947280807335);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (4, 12.0, '谷川橋', 138.96821152955684, 36.781990133152945);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (4, 13.0, 'JA前プットイン', 138.96677312289464, 36.7774973059239);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (4, 14.0, '湯原橋', 138.97063062448825, 36.773363336652665);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (4, 15.0, '水上ウェーブスの瀬', 138.97147983563187, 36.76974757801139);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (4, 16.0, '水上橋', 138.97156398476886, 36.768628606015255);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (4, 17.0, '紅葉橋', 138.97054310545406, 36.76603688333975);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (4, 18.0, '笹笛橋', 138.97087135326413, 36.76114514369988);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (4, 19.0, 'AKIKOの瀬', 138.97088591255465, 36.76106651773138);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (4, 20.0, '諏訪峡大橋', 138.97360567249578, 36.75972134449587);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (4, 21.0, '竜ヶ瀬', 138.974034879457, 36.75954160380678);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (4, 22.0, 'フリッパーズの瀬', 138.9760166884277, 36.758177067158655);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (4, 23.0, 'ショットガンの瀬', 138.97661906450782, 36.757554758588654);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (4, 24.0, 'メガウォッシュの瀬', 138.97692817854886, 36.75672923942963);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (4, 25.0, '銚子橋', 138.97655153538128, 36.7532096446173);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (4, 26.0, '銚子橋プットイン', 138.97552810937503, 36.75214311165925);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (4, 27.0, 'なすびホールの瀬', 138.9781666596558, 36.747801219938125);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (4, 28.0, 'ホットウェーブスの瀬', 138.98349476219892, 36.738766121460486);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (4, 29.0, '吾妻橋', 138.98278202114417, 36.73762377278045);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (4, 30.0, '利根橋', 138.97980631828187, 36.72374507030281);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (4, 31.0, '木の根ホール', 138.97928443048318, 36.72399585071685);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (4, 32.0, '利根橋前堰堤', 138.98053408897837, 36.723446772824914);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (4, 33.0, '上牧発電所前堰堤', 138.9786200431598, 36.71349645771582);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (4, 34.0, '大峰橋', 138.9788558778381, 36.7098484321733);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (4, 35.0, '関越ウェーブス', 138.9788141917799, 36.708539838986425);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (4, 36.0, 'スラッパー', 138.9788339231861, 36.70348716046914);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (4, 37.0, '矢瀬橋', 138.9826917940874, 36.699031140868826);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (4, 38.0, 'ホタル', 138.98366348297225, 36.695962294825236);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (4, 39.0, 'テトリス', 138.98371593193784, 36.69518556598884);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (4, 40.0, 'ナイスビューの瀬', 138.99620885423485, 36.68079081338186);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (4, 41.0, '月夜野橋', 138.99610045867362, 36.682398398382205);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (4, 42.0, '徒渡橋ただわたり', 138.9963700365566, 36.678581885488626);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (4, 43.0, '月夜野大橋', 139.00149373024783, 36.669813041488496);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (4, 44.0, 'Fカップの瀬', 139.0023276045115, 36.66932950517871);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (4, 45.0, '道路情報ターミナル 多機能トイレ', 139.0110442726823, 36.664933521722546);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (4, 46.0, '地蔵橋', 139.02778439084656, 36.63972594642);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (4, 47.0, 'ファイナルファンタジーの瀬', 139.0243775898637, 36.64143667107351);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (4, 48.0, '沼田大橋', 139.03070985289065, 36.637617021527035);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (4, 49.0, '鷺石橋', 139.03726765029992, 36.632333244343144);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (4, 50.0, '地蔵橋下プットオン', 139.0282047086552, 36.638934955462005);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (4, 51.0, '第五利根川橋梁', 139.03867495184215, 36.63077991146899);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (4, 52.0, '瀬3', 139.03940878587812, 36.62976319126622);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (4, 53.0, '沼田八景 利根の早瀬', 139.03722507438027, 36.627159401146585);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (4, 54.0, '戸鹿野橋', 139.04116811244825, 36.618815624200664);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (4, 55.0, '久呂保橋', 139.0472740368721, 36.60483745846642);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (4, 56.0, '綾戸ダム', 139.0547895534436, 36.589052832779984);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (4, 57.0, '鉄道橋', 139.0560660244634, 36.58407594931748);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (4, 58.0, '鉄道橋2', 139.05519885148826, 36.584130922346986);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (4, 59.0, '綾戸簗', 139.053945281247, 36.58447965734172);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (4, 60.0, '綾戸橋', 139.04971553622937, 36.581384901678575);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (4, 61.0, '鉄道橋3', 139.052386800509, 36.57382824307513);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (4, 62.0, '鉄道橋4', 139.04111946988132, 36.563923474598965);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (4, 63.0, '赤い水管橋の瀬', 139.0394581253221, 36.5545498667485);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (4, 64.0, '敷島橋', 139.0304686300708, 36.54588397300043);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (4, 65.0, '浅田橋', 139.02803204647716, 36.53504477250246);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (4, 66.0, 'ユートピアの瀬', 139.02678840194574, 36.537389806226926);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (4, 67.0, '宮田橋', 139.0208017114243, 36.526113655256964);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (4, 68.0, '渋川医療センターテイクアウト', 139.01969163625554, 36.51041113985781);

INSERT INTO rivers (name) VALUES ('富士川');
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (5, 1.0, '富士川親水公園駐車場', 138.47188663475953, 35.554366614655166);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (5, 2.0, '富士川大橋1', 138.4768114976785, 35.55613440782284);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (5, 3.0, '鰍沢口駅', 138.47080781594553, 35.54145715738298);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (5, 4.0, '富士川橋1', 138.47322540493846, 35.550299291252585);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (5, 5.0, '戸川', 138.46439487607435, 35.54510191211489);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (5, 6.0, '富士橋', 138.45961623162015, 35.53952615492956);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (5, 7.0, '大柳川流入', 138.4548873789371, 35.515876552258774);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (5, 8.0, '鹿島橋', 138.45861729413375, 35.512603467908775);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (5, 9.0, '月見橋', 138.44959176207533, 35.49637094024685);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (5, 10.0, '甲斐岩間駅', 138.4630850869858, 35.49249027660201);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (5, 11.0, '峡南橋', 138.45485574929825, 35.484738742469574);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (5, 12.0, '富士川橋', 138.44199594412981, 35.46550078152008);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (5, 13.0, '中富橋', 138.4452317525275, 35.45696064399594);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (5, 14.0, '飯富橋', 138.43831307098074, 35.440289832643444);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (5, 15.0, '早川流入', 138.44748161174053, 35.426597286504986);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (5, 16.0, '富山橋', 138.45294904590992, 35.419777126113715);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (5, 17.0, '常葉川流入', 138.45648865167948, 35.41586778803813);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (5, 18.0, '波高島駅', 138.45981789108538, 35.41859861054476);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (5, 19.0, '塩之沢駅', 138.45386024391934, 35.379670175570226);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (5, 20.0, '塩之沢堰', 138.4501427945582, 35.377015810575216);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (5, 21.0, '波木井川', 138.44726062990574, 35.371184455149645);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (5, 22.0, '身延橋', 138.4514580235213, 35.36478176922981);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (5, 23.0, '身延駅', 138.45308096672835, 35.36150773750484);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (5, 24.0, '甲斐大島駅', 138.4521067541609, 35.32823615100236);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (5, 25.0, '富士川大橋', 138.4510173776276, 35.312230879504355);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (5, 26.0, '南部橋', 138.45970208874013, 35.28222816660437);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (5, 27.0, '内船駅', 138.46482856594386, 35.282155835056514);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (5, 28.0, '寄畑駅', 138.47512493411136, 35.264880078200974);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (5, 29.0, '富栄橋', 138.4811168455151, 35.25342631863475);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (5, 30.0, '道の駅とみざわ', 138.48513524467853, 35.24332964682044);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (5, 31.0, '福士川流入', 138.48708300296872, 35.24146405720441);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (5, 32.0, '十島堰', 138.50586565828345, 35.23832710760807);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (5, 33.0, '佐野川流入', 138.5094602546501, 35.237307029828955);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (5, 34.0, '十島駅', 138.51603584673128, 35.23014756796282);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (5, 35.0, '十島の瀬2級', 138.51533999478085, 35.22721859020264);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (5, 36.0, '万栄橋', 138.5180129061753, 35.22779017492714);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (5, 37.0, '稲子駅', 138.5368799914983, 35.23052488521837);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (5, 38.0, '稲子川流入', 138.536906402033, 35.227562006027355);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (5, 39.0, '稲子の瀬', 138.53496335630723, 35.226629842537704);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (5, 40.0, '前釜の瀬3級', 138.55425763925237, 35.204272619196416);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (5, 41.0, '新内房橋', 138.5545031545685, 35.20250720553369);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (5, 42.0, '内房橋', 138.55476557827666, 35.2004031671902);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (5, 43.0, '稲瀬川流入', 138.55340256402835, 35.19715243584423);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (5, 44.0, '釜口の瀬3級', 138.55773492077668, 35.20085939714659);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (5, 45.0, '釜口橋', 138.55809016307515, 35.20045316959596);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (5, 46.0, '芝川流入', 138.56241024946132, 35.19823565334232);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (5, 47.0, '芝川駅', 138.56400535068263, 35.197267893766195);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (5, 48.0, 'デビの瀬2級', 138.5633429201132, 35.19295789367891);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (5, 49.0, 'ポッキズリーブの瀬2級', 138.5652543552868, 35.19087269778038);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (5, 50.0, '富原橋', 138.56997771342915, 35.1922118046794);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (5, 51.0, 'ドラゴンの瀬2級', 138.5759833119094, 35.195005114030636);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (5, 52.0, 'クラッシャーの瀬2級', 138.58244307825944, 35.195509767580134);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (5, 53.0, '沼久保駅', 138.58445290179128, 35.19919045161812);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (5, 54.0, '蓬莱橋', 138.58993334850007, 35.19400042725135);



INSERT INTO rivers (name) VALUES ('阿賀野川');
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (6, 1.0, '湖山荘', 139.13865245035691, 37.13544041743985);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (6, 2.0, '奥只見山荘', 139.13818900179223, 37.134947794321036);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (6, 3.0, '奥只見湖EP', 139.17101255341294, 37.134153898090474);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (6, 4.0, '奥只見湖駐車場', 139.16956629399255, 37.13431510025438);

INSERT INTO rivers (name) VALUES ('本栖湖');
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (7, 1.0, '本栖湖駐車場', 138.60295772590936, 35.46320148088398);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (7, 2.0, '本栖湖EP', 138.60195974446194, 35.4622899625447);


INSERT INTO rivers (name) VALUES ('那珂川');
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (8, 1.0, '烏山大橋手前の堰堤', 140.17254208612061, 36.65290029452072);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (8, 2.0, '烏山駅', 140.15510528663455, 36.65018180982099);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (8, 3.0, '烏山大橋', 140.16907113642833, 36.65188872486162);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (8, 4.0, '下野大橋', 140.16494150804505, 36.63184825763848);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (8, 5.0, '大瀬橋', 140.18929236115102, 36.57257759752949);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (8, 6.0, '大蔵橋', 140.21081109338797, 36.57578521418853);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (8, 7.0, '新那珂川橋', 140.24281410560286, 36.563579084432604);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (8, 8.0, '御前山橋', 140.29696044084136, 36.562241480524136);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (8, 9.0, '那珂川大橋', 140.33157895368552, 36.547033171478134);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (8, 10.0, '那珂川大橋EP', 140.329415301655, 36.54781664038329);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (8, 11.0, '道の駅かつら', 140.33384235049834, 36.54441535914155);
INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude) VALUES (8, 12.0, '道の駅かつらキャンプ場', 140.33293390761156, 36.546102742907166);
