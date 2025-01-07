use reqwest::Client;
    use regex::Regex;

    pub async fn is_downloadable(client: &Client, api_key: &str, file_id: &str, file_type: &str) -> bool {
      let url = format!("https://www.googleapis.com/drive/v3/files/{}?fields=mimeType&key={}", file_id, api_key);

      let resp = client.get(&url)
        .send()
        .await;

      match resp {
        Ok(response) => {
          if response.status().is_success() {
            let json: serde_json::Value = response.json().await.unwrap_or_default();
            let mime_type = json.get("mimeType").and_then(|v| v.as_str()).unwrap_or("");
            match_file_type(mime_type, file_type)
          } else {
            eprintln!("Error checking file: {:?}", response.text().await);
            false
          }
        }
        Err(e) => {
          eprintln!("Error checking file: {:?}", e);
          false
        }
      }
    }

    pub async fn check_folder(client: &Client, api_key: &str, folder_id: &str, file_type: &str) -> bool {
      let url = format!("https://www.googleapis.com/drive/v3/files?q='{}'+in+parents&fields=files(mimeType)&key={}", folder_id, api_key);

      let resp = client.get(&url)
        .send()
        .await;

      match resp {
        Ok(response) => {
          if response.status().is_success() {
            let json: serde_json::Value = response.json().await.unwrap_or_default();
            if let Some(files) = json.get("files").and_then(|v| v.as_array()) {
              for file in files {
                if let Some(mime_type) = file.get("mimeType").and_then(|v| v.as_str()) {
                  if match_file_type(mime_type, file_type) {
                    return true;
                  }
                }
              }
            }
            false
          } else {
            eprintln!("Error checking folder: {:?}", response.text().await);
            false
          }
        }
        Err(e) => {
          eprintln!("Error checking folder: {:?}", e);
          false
        }
      }
    }

    pub fn match_file_type(mime_type: &str, file_type: &str) -> bool {
      match file_type {
        "pdf" => mime_type == "application/pdf",
        "image" => mime_type.starts_with("image/"),
        "video" => mime_type.starts_with("video/"),
        _ => false,
      }
    }

    pub fn extract_id_from_link(link: &str) -> String {
      let file_id_regex = Regex::new(r"/d/([a-zA-Z0-9-_]+)").unwrap();
      let folder_id_regex = Regex::new(r"/folders/([a-zA-Z0-9-_]+)").unwrap();

      if let Some(caps) = file_id_regex.captures(link) {
        caps.get(1).map_or(String::new(), |m| m.as_str().to_string())
      } else if let Some(caps) = folder_id_regex.captures(link) {
        caps.get(1).map_or(String::new(), |m| m.as_str().to_string())
      } else {
        String::new()
      }
    }
