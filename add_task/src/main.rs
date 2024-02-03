use reqwest::Client;
use serde_json::json;
use std::env;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = env::var("TOKEN").expect("TOKEN must be set");
    let database_id = env::var("DATABASE_ID").expect("DATABASE_ID must be set");

    let args: Vec<String> = env::args().collect();
    let task_type;
    let task_name;
    if args.len() < 2 {
        eprintln!("タスク名を入力してください");
        std::process::exit(1);
    } else if args.len() == 2 {
        task_type = "";
        task_name = &args[1];
    } else {
        task_type = &args[1];
        task_name = &args[2];
    }

    add_page(&token, &database_id, convert_task_type(task_type), task_name).await?;

    Ok(())
}

async fn add_page(token: &str, database_id: &str, task_type: &str, task_name: &str) -> Result<(), reqwest::Error> {
    let client = Client::new();
    let url = "https://api.notion.com/v1/pages";
    let response = client.post(url)
        .header("Authorization", format!("Bearer {}", token))
        .header("Notion-Version", "2022-06-28")
        .header("Content-Type", "application/json")
        .json(&json!({
            "parent": { "database_id": database_id },
            "properties": {
                "Name": { // Notionデータベースのタイトルプロパティ名に合わせてください
                    "title": [
                        {
                            "text": {
                                "content": task_name
                            }
                        }
                    ]
                },
                "タスク種別": { // Notionデータベースのステータスプロパティ名に合わせてください
                    "select": {
                        "name": task_type
                    }
                }
            }
        }))
        .send()
        .await?;

    println!("Status: {}", response.status());
    println!("Body: {:?}", response.text().await?);

    Ok(())
}

fn convert_task_type(task_type: &str) -> &str {
    match task_type {
        "in" => "infra",
        "ch" => "chore",
        "re" => "review",
        "ex" => "execution",
        "im" => "implementation",
        "de" => "design",
        _ => "no set",
    }
}
