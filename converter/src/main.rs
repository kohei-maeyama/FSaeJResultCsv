use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use clap::{Arg, Command};
use encoding_rs::*;
use encoding_rs_io::DecodeReaderBytesBuilder;
use regex::Regex;

fn main() -> io::Result<()> {
    // clapでコマンドライン引数を定義
    let matches = Command::new("csv_converter")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("Convert CSV files from Shift-JIS to UTF-8")
        .arg(
            Arg::new("inverse")
                .short('i')
                .long("inverse")
                .num_args(0)
                .help("Convert from UTF-8 to Shift-JIS"),
        )
        .get_matches();

    // コマンドライン引数を取得
    let convert_to_shiftjis = matches.get_one::<bool>("inverse").unwrap().clone();

    // 現在のディレクトリを取得
    let current_dir = std::env::current_dir()?;

    // cargo runで実行するときは次の行のコメントアウトを解除する ユーザー向けにはコメントアウトする
    //let current_dir = current_dir.join("..");
    let work_list =get_work_list(&current_dir)?;

    for work in work_list {
        for csv_path_list in work.values() {
            for csv_path in csv_path_list {
                convert_csv_file(&csv_path.as_path(), convert_to_shiftjis)?;
            }
        }
    }

    Ok(())
}

// CSVファイルの文字コード変換関数
fn convert_csv_file(path: &Path, to_shiftjis: bool) -> io::Result<()> {
    // ファイルを読み込む
    let mut input_file = File::open(path)?;

    let mut buffer = Vec::new();
    input_file.read_to_end(&mut buffer)?;

    let output_data = if to_shiftjis {
        let tmp = String::from_utf8_lossy(&buffer);
        // UTF-8からShift-JISに変換
        let (encoding, _, _) = SHIFT_JIS.encode(&tmp);
        encoding.into_owned()
    } else {
        // Shift-JISからUTF-8に変換
        let mut decoder = DecodeReaderBytesBuilder::new()
            .encoding(Some(SHIFT_JIS))
            .build(buffer.as_slice());
        let mut output = String::new();
        decoder.read_to_string(&mut output)?;
        output.into_bytes()
    };

    // 元のファイルを上書き
    let mut output_file = File::create(path)?;
    output_file.write_all(&output_data)?;

    println!(
        "Converted file: {} to {}",
        path.display(),
        if to_shiftjis { "Shift-JIS" } else { "UTF-8" }
    );

    Ok(())
}


fn get_work_list(base_dir: &PathBuf) -> io::Result<Vec<HashMap<String, Vec<PathBuf>>>> {
    let mut ret = Vec::new();
    
    // フォルダ名が ^20[0-9]{2}$ にマッチする正規表現
    let re = Regex::new(r"^20[0-9]{2}$").unwrap();

    // base_dir の直下にあるフォルダを列挙
    for entry in fs::read_dir(base_dir)? {
        let entry = entry?;
        let path = entry.path();

        let mut folder_works = HashMap::new();

        // ディレクトリかどうかチェック
        if path.is_dir() {
            // フォルダ名が ^20[0-9]{2}$ にマッチするかどうか
            if let Some(folder_name) = path.file_name().and_then(|n| n.to_str()) {
                if re.is_match(folder_name) {

                    let mut csv_path_list = Vec::<std::path::PathBuf>::new();
                    
                    // フォルダ内のファイルを列挙
                    for sub_entry in fs::read_dir(&path)? {
                        let sub_entry = sub_entry?;
                        let sub_path = sub_entry.path();
                        
                        // 拡張子が .csv のファイルを列挙
                        if sub_path.is_file() {
                            if let Some(extension) = sub_path.extension() {
                                if extension == "csv" {
                                    csv_path_list.push(sub_path);
                                }
                            }
                        }
                    }

                    folder_works.insert(String::from(folder_name), csv_path_list);
                }
            }
        }

        if !folder_works.is_empty() {
            ret.push(folder_works);
        }

    }

    // ret が空かどうかを判定し、空なら Err、そうでなければ Ok(ret)
    if ret.is_empty() {
        Err(io::Error::new(io::ErrorKind::NotFound, "No matching folders with CSV files found. Check location of this program."))
    } else {
        Ok(ret)
    }
}