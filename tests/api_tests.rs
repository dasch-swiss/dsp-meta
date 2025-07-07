#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::sync::Arc;

    use api::handler::v1::projects::responses::ProjectMetadataWithInfoDto;
    use axum::http::StatusCode;
    use axum_test::TestServer;
    use chrono::NaiveDate;
    use dsp_meta::api;
    use dsp_meta::app_state::AppState;
    use dsp_meta::domain::metadata_repository::MetadataRepository;
    use dsp_meta::domain::metadata_service::MetadataService;
    use dsp_meta::domain::model::draft_model::{
        DraftDate, DraftMetadata, DraftProject, DraftProjectStatus, DraftText, DraftTextOrUrl,
        Shortcode,
    };
    use fake::{Fake, Faker};
    use nonempty::NonEmpty;
    use serde_json::Value;
    use url::Url;

    fn build_server(data: Vec<DraftMetadata>) -> TestServer {
        let meta_repo = MetadataRepository::for_test(data);
        let shared_state = Arc::new(AppState {
            metadata_service: MetadataService::new(meta_repo),
            public_dir: "/public".to_string(),
            version: "foo_version",
            base_url: Url::parse("http://localhost:3000").expect("Valid url"),
        });
        let app = api::router::router(shared_state);
        TestServer::new(app).unwrap()
    }

    #[tokio::test]
    async fn server_should_get_version_txt() {
        let server = build_server(vec![]);

        let response = server.get("/version.txt").await;

        assert_eq!(response.text(), "foo_version");
        assert_eq!(response.status_code(), StatusCode::OK);
    }

    #[tokio::test]
    async fn server_should_get_the_robots_txt() {
        let server = build_server(vec![]);

        let response = server.get("/robots.txt").await;

        assert_eq!(response.status_code(), StatusCode::OK);
        assert_eq!(
            response.text(),
            "Sitemap: http://localhost:3000/sitemap.xml\nUser-agent: *\nDisallow:\n"
        );
        assert_eq!(
            response
                .headers()
                .get("Content-Type")
                .map(|v| v.to_str().unwrap()),
            Some("text/plain")
        );
    }

    #[tokio::test]
    async fn server_should_get_the_sitemap_xml() {
        let server = build_server(vec![]);

        let response = server.get("/sitemap.xml").await;

        assert_eq!(response.status_code(), StatusCode::OK);
        assert!(response.text().contains("xml"));
        assert_eq!(
            response
                .headers()
                .get("Content-Type")
                .map(|v| v.to_str().unwrap()),
            Some("application/xml")
        );
    }

    #[tokio::test]
    async fn projects_should_return_404_for_an_unknown_project() {
        let server = build_server(vec![]);

        let response = server.get("/api/v1/projects/0001").await;

        assert_eq!(response.status_code(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn projects_shortcode_should_return_200_for_an_known_project() {
        let expected = test_data("0001");
        let server = build_server(vec![expected.clone()]);

        let response = server.get("/api/v1/projects/0001").await;

        let actual: DraftMetadata = serde_json::from_str(&response.text()).unwrap();
        assert_eq!(response.status_code(), StatusCode::OK);
        assert_eq!(actual, expected);
    }

    #[tokio::test]
    async fn projects_should_return_empty_array_with_no_data() {
        let server = build_server(vec![]);

        let response = server.get("/api/v1/projects").await;

        assert_eq!(response.status_code(), StatusCode::OK);

        let resp_txt = response.text();
        assert!(resp_txt == "[]");
        let actual: Value = serde_json::from_str(&resp_txt).unwrap();
        if let Value::Array(arr) = actual {
            assert_eq!(arr.len(), 0);
        } else {
            panic!("Expected an array");
        }
        assert_eq!(response.status_code(), StatusCode::OK);
    }

    #[tokio::test]
    async fn projects_should_return_empty_array_with_no_data_using_pagination() {
        let server = build_server(vec![]);

        let response = server.get("/api/v1/projects?_page=1&_limit=10").await;

        let resp_txt = response.text();
        let actual: Value = serde_json::from_str(&resp_txt).unwrap();
        if let Value::Array(arr) = actual {
            assert_eq!(arr.len(), 0);
        } else {
            panic!("Expected an array");
        }
        assert_eq!(response.status_code(), StatusCode::OK);
    }

    #[tokio::test]
    async fn projects_should_return_empty_array_with_no_data_using_filter_none() {
        let server = build_server(vec![]);

        let response = server.get("/api/v1/projects?filter=none").await;

        let resp_txt = response.text();
        let actual: Value = serde_json::from_str(&resp_txt).unwrap();
        if let Value::Array(arr) = actual {
            assert_eq!(arr.len(), 0);
        } else {
            panic!("Expected an array");
        }
        assert_eq!(response.status_code(), StatusCode::OK);
    }

    #[tokio::test]
    async fn invalid_filter_should_return_400() {
        let server = build_server(vec![]);
        let response = server.get("/api/v1/projects?filter=invalid").await;
        response.assert_status_bad_request();
    }

    #[tokio::test]
    async fn projects_should_return_known_projects() {
        let meta1 = test_data("0001");
        let meta2 = test_data("0002");
        let server = build_server(vec![meta1, meta2]);

        let response = server.get("/api/v1/projects").await;

        let resp_txt = response.text();
        let actual: Value = serde_json::from_str(&resp_txt).unwrap();
        if let Value::Array(arr) = actual {
            assert_eq!(arr.len(), 2);
        } else {
            panic!("Expected an array");
        }
        assert_eq!(response.status_code(), StatusCode::OK);
    }

    #[tokio::test]
    async fn projects_should_page() {
        let data = (1..5)
            .map(|i| format!("{:04}", i))
            .map(|shortcode| test_data(&shortcode))
            .collect::<Vec<_>>();

        let server = build_server(data);

        let response = server.get("/api/v1/projects?_page=1&_limit=2").await;

        let json: Vec<ProjectMetadataWithInfoDto> = response.json();
        assert_eq!(json.len(), 2);
        response.assert_header("X-Total-Count", "4");
        response.assert_status_ok();
    }

    fn test_data(shortcode: &str) -> DraftMetadata {
        test_data_with_status(shortcode, None)
    }

    fn test_data_with_status(shortcode: &str, status: Option<DraftProjectStatus>) -> DraftMetadata {
        let fake_name = Faker.fake::<String>();
        let expected: DraftMetadata = DraftMetadata {
            project: DraftProject {
                shortcode: Shortcode::try_from(shortcode.to_string()).expect("Valid shortcode"),
                status,
                name: fake_name,
                description: None,
                start_date: DraftDate(NaiveDate::default()),
                teaser_text: "a teaser".to_string(),
                datasets: Default::default(),
                keywords: NonEmpty::new(DraftText(HashMap::new())),
                disciplines: NonEmpty::new(DraftTextOrUrl::TextValue(DraftText(HashMap::new()))),
                temporal_coverage: None,
                spatial_coverage: None,
                funders: None,
                url: None,
                secondary_url: None,
                data_management_plan: None,
                end_date: None,
                contact_point: None,
                how_to_cite: None,
                publications: None,
                grants: None,
                alternative_names: None,
            },

            datasets: None,
            persons: None,
            organizations: None,
            grants: None,
        };
        expected
    }
}
