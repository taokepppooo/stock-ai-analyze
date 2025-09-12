use tauri::AppHandle;

pub struct DatabaseConfig {
  pub name: String,
  pub url: String,
  #[allow(dead_code)]
  pub description: String,
}

fn get_project_db_path(db_name: &str) -> String {
  let current_dir = std::env::current_dir()
    .unwrap_or_else(|_| std::path::PathBuf::from("."));
  let project_root: std::path::PathBuf = current_dir
    .parent() // 从 src-tauri 目录回到项目根目录
    .unwrap_or_else(|| std::path::Path::new("."))
    .join("db")
    .join(db_name);
  
  format!("sqlite:{}", project_root.to_string_lossy())
}

fn get_database_configs() -> Vec<DatabaseConfig> {
  vec![
    DatabaseConfig {
      name: "stock_data".to_string(),
      url: get_project_db_path("stock_data.db"),
      description: "股票数据库".to_string(),
    },
  ]
}

// 公共函数：初始化所有数据库
pub async fn initialize_all_databases(app: &AppHandle) -> Result<String, String> {
  println!("开始初始化所有数据库...");
  
  let configs = get_database_configs();
  let mut results = Vec::new();
  
  for config in &configs {
    match initialize_single_database(app, config).await {
      Ok(msg) => {
        println!("✅ {}", msg);
        results.push(format!("✅ {}: 成功", config.name));
      }
      Err(e) => {
        println!("❌ {}: {}", config.name, e);
        results.push(format!("❌ {}: {}", config.name, e));
      }
    }
  }
  
  Ok(format!("数据库初始化完成:\n{}", results.join("\n")))
}

async fn initialize_single_database(
  _app: &AppHandle,
  config: &DatabaseConfig,
) -> Result<String, String> {
  let db_name = &config.name;
  let db_url = &config.url;

  println!("初始化数据库: {} ({})", db_name, db_url);

  // 从 sqlite:path 格式中提取实际的文件路径
  let db_path = db_url.strip_prefix("sqlite:").unwrap_or(db_url);
  
  // 确保数据库目录存在
  if let Some(parent) = std::path::Path::new(db_path).parent() {
    std::fs::create_dir_all(parent)
      .map_err(|e| format!("创建数据库目录失败: {}", e))?;
  }

  // 使用 rusqlite 直接连接并创建表
  match rusqlite::Connection::open(db_path) {
    Ok(conn) => {
      // 执行建表 SQL
      let create_table_sql = r#"
        CREATE TABLE IF NOT EXISTS stocks (
          id INTEGER PRIMARY KEY AUTOINCREMENT,
          symbol TEXT NOT NULL,
          name TEXT,
          market TEXT,
          sector TEXT,
          industry TEXT,
          UNIQUE(symbol)
        );
      "#;
      
      match conn.execute(create_table_sql, []) {
        Ok(_) => {
          println!("✅ 数据库表创建成功: {} -> {}", db_name, db_path);
          Ok(format!("数据库 {} 初始化成功，表已创建在: {}", db_name, db_path))
        }
        Err(e) => {
          println!("❌ 创建表失败: {}", e);
          Err(format!("创建表失败: {}", e))
        }
      }
    }
    Err(e) => {
      println!("❌ 连接数据库失败: {}", e);
      Err(format!("连接数据库失败: {}", e))
    }
  }
}
