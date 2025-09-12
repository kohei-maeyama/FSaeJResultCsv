import os
import argparse
import re
import sys

def main():
    # argparseでコマンドライン引数を定義
    parser = argparse.ArgumentParser(description="Convert CSV files from Shift-JIS to UTF-8 or vice versa.")
    parser.add_argument('-i', '--inverse', action='store_true', help="Convert from UTF-8 to Shift-JIS")
    args = parser.parse_args()

    convert_to_shift_jis = args.inverse

    # 現在のディレクトリを取得
    current_dir = os.getcwd()

    # 作業リストを取得
    work_list = get_work_list(current_dir)

    if not work_list:
        print("No matching folders with CSV files.")
        return

    for work in work_list:
        for csv_path_list in work.values():
            for csv_path in csv_path_list:
                convert_csv_file(csv_path, convert_to_shift_jis)


# CSVファイルの文字コード変換関数
def convert_csv_file(path, to_shift_jis):
    # ファイルを読み込む
    with open(path, 'rb') as input_file:
        buffer = input_file.read()

    if to_shift_jis:
        try:
            # UTF-8からShift-JISに変換
            tmp = buffer.decode('utf-8')
        except UnicodeDecodeError as _:
            print("Conversion was skipped. Current encoding is shift-jis.")
            sys.exit(1)
        output_data = tmp.encode('shift_jis')
    else:
        try:
            # Shift-JISからUTF-8に変換
            tmp = buffer.decode('shift_jis')
        except Exception as _:
            print("Conversion was skipped. Current encoding is utf-8.")
            sys.exit(1)
        output_data = tmp.encode('utf-8')

    # 元のファイルを上書き
    with open(path, 'wb') as output_file:
        output_file.write(output_data)

    print(f"Converted file: {path} to {'Shift-JIS' if to_shift_jis else 'UTF-8'}")


# 特定の正規表現にマッチするディレクトリ内のCSVファイルリストを取得
def get_work_list(base_dir):
    work_list = []

    # フォルダ名が ^20[0-9]{2}$ にマッチする正規表現
    re_pattern = re.compile(r'^20[0-9]{2}')

    # base_dir の直下にあるフォルダを列挙
    for entry in os.scandir(base_dir):
        if entry.is_dir():
            folder_name = os.path.basename(entry.path)
            if re_pattern.match(folder_name):
                csv_path_list = []
                
                # フォルダ内のファイルを列挙
                for sub_entry in os.scandir(entry.path):
                    if sub_entry.is_file() and sub_entry.path.endswith('.csv'):
                        csv_path_list.append(sub_entry.path)
                
                if csv_path_list:
                    work_list.append({folder_name: csv_path_list})

    return work_list if work_list else None


if __name__ == "__main__":
    main()
