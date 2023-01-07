mod database;
mod json_models;
mod models;
mod schema;

use crate::database::Database;
use crate::json_models::*;
use serde_json::json;
use std::sync::{Arc, Mutex};
use tide::Request;

fn main() -> Result<(), std::io::Error> {
    let version: &'static str = env!("CARGO_PKG_VERSION");
    tide::log::start();
    let f = async {
        let database = Database;
        let state = Arc::new(Mutex::new(database));
        let mut app = tide::with_state(state);

        app.at("/version")
            .get(move |_| async move { Ok(format!("version: {version}")) });
        app.at("/registr-user")
            .post(|mut request: Request<Arc<Mutex<Database>>>| async move {
                let Username { username } = request.body_json().await.map_err(|e| {
                    tide::Error::from_str(tide::StatusCode::BadRequest, json!(e.to_string()))
                })?;

                let state = request.state();
                let guard = state.lock().unwrap();

                match guard.create_user(username.as_str()) {
                    Ok(_) => Ok(json!(tide::StatusCode::Ok)),
                    Err(e) => Err(tide::Error::from_str(
                        tide::StatusCode::Conflict,
                        json!(e.to_string()),
                    )),
                }
            });
        app.at("/create-group")
            .post(|mut request: Request<Arc<Mutex<Database>>>| async move {
                let UserGroupName {
                    username,
                    group_name,
                } = request.body_json().await.map_err(|e| {
                    tide::Error::from_str(tide::StatusCode::BadRequest, json!(e.to_string()))
                })?;

                let state = request.state();
                let guard = state.lock().unwrap();

                match guard.create_group_by_user(username.as_str(), group_name.as_str()) {
                    Ok(_) => Ok(json!(tide::StatusCode::Ok)),
                    Err(e) => Err(tide::Error::from_str(
                        tide::StatusCode::Conflict,
                        json!(e.to_string()),
                    )),
                }
            });
        app.at("/join-group")
            .post(|mut request: Request<Arc<Mutex<Database>>>| async move {
                let UserGroupName {
                    username,
                    group_name,
                } = request.body_json().await.map_err(|e| {
                    tide::Error::from_str(tide::StatusCode::BadRequest, json!(e.to_string()))
                })?;

                let state = request.state();
                let guard = state.lock().unwrap();

                match guard.add_user_to_group(username.as_str(), group_name.as_str()) {
                    Ok(_) => Ok(json!(tide::StatusCode::Ok)),
                    Err(e) => Err(tide::Error::from_str(
                        tide::StatusCode::Conflict,
                        json!(e.to_string()),
                    )),
                }
            });
        app.at("/delete-group")
            .post(|mut request: Request<Arc<Mutex<Database>>>| async move {
                let UserGroupName {
                    username,
                    group_name,
                } = request.body_json().await.map_err(|e| {
                    tide::Error::from_str(tide::StatusCode::BadRequest, json!(e.to_string()))
                })?;

                let state = request.state();
                let guard = state.lock().unwrap();

                match guard.delete_group_by_admin(username.as_str(), group_name.as_str()) {
                    Ok(_) => Ok(json!(tide::StatusCode::Ok)),
                    Err(e) => Err(tide::Error::from_str(
                        tide::StatusCode::NotFound,
                        json!(e.to_string()),
                    )),
                }
            });
        app.at("/group-members")
            .get(|mut request: Request<Arc<Mutex<Database>>>| async move {
                let UserGroupName {
                    username,
                    group_name,
                } = request.body_json().await.map_err(|e| {
                    tide::Error::from_str(tide::StatusCode::BadRequest, json!(e.to_string()))
                })?;

                let state = request.state();
                let guard = state.lock().unwrap();

                match guard.get_group_members(username.as_str(), group_name.as_str()) {
                    Ok(list) => Ok(json!({group_name: list})),
                    Err(e) => Err(tide::Error::from_str(
                        tide::StatusCode::Conflict,
                        json!(e.to_string())
                    )),
                }
            });
        app.at("/get-recipient-name")
            .get(|mut request: Request<Arc<Mutex<Database>>>| async move {
                let UserGroupName {
                    username,
                    group_name,
                } = request.body_json().await.map_err(|e| {
                    tide::Error::from_str(tide::StatusCode::BadRequest, json!(e.to_string()))
                })?;

                let state = request.state();
                let guard = state.lock().unwrap();

                match guard.get_recipient_name(username.as_str(), group_name.as_str()) {
                    Ok(result) => Ok(json!({"recipient": result})),
                    Err(e) => Err(tide::Error::from_str(
                        tide::StatusCode::Conflict,
                        json!(e.to_string())
                    )),
                }
            });
        app.at("/add-admin")
            .post(|mut request: Request<Arc<Mutex<Database>>>| async move {
                let NewAdminUserGroupName {
                    username,
                    group_name,
                    new_admin,
                } = request.body_json().await.map_err(|e| {
                    tide::Error::from_str(tide::StatusCode::BadRequest, json!(e.to_string()))
                })?;

                let state = request.state();
                let guard = state.lock().unwrap();

                match guard.add_admin_to_group(username.as_str(), new_admin.as_str(), group_name.as_str()) {
                    Ok(_) => Ok(json!(tide::StatusCode::Ok)),
                    Err(e) => Err(tide::Error::from_str(
                        tide::StatusCode::Conflict,
                        json!(e.to_string()),
                    )),
                }
            });
        app.at("/start-secret-santa")
            .post(|mut request: Request<Arc<Mutex<Database>>>| async move {
                let UserGroupName {
                    username,
                    group_name,
                } = request.body_json().await.map_err(|e| {
                    tide::Error::from_str(tide::StatusCode::BadRequest, json!(e.to_string()))
                })?;

                let state = request.state();
                let guard = state.lock().unwrap();

                match guard.close_group(username.as_str(), group_name.as_str()) {
                    Ok(_) => Ok(json!(tide::StatusCode::Ok)),
                    Err(e) => Err(tide::Error::from_str(
                        tide::StatusCode::Conflict,
                        json!(e.to_string()),
                    )),
                }
            });

        app.listen("127.0.0.1:8080").await

    };
    futures::executor::block_on(f)
}
