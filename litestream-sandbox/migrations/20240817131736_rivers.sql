-- Add migration script here

CREATE TABLE rivers (
   river_id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
   river_name TEXT NOT NULL,
   created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
   updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);
INSERT INTO rivers (river_name) VALUES ('荒川長瀞');
INSERT INTO rivers (river_name) VALUES ('多摩川');
INSERT INTO rivers (river_name) VALUES ('久慈川');
INSERT INTO rivers (river_name) VALUES ('利根川');
INSERT INTO rivers (river_name) VALUES ('富士川');
INSERT INTO rivers (river_name) VALUES ('那珂川');

CREATE TABLE river_paths (
    ancestor INTEGER NOT NULL,
    descendant INTEGER NOT NULL,
    PRIMARY KEY (ancestor, descendant),
    FOREIGN KEY (ancestor) REFERENCES rivers(river_id),
    FOREIGN KEY (descendant) REFERENCES rivers(river_id)
);
INSERT INTO river_paths (ancestor, descendant) VALUES (1, 1);
INSERT INTO river_paths (ancestor, descendant) VALUES (2, 2);
INSERT INTO river_paths (ancestor, descendant) VALUES (3, 3);
INSERT INTO river_paths (ancestor, descendant) VALUES (4, 4);
INSERT INTO river_paths (ancestor, descendant) VALUES (5, 5);
INSERT INTO river_paths (ancestor, descendant) VALUES (6, 6);

CREATE TABLE river_waypoints (
  river_waypoints_id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
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
