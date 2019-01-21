pub mod foods {
  use std::collections::HashMap;

  pub fn get(s:&str) -> String {
    let mut f = HashMap::new();
    f.insert("小松菜".to_string(), "<speak>はい。小松菜は、<prosody volume=loud>あげても大丈夫です。</prosody>カルシウムも豊富でおススメの食べ物です。ではまた。</speak>");
    f.insert("水菜".to_string(), "<speak>はい。水菜は、<prosody volume=loud>あげても大丈夫です。</prosody>カルシウムも豊富でおススメの食べ物です。ではまた。</speak>");
    f.insert("チンゲンサイ".to_string(), "<speak>はい。チンゲンサイは、<prosody volume=loud>あげても大丈夫です。</prosody>カルシウムも豊富でおススメの食べ物です。ではまた。</speak>");
    f.insert("大根の葉".to_string(), "<speak>はい。大根の葉は、<prosody volume=loud>あげても大丈夫です。</prosody>カルシウムも豊富でおススメの食べ物です。ではまた。</speak>");
    f.insert("かぶの葉".to_string(), "<speak>はい。かぶの葉は、<prosody volume=loud>あげても大丈夫です。</prosody>カルシウムも豊富でおススメの食べ物です。ではまた。</speak>");
    f.insert("モロヘイヤ".to_string(), "<speak>はい。モロヘイヤは、<prosody volume=loud>あげても大丈夫です。</prosody>カルシウムも豊富でおススメの食べ物です。ではまた。</speak>");
    f.insert("サラダ菜".to_string(), "<speak>はい。サラダ菜は、<prosody volume=loud>あげても大丈夫です。</prosody>カルシウムも豊富でおススメの食べ物です。ではまた。</speak>");
    f.insert("アシタバ".to_string(), "<speak>はい。アシタバは、<prosody volume=loud>あげても大丈夫です。</prosody>カルシウムも豊富でおススメの食べ物です。ではまた。</speak>");
    f.insert("高菜".to_string(), "<speak>はい。高菜は、<prosody volume=loud>あげても大丈夫です。</prosody>カルシウムも豊富でおススメの食べ物です。ではまた。</speak>");
    f.insert("シソ".to_string(), "<speak>はい。シソは、<prosody volume=loud>あげても大丈夫です。</prosody>カルシウムも豊富でおススメの食べ物です。ではまた。</speak>");
    f.insert("ツルムラサキ".to_string(), "<speak>はい。ツルムラサキは、<prosody volume=loud>あげても大丈夫です。</prosody>カルシウムも豊富でおススメの食べ物です。ではまた。</speak>");
    f.insert("オオバコ".to_string(), "<speak>はい。オオバコは、<prosody volume=loud>あげても大丈夫です。</prosody>カルシウムも豊富でおススメの食べ物です。ではまた。</speak>");
    f.insert("ナズナ".to_string(), "<speak>はい。ナズナは、<prosody volume=loud>あげても大丈夫です。</prosody>カルシウムも豊富でおススメの食べ物です。ではまた。</speak>");
    f.insert("アカツメクサ".to_string(), "<speak>はい。アカツメクサは、<prosody volume=loud>あげても大丈夫です。</prosody>カルシウムも豊富でおススメの食べ物です。ではまた。</speak>");
    f.insert("タンポポ".to_string(), "<speak>はい。タンポポは、<prosody volume=loud>あげても大丈夫です。</prosody>カルシウムも豊富でおススメの食べ物です。ではまた。</speak>");
    f.insert("枝豆".to_string(), "<speak>はい。枝豆は、<prosody volume=loud>あげても大丈夫です。</prosody>カルシウムも豊富でおススメの食べ物です。ではまた。</speak>");
    f.insert("つりがね人参".to_string(), "<speak>はい。つりがね人参は、<prosody volume=loud>あげても大丈夫です。</prosody>カルシウムも豊富でおススメの食べ物です。ではまた。</speak>");
    f.insert("ウチワサボテン".to_string(), "<speak>はい。ウチワサボテンは、<prosody volume=loud>あげても大丈夫です。</prosody>カルシウムも豊富でおススメの食べ物です。ではまた。</speak>");
    f.insert("アロエ".to_string(), "<speak>はい。アロエは、<prosody volume=loud>あげても大丈夫です。</prosody>カルシウムも豊富でおススメの食べ物です。ではまた。</speak>");
    f.insert("ハイビスカス".to_string(), "<speak>はい。ハイビスカスは、<prosody volume=loud>あげても大丈夫です。</prosody>カルシウムも豊富でおススメの食べ物です。ではまた。</speak>");
    f.insert("京菜".to_string(), "<speak>はい。京菜は、<prosody volume=loud>あげても大丈夫です。</prosody>カルシウムも豊富でおススメの食べ物です。ではまた。</speak>");
    f.insert("タアサイ".to_string(), "<speak>はい。タアサイは、<prosody volume=loud>あげても大丈夫です。</prosody>カルシウムも豊富でおススメの食べ物です。ではまた。</speak>");
    f.insert("野沢菜".to_string(), "<speak>はい。野沢菜は、<prosody volume=loud>あげても大丈夫です。</prosody>カルシウムも豊富でおススメの食べ物です。ではまた。</speak>");
    f.insert("ヨモギ".to_string(), "<speak>はい。ヨモギは、<prosody volume=loud>あげても大丈夫です。</prosody>カルシウムも豊富でおススメの食べ物です。ではまた。</speak>");
    f.insert("広島菜".to_string(), "<speak>はい。広島菜は、<prosody volume=loud>あげても大丈夫です。</prosody>カルシウムも豊富でおススメの食べ物です。ではまた。</speak>");
    f.insert("クワの葉".to_string(), "<speak>はい。クワの葉は、<prosody volume=loud>あげても大丈夫です。</prosody>カルシウムは少なめです。ではまた。</speak>");
    f.insert("サツマイモの葉".to_string(), "<speak>はい。サツマイモの葉は、<prosody volume=loud>あげても大丈夫です。</prosody>カルシウムは少なめです。ではまた。</speak>");
    f.insert("アルファルファ".to_string(), "<speak>はい。アルファルファは、<prosody volume=loud>あげても大丈夫です。</prosody>カルシウムは少なめです。ではまた。</speak>");
    f.insert("ルッコラ".to_string(), "<speak>はい。ルッコラは、<prosody volume=loud>あげても大丈夫です。</prosody>カルシウムは少なめです。ではまた。</speak>");
    f.insert("カイワレ".to_string(), "<speak>はい。カイワレは、<prosody volume=loud>あげても大丈夫です。</prosody>カルシウムは少なめです。ではまた。</speak>");
    f.insert("パセリ".to_string(), "<speak>はい。パセリは、<prosody volume=loud>あげても大丈夫です。</prosody>カルシウムは少なめです。ではまた。</speak>");
    f.insert("ゴーヤの葉".to_string(), "<speak>はい。ゴーヤの葉は、<prosody volume=loud>あげても大丈夫です。</prosody>カルシウムは少なめです。ではまた。</speak>");
    f.insert("白菜".to_string(), "<speak>はい。白菜は、<prosody volume=loud>あげても大丈夫です。</prosody>カルシウムは少なめです。ではまた。</speak>");
    f.insert("レタス".to_string(), "<speak>はい。レタスは、<prosody volume=loud>あげても大丈夫です。</prosody>カルシウムは少なめです。ではまた。</speak>");
    f.insert("セロリ".to_string(), "<speak>はい。セロリは、<prosody volume=loud>あげても大丈夫です。</prosody>カルシウムは少なめです。ではまた。</speak>");
    f.insert("ピーマン".to_string(), "<speak>はい。ピーマンは、<prosody volume=loud>あげても大丈夫です。</prosody>カルシウムはあまりありません。ではまた。</speak>");
    f.insert("にんじん".to_string(), "<speak>はい。にんじんは、<prosody volume=loud>あげても大丈夫です。</prosody>カルシウムは少なめです。ではまた。</speak>");
    f.insert("かぼちゃ".to_string(), "<speak>はい。かぼちゃは、<prosody volume=loud>あげても大丈夫です。</prosody>カルシウムは少なめです。ではまた。</speak>");
    f.insert("オクラ".to_string(), "<speak>はい。オクラは、<prosody volume=loud>あげても大丈夫です。</prosody>カルシウムは少なめです。ではまた。</speak>");
    f.insert("きゅうり".to_string(), "<speak>はい。きゅうりは、<prosody volume=loud>あげても大丈夫です。</prosody>カルシウムは少なめです。ではまた。</speak>");
    f.insert("トマト".to_string(), "<speak>はい。トマトは、<prosody volume=loud>あげても大丈夫です。</prosody>カルシウムは少なめです。ではまた。</speak>");
    f.insert("ミニトマト".to_string(), "<speak>はい。ミニトマトは、<prosody volume=loud>あげても大丈夫です。</prosody>カルシウムは少なめです。ではまた。</speak>");
    f.insert("ポーラチュカ".to_string(), "<speak>はい。ポーラチュカは、<prosody volume=loud>あげても大丈夫です。</prosody>ではまた。</speak>");
    f.insert("菜の花".to_string(), "<speak>はい。菜の花は、<prosody volume=loud>あげても大丈夫です。</prosody>ではまた。</speak>");
    f.insert("レンゲ".to_string(), "<speak>はい。レンゲは、<prosody volume=loud>あげても大丈夫です。</prosody>ではまた。</speak>");
    f.insert("クズ".to_string(), "<speak>はい。クズは、<prosody volume=loud>あげても大丈夫です。</prosody>ではまた。</speak>");
    f.insert("ホトケノザ".to_string(), "<speak>はい。ホトケノザは、<prosody volume=loud>あげても大丈夫です。</prosody>ではまた。</speak>");
    f.insert("ハコベ".to_string(), "<speak>はい。ハコベは、<prosody volume=loud>あげても大丈夫です。</prosody>ではまた。</speak>");
    f.insert("ヒメオドリコソウ".to_string(), "<speak>はい。ヒメオドリコソウは、<prosody volume=loud>あげても大丈夫です。</prosody>ではまた。</speak>");
    f.insert("コバンソウ".to_string(), "<speak>はい。コバンソウは、<prosody volume=loud>あげても大丈夫です。</prosody>ではまた。</speak>");
    f.insert("エノコログサ".to_string(), "<speak>はい。エノコログサは、<prosody volume=loud>あげても大丈夫です。</prosody>ではまた。</speak>");
    f.insert("ツユクサ".to_string(), "<speak>はい。ツユクサは、<prosody volume=loud>あげても大丈夫です。</prosody>ではまた。</speak>");
    f.insert("ヘビイチゴ".to_string(), "<speak>はい。ヘビイチゴは、<prosody volume=loud>あげても大丈夫です。</prosody>ではまた。</speak>");
    f.insert("ホウレンソウ".to_string(), "<speak>はい。ホウレンソウは、<prosody volume=loud>少しならあげても大丈夫です。</prosody>ではまた。</speak>");
    f.insert("キャベツ".to_string(), "<speak>はい。キャベツは、<prosody volume=loud>少しならあげても大丈夫です。</prosody>ではまた。</speak>");
    f.insert("ブロッコリー".to_string(), "<speak>はい。ブロッコリーは、<prosody volume=loud>少しならあげても大丈夫です。</prosody>ではまた。</speak>");
    f.insert("リンゴ".to_string(), "<speak>はい。リンゴは、<prosody volume=loud>少しならあげても大丈夫です。</prosody>ではまた。</speak>");
    f.insert("バナナ".to_string(), "<speak>はい。バナナは、<prosody volume=loud>少しならあげても大丈夫です。</prosody>ではまた。</speak>");
    f.insert("キウイ".to_string(), "<speak>はい。キウイは、<prosody volume=loud>少しならあげても大丈夫です。</prosody>ではまた。</speak>");
    f.insert("みかん".to_string(), "<speak>はい。みかんは、<prosody volume=loud>少しならあげても大丈夫です。</prosody>ではまた。</speak>");
    f.insert("パパイヤ".to_string(), "<speak>はい。パパイヤは、<prosody volume=loud>少しならあげても大丈夫です。</prosody>ではまた。</speak>");
    f.insert("マンゴー".to_string(), "<speak>はい。マンゴーは、<prosody volume=loud>少しならあげても大丈夫です。</prosody>ではまた。</speak>");
    f.insert("いちご".to_string(), "<speak>はい。いちごは、<prosody volume=loud>少しならあげても大丈夫です。</prosody>ではまた。</speak>");
    f.insert("ブルーベリー".to_string(), "<speak>はい。ブルーベリーは、<prosody volume=loud>少しならあげても大丈夫です。</prosody>ではまた。</speak>");
    f.insert("ブラックベリー".to_string(), "<speak>はい。ブラックベリーは、<prosody volume=loud>少しならあげても大丈夫です。</prosody>ではまた。</speak>");
    f.insert("かき".to_string(), "<speak>はい。かきは、<prosody volume=loud>少しならあげても大丈夫です。</prosody>ではまた。</speak>");
    f.insert("すいか".to_string(), "<speak>はい。すいかは、<prosody volume=loud>少しならあげても大丈夫です。</prosody>ではまた。</speak>");
    f.insert("さくらんぼ".to_string(), "<speak>はい。さくらんぼは、<prosody volume=loud>少しならあげても大丈夫です。</prosody>ではまた。</speak>");
    f.insert("なし".to_string(), "<speak>はい。なしは、<prosody volume=loud>少しならあげても大丈夫です。</prosody>ではまた。</speak>");
    f.insert("ぶどう".to_string(), "<speak>はい。ぶどうは、<prosody volume=loud>少しならあげても大丈夫です。</prosody>ではまた。</speak>");
    f.insert("メロン".to_string(), "<speak>はい。メロンは、<prosody volume=loud>少しならあげても大丈夫です。</prosody>ではまた。</speak>");
    f.insert("もも".to_string(), "<speak>はい。ももは、<prosody volume=loud>少しならあげても大丈夫です。</prosody>ではまた。</speak>");
    f.insert("大葉".to_string(), "<speak>はい。大葉は、<prosody volume=loud>あげても大丈夫です。</prosody>ではまた。</speak>");	
    f.insert("びわ".to_string(), "<speak>はい。びわは、<prosody volume=loud>少しならあげても大丈夫です。</prosody>ではまた。</speak>");
    f.insert("アサ".to_string(), "<speak>はい。アサは、<prosody volume=loud>あげてはいけない食べ物です！</prosody>注意が必要です。ではまた。</speak>");
    f.insert("イチジク".to_string(), "<speak>はい。イチジクは、<prosody volume=loud>あげてはいけない食べ物です！</prosody>注意が必要です。ではまた。</speak>");
    f.insert("アジサイ".to_string(), "<speak>はい。アジサイは、<prosody volume=loud>あげてはいけない食べ物です！</prosody>注意が必要です。ではまた。</speak>");
    f.insert("アセビ".to_string(), "<speak>はい。アセビは、<prosody volume=loud>あげてはいけない食べ物です！</prosody>注意が必要です。ではまた。</speak>");
    f.insert("イチイ".to_string(), "<speak>はい。イチイは、<prosody volume=loud>あげてはいけない食べ物です！</prosody>注意が必要です。ではまた。</speak>");
    f.insert("イラクサ".to_string(), "<speak>はい。イラクサは、<prosody volume=loud>あげてはいけない食べ物です！</prosody>注意が必要です。ではまた。</speak>");
    f.insert("フクジュソウ".to_string(), "<speak>はい。フクジュソウは、<prosody volume=loud>あげてはいけない食べ物です！</prosody>注意が必要です。ではまた。</speak>");
    f.insert("スズラン".to_string(), "<speak>はい。スズランは、<prosody volume=loud>あげてはいけない食べ物です！</prosody>注意が必要です。ではまた。</speak>");
    f.insert("ヒガンバナ".to_string(), "<speak>はい。ヒガンバナは、<prosody volume=loud>あげてはいけない食べ物です！</prosody>注意が必要です。ではまた。</speak>");
    f.insert("ワラビ".to_string(), "<speak>はい。ワラビは、<prosody volume=loud>あげてはいけない食べ物です！</prosody>注意が必要です。ではまた。</speak>");
    f.insert("ヨウシュヤマゴボウ".to_string(), "<speak>はい。ヨウシュヤマゴボウは、<prosody volume=loud>あげてはいけない食べ物です！</prosody>注意が必要です。ではまた。</speak>");
    f.insert("ポインセチア".to_string(), "<speak>はい。ポインセチアは、<prosody volume=loud>あげてはいけない食べ物です！</prosody>注意が必要です。ではまた。</speak>");
    f.insert("アマリリス".to_string(), "<speak>はい。アマリリスは、<prosody volume=loud>あげてはいけない食べ物です！</prosody>注意が必要です。ではまた。</speak>");
    f.insert("オシロイバナ".to_string(), "<speak>はい。オシロイバナは、<prosody volume=loud>あげてはいけない食べ物です！</prosody>注意が必要です。ではまた。</speak>");
    f.insert("ヒヤシンス".to_string(), "<speak>はい。ヒヤシンスは、<prosody volume=loud>あげてはいけない食べ物です！</prosody>注意が必要です。ではまた。</speak>");
    f.insert("カラー".to_string(), "<speak>はい。カラーは、<prosody volume=loud>あげてはいけない食べ物です！</prosody>注意が必要です。ではまた。</speak>");
    f.insert("クリスマスローズ".to_string(), "<speak>はい。クリスマスローズは、<prosody volume=loud>あげてはいけない食べ物です！</prosody>注意が必要です。ではまた。</speak>");
    f.insert("ラン".to_string(), "<speak>はい。ランは、<prosody volume=loud>あげてはいけない食べ物です！</prosody>注意が必要です。ではまた。</speak>");
    f.insert("ニセアカシア".to_string(), "<speak>はい。ニセアカシアは、<prosody volume=loud>あげてはいけない食べ物です！</prosody>注意が必要です。ではまた。</speak>");
    f.insert("トリカブト".to_string(), "<speak>はい。トリカブトは、<prosody volume=loud>あげてはいけない食べ物です！</prosody>注意が必要です。ではまた。</speak>");
    f.insert("トマトの葉".to_string(), "<speak>はい。トマトの葉は、<prosody volume=loud>あげてはいけない食べ物です！</prosody>注意が必要です。ではまた。</speak>");
    f.insert("じゃがいもの葉".to_string(), "<speak>はい。じゃがいもの葉は、<prosody volume=loud>あげてはいけない食べ物です！</prosody>注意が必要です。ではまた。</speak>");
    return format!("{}", f.get(s).as_ref().unwrap());
  }
}
