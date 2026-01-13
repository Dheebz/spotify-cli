use super::init_pin_store;
use crate::io::output::{ErrorKind, Response};
use crate::storage::pins::{Pin, ResourceType};

pub async fn pin_add(
    resource_type: &str,
    url_or_id: &str,
    alias: &str,
    tags: Option<&str>,
) -> Response {
    let rtype: ResourceType = match resource_type.parse() {
        Ok(t) => t,
        Err(e) => return Response::err(400, &e, ErrorKind::Validation),
    };

    let id = Pin::extract_id(url_or_id);
    let tag_list: Vec<String> = tags
        .map(|t| t.split(',').map(|s| s.trim().to_string()).collect())
        .unwrap_or_default();

    let pin = Pin::new(rtype, id.clone(), alias.to_string(), tag_list.clone());

    let mut store = match init_pin_store() {
        Ok(s) => s,
        Err(e) => return e,
    };

    match store.add(pin) {
        Ok(_) => Response::success_with_payload(
            201,
            "Pin added",
            serde_json::json!({
                "type": resource_type,
                "id": id,
                "alias": alias,
                "tags": tag_list,
                "uri": format!("spotify:{}:{}", resource_type, id)
            }),
        ),
        Err(e) => Response::err_with_details(400, "Failed to add pin", ErrorKind::Storage, e.to_string()),
    }
}

pub async fn pin_remove(alias_or_id: &str) -> Response {
    let mut store = match init_pin_store() {
        Ok(s) => s,
        Err(e) => return e,
    };

    match store.remove(alias_or_id) {
        Ok(removed) => Response::success_with_payload(
            200,
            "Pin removed",
            serde_json::json!({
                "type": removed.resource_type.as_str(),
                "id": removed.id,
                "alias": removed.alias
            }),
        ),
        Err(e) => Response::err_with_details(404, "Failed to remove pin", ErrorKind::NotFound, e.to_string()),
    }
}

pub async fn pin_list(filter_type: Option<&str>) -> Response {
    let store = match init_pin_store() {
        Ok(s) => s,
        Err(e) => return e,
    };

    let rtype: Option<ResourceType> = match filter_type {
        Some(t) => match t.parse() {
            Ok(rt) => Some(rt),
            Err(e) => return Response::err(400, &e, ErrorKind::Validation),
        },
        None => None,
    };

    let pins: Vec<serde_json::Value> = store
        .list(rtype)
        .iter()
        .map(|p| {
            serde_json::json!({
                "type": p.resource_type.as_str(),
                "id": p.id,
                "alias": p.alias,
                "tags": p.tags,
                "uri": p.uri()
            })
        })
        .collect();

    Response::success_with_payload(
        200,
        format!("{} pin(s)", pins.len()),
        serde_json::json!({ "pins": pins }),
    )
}
