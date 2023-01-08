mod database;
mod json_models;
mod models;
mod schema;
mod errors;

use crate::database::Database;
use crate::json_models::*;
use serde_json::json;
use std::sync::{Arc, RwLock};
use tide::{log, Request, Response, StatusCode};

fn make_response_from_result(result: Result<String, tide::Error>) -> Response {
    match result {
        Ok(s) => {
            let mut response = Response::new(StatusCode::Ok);
            response.set_body(s);
            response
        }
        Err(e) => {
            let mut response = Response::new(e.status());
            let msg = e.into_inner().to_string();
            log::debug!("Error: {msg}");
            response.set_body(json!({"error_message": msg}).to_string());
            response
        }
    }
}

fn main() -> Result<(), std::io::Error> {
    let version: &'static str = env!("CARGO_PKG_VERSION");
    log::with_level(log::LevelFilter::Debug);

    let f = async {
        let database = Database;
        let state = Arc::new(RwLock::new(database));
        let mut app = tide::with_state(state);

        app.at("/version")
            .get(move |_| async move { Ok(format!("version: {version}")) });
        app.at("/registr-user")
            .post(|mut request: Request<Arc<RwLock<Database>>>| async move {
                let jreq = request.body_json().await;
                let Username { username } = match jreq {
                    Ok(v) => v,
                    Err(_) => {
                        return Ok(make_response_from_result(
                            Err(errors::error_bad_request("Incorect request".to_string()))
                        ))
                    }
                };

                let state = request.state();
                let guard = state.write().unwrap();

                Ok(make_response_from_result(
                    guard.create_user(username.as_str())
                ))
            });
        app.at("/create-group")
            .post(|mut request: Request<Arc<RwLock<Database>>>| async move {
                let jreq = request.body_json().await;
                let UserGroupName { username, group_name } = match jreq {
                    Ok(v) => v,
                    Err(_) => {
                        return Ok(make_response_from_result(
                            Err(errors::error_bad_request("Incorect request".to_string()))
                        ))
                    }
                };

                let state = request.state();
                let guard = state.write().unwrap();

                Ok(make_response_from_result(
                    guard.create_group_by_user(username.as_str(), group_name.as_str())
                ))
            });
        app.at("/join-group")
            .post(|mut request: Request<Arc<RwLock<Database>>>| async move {
                let jreq = request.body_json().await;
                let UserGroupName { username, group_name } = match jreq {
                    Ok(v) => v,
                    Err(_) => {
                        return Ok(make_response_from_result(
                            Err(errors::error_bad_request("Incorect request".to_string()))
                        ))
                    }
                };

                let state = request.state();
                let guard = state.write().unwrap();

                Ok(make_response_from_result(
                    guard.add_user_to_group(username.as_str(), group_name.as_str())
                ))
            });
        app.at("/delete-group")
            .post(|mut request: Request<Arc<RwLock<Database>>>| async move {
                let jreq = request.body_json().await;
                let UserGroupName { username, group_name } = match jreq {
                    Ok(v) => v,
                    Err(_) => {
                        return Ok(make_response_from_result(
                            Err(errors::error_bad_request("Incorect request".to_string()))
                        ))
                    }
                };

                let state = request.state();
                let guard = state.write().unwrap();

                Ok(make_response_from_result(
                    guard.delete_group_by_admin(username.as_str(), group_name.as_str())
                ))
            });
        app.at("/group-members")
            .get(|mut request: Request<Arc<RwLock<Database>>>| async move {
                let jreq = request.body_json().await;
                let UserGroupName { username, group_name } = match jreq {
                    Ok(v) => v,
                    Err(_) => {
                        return Ok(make_response_from_result(
                            Err(errors::error_bad_request("Incorect request".to_string()))
                        ))
                    }
                };

                let state = request.state();
                let guard = state.read().unwrap();

                Ok(make_response_from_result(
                    guard.get_group_members(username.as_str(), group_name.as_str())
                ))
            });
        app.at("/get-recipient-name")
            .get(|mut request: Request<Arc<RwLock<Database>>>| async move {
                let jreq = request.body_json().await;
                let UserGroupName { username, group_name } = match jreq {
                    Ok(v) => v,
                    Err(_) => {
                        return Ok(make_response_from_result(
                            Err(errors::error_bad_request("Incorect request".to_string()))
                        ))
                    }
                };

                let state = request.state();
                let guard = state.read().unwrap();

                Ok(make_response_from_result(
                    guard.get_recipient_name(username.as_str(), group_name.as_str())
                ))
            });
        app.at("/add-admin")
            .post(|mut request: Request<Arc<RwLock<Database>>>| async move {
                let jreq = request.body_json().await;
                let UserGroupNewAdminName { username, group_name, new_admin } = match jreq {
                    Ok(v) => v,
                    Err(_) => {
                        return Ok(make_response_from_result(
                            Err(errors::error_bad_request("Incorect request".to_string()))
                        ))
                    }
                };

                let state = request.state();
                let guard = state.write().unwrap();

                Ok(make_response_from_result(
                    guard.add_admin_to_group(username.as_str(), new_admin.as_str(), group_name.as_str())
                ))
            });
        app.at("/start-secret-santa")
            .post(|mut request: Request<Arc<RwLock<Database>>>| async move {
                let jreq = request.body_json().await;
                let UserGroupName { username, group_name } = match jreq {
                    Ok(v) => v,
                    Err(_) => {
                        return Ok(make_response_from_result(
                            Err(errors::error_bad_request("Incorect request".to_string()))
                        ))
                    }
                };

                let state = request.state();
                let guard = state.write().unwrap();

                Ok(make_response_from_result(
                    guard.close_group(username.as_str(), group_name.as_str())
                ))
            });
        app.at("/revoke-admin-rights")
            .post(|mut request: Request<Arc<RwLock<Database>>>| async move {
                let jreq = request.body_json().await;
                let UserGroupName { username, group_name } = match jreq {
                    Ok(v) => v,
                    Err(_) => {
                        return Ok(make_response_from_result(
                            Err(errors::error_bad_request("Incorect request".to_string()))
                        ))
                    }
                };

                let state = request.state();
                let guard = state.write().unwrap();

                Ok(make_response_from_result(
                    guard.revoke_rights_of_admin(username.as_str(), group_name.as_str())
                ))
            });
        app.at("/get-groups")
            .get(|request: Request<Arc<RwLock<Database>>>| async move {
                let state = request.state();
                let guard = state.read().unwrap();

                Ok(make_response_from_result(
                    guard.get_open_groups()
                ))
            });
        app.listen("127.0.0.1:80").await
    };
    futures::executor::block_on(f)
}
