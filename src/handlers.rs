use actix_web::{post, web, HttpResponse, Responder};
    use serde::Deserialize;
    use serde_json::json;
    use std::env;
    use reqwest::Client;

    use crate::utils::{extract_id_from_link, is_downloadable, check_folder};

    #[derive(Deserialize)]
    pub struct CheckRequest {
      link: String,
      r#type: String,
    }

    #[post("/check-downloadable")]
    pub async fn check_downloadable(req_body: web::Json<CheckRequest>) -> impl Responder {
      let api_key = env::var("GOOGLE_DRIVE_API_KEY").unwrap_or_default();
      let client = Client::builder().build().unwrap();

      let link = &req_body.link;
      let file_type = &req_body.r#type;

      let id = extract_id_from_link(link);
      if id.is_empty() {
        return HttpResponse::BadRequest().body("Invalid link format");
      }

      let is_folder_link = link.contains("/folders/");
      let downloadable = if is_folder_link {
        check_folder(&client, &api_key, &id, &file_type).await
      } else {
        is_downloadable(&client, &api_key, &id, &file_type).await
      };

      if downloadable {
        HttpResponse::Ok().json(json!({ "result": "yes" }))
      } else {
        HttpResponse::Ok().json(json!({ "result": "no" }))
      }
    }
