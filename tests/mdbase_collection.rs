use pickle::collection::PickleCollection;
use pickle::model::{CreateRequest, CreateResponse, RequestState};
use pickle::templates::{
    ACK_RESPONSE_TYPE, APPROVAL_RESPONSE_TYPE, MDBASE_CONFIG_V02, REQUEST_TYPE,
};
use serde_json::{json, Value};
use std::fs;
use tempfile::tempdir;

#[test]
fn new_collections_use_canonical_v03_config_and_type_files() {
    let root = tempdir().unwrap();
    let collection = PickleCollection::new(root.path());

    collection.ensure().unwrap();

    let config = yaml_to_json(&fs::read_to_string(root.path().join("mdbase.yaml")).unwrap());
    assert!(mdbase::v03::validate_config(&config, "mdbase.yaml").is_empty());
    assert_eq!(config["spec_version"], "0.3.0");

    for type_name in [REQUEST_TYPE, APPROVAL_RESPONSE_TYPE, ACK_RESPONSE_TYPE] {
        let path = format!("_types/{type_name}.md");
        let frontmatter = markdown_frontmatter(
            &fs::read_to_string(root.path().join(&path)).expect("generated type file"),
        );
        assert_eq!(frontmatter["kind"], "mdbase.type");
        assert!(mdbase::v03::validate_type_file(&frontmatter, &path).is_empty());
    }

    let type_definition = collection
        .read_type_definition(APPROVAL_RESPONSE_TYPE)
        .unwrap();
    assert_eq!(type_definition.kind.as_deref(), Some("mdbase.type"));
    assert!(type_definition.fields.is_empty());
    assert!(type_definition.schema.is_some());
    assert!(type_definition.collection.is_some());
    assert!(type_definition.lifecycle.is_some());

    let validation = collection.validate().unwrap();
    assert_eq!(validation["valid"], true);
    assert!(validation.get("diagnostics").is_some());
}

#[test]
fn existing_v02_collections_keep_v02_templates_and_crud_behavior() {
    let root = tempdir().unwrap();
    fs::write(root.path().join("mdbase.yaml"), MDBASE_CONFIG_V02).unwrap();
    let collection = PickleCollection::new(root.path());

    collection.ensure().unwrap();

    let request_type = fs::read_to_string(root.path().join("_types/pickle_request.md")).unwrap();
    let frontmatter = markdown_frontmatter(&request_type);
    assert_eq!(frontmatter["name"], REQUEST_TYPE);
    assert!(frontmatter.get("fields").is_some());
    assert!(frontmatter.get("kind").is_none());

    let request = collection
        .create_request(CreateRequest {
            title: "Legacy collection request".to_string(),
            ..empty_request()
        })
        .unwrap();
    assert_eq!(request.title, "Legacy collection request");
}

#[test]
fn unsupported_collection_versions_fail_closed_before_writing_types() {
    let root = tempdir().unwrap();
    fs::write(
        root.path().join("mdbase.yaml"),
        "spec_version: \"0.4.0\"\nsettings: {}\n",
    )
    .unwrap();
    let collection = PickleCollection::new(root.path());

    let error = collection.ensure().unwrap_err().to_string();

    assert!(error.contains("unsupported mdbase spec_version 0.4.0"));
    assert!(!root.path().join("_types/pickle_request.md").exists());
}

#[test]
fn approval_response_is_a_linked_mdbase_file_without_status_mutation() {
    let root = tempdir().unwrap();
    let collection = PickleCollection::new(root.path());

    let request = collection
        .create_request(CreateRequest {
            title: "Approve build".to_string(),
            source: "vitest".to_string(),
            kind: "approval".to_string(),
            message: "Approve the build?".to_string(),
            body: "The smoke build passed.".to_string(),
            priority: "normal".to_string(),
            response_type: APPROVAL_RESPONSE_TYPE.to_string(),
            tags: vec!["ci".to_string()],
            ..empty_request()
        })
        .unwrap();

    assert_eq!(request.state, RequestState::Pending);
    assert_eq!(request.response_count, 0);
    assert!(root.path().join(&request.path).exists());

    let answered = collection
        .respond(
            &request.id,
            CreateResponse {
                responder: "callum".to_string(),
                payload: json!({ "decision": "approve", "comment": "Ship it." }),
            },
        )
        .unwrap();

    assert_eq!(answered.state, RequestState::Answered);
    assert_eq!(answered.status, "answered");
    assert_eq!(answered.response_count, 1);
    assert_eq!(
        answered.response.as_ref().unwrap().payload,
        json!({ "decision": "approve", "comment": "Ship it." })
    );

    let request_file = fs::read_to_string(root.path().join(&request.path)).unwrap();
    assert!(!request_file.contains("status: answered"));

    let response_path = &answered.response.as_ref().unwrap().path;
    let response_file = fs::read_to_string(root.path().join(response_path)).unwrap();
    assert!(response_file.contains(&format!(
        "request: '[[{}]]'",
        request.path.trim_end_matches(".md")
    )));
}

#[test]
fn message_requests_default_to_ack_response_type() {
    let root = tempdir().unwrap();
    let collection = PickleCollection::new(root.path());

    let request = collection
        .create_request(CreateRequest {
            title: "Read deployment update".to_string(),
            source: "tickle".to_string(),
            kind: "message".to_string(),
            message: "Deployment finished cleanly.".to_string(),
            ..empty_request()
        })
        .unwrap();

    assert_eq!(request.response_type, ACK_RESPONSE_TYPE);
    assert_eq!(
        request.response_type_definition.unwrap().name,
        ACK_RESPONSE_TYPE
    );

    let answered = collection
        .respond(
            &request.id,
            CreateResponse {
                responder: "callum".to_string(),
                payload: json!({ "message": "Acknowledged." }),
            },
        )
        .unwrap();

    assert_eq!(answered.state, RequestState::Answered);
    assert_eq!(
        answered.response.unwrap().payload,
        json!({ "message": "Acknowledged." })
    );
}

#[test]
fn request_listing_is_unbounded_by_default_and_honors_explicit_limits() {
    let root = tempdir().unwrap();
    let collection = PickleCollection::new(root.path());
    collection.ensure().unwrap();

    for index in 0..505 {
        fs::write(
            root.path()
                .join("requests")
                .join(format!("req-{index:03}.md")),
            format!(
                "---\ntype: {REQUEST_TYPE}\nid: req-{index:03}\ntitle: Request {index:03}\nsource: test\nkind: approval\npriority: normal\nresponse_type: {APPROVAL_RESPONSE_TYPE}\ncreated_at: \"2026-05-25T00:00:00Z\"\n---\nBody {index:03}\n"
            ),
        )
        .unwrap();
    }

    assert_eq!(
        collection.list_requests("pending", None).unwrap().len(),
        505
    );
    assert_eq!(
        collection
            .list_requests("pending", Some(501))
            .unwrap()
            .len(),
        501
    );
}

#[test]
fn multiple_response_links_make_request_conflicted() {
    let root = tempdir().unwrap();
    let collection = PickleCollection::new(root.path());
    let request = collection
        .create_request(CreateRequest {
            title: "Choose deployment".to_string(),
            source: "vitest".to_string(),
            kind: "approval".to_string(),
            response_type: APPROVAL_RESPONSE_TYPE.to_string(),
            ..empty_request()
        })
        .unwrap();

    let link = request.path.trim_end_matches(".md");
    let responses = root.path().join("responses");
    fs::write(
        responses.join("one.md"),
        format!(
            "---\ntype: {APPROVAL_RESPONSE_TYPE}\nrequest: '[[{link}]]'\ndecision: approve\nresponded_at: \"2026-05-25T00:00:00Z\"\nresponder: one\n---\n"
        ),
    )
    .unwrap();
    fs::write(
        responses.join("two.md"),
        format!(
            "---\ntype: {APPROVAL_RESPONSE_TYPE}\nrequest: '[[{link}]]'\ndecision: reject\nresponded_at: \"2026-05-25T00:01:00Z\"\nresponder: two\n---\n"
        ),
    )
    .unwrap();

    let conflicted = collection.get_request_by_id(&request.id).unwrap();
    assert_eq!(conflicted.state, RequestState::Conflict);
    assert_eq!(conflicted.status, "conflict");
    assert_eq!(conflicted.response_count, 2);
    assert!(conflicted.response.is_none());
}

fn markdown_frontmatter(content: &str) -> Value {
    let frontmatter = content
        .strip_prefix("---\n")
        .and_then(|content| content.split_once("\n---"))
        .map(|(frontmatter, _)| frontmatter)
        .expect("markdown frontmatter");
    yaml_to_json(frontmatter)
}

fn yaml_to_json(content: &str) -> Value {
    serde_json::to_value(serde_yaml::from_str::<serde_yaml::Value>(content).unwrap()).unwrap()
}

fn empty_request() -> CreateRequest {
    CreateRequest {
        source: String::new(),
        kind: String::new(),
        title: String::new(),
        message: String::new(),
        body: String::new(),
        priority: String::new(),
        response_type: String::new(),
        tags: Vec::new(),
        links: Vec::new(),
        attachments: Vec::new(),
        metadata: json!({}),
        context: json!({}),
        dedupe_key: String::new(),
    }
}
