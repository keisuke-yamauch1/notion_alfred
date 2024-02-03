# notion_alfred

## 事前準備

1. NotionのAPIを使えるようにする
   1. [Notionの開発者ページ](https://www.notion.so/my-integrations)にアクセスして、新しいインテグレーションを作成する
   2. Secretsを控えておく
2. Notionのデータベースを作成する
   1. Notionのページにアクセスして、新しいデータベースを作成する
   2. データベースのIDを控えておく
      1. https://www.notion.so/{ワークスペース名}/{データベースID}?{クエリ}
3. AlfredのWorkflowを作成し、以下の環境変数を設定する（右上の[x]こんなマークから設定可能）
   1. `TOKEN`: 1で控えたSecrets
   2. `DATABASE_ID`: 2で控えたデータベースID

## 使い方

例）add_taskの場合

1. リポジトリをクローン
2. `cd add_task`
3. `cargo build --release`
   1. Docker内ではなく、マシン上で実行した方がアーキテクチャの違いでバイナリが実行できないということがなくて楽だと思う
   2. `cargo`がインストールされていない場合は、[Rustの公式ページ](https://www.rust-lang.org/tools/install)等からインストールする
4. AlfredのWorkflowを右クリックして、Open in Terminalをクリック
5. `cp {リポジトリのパス}/target/release/add_task .`を実行
6. Workflowを開いて、右クリック→Input→Keywordで好きなキーワードを設定
7. 右にflowを追加→Actions→Run Scriptを追加
8. Scriptに以下を設定
   ```sh
   query=$1
   ./add_task $query
   ```
9. 完成