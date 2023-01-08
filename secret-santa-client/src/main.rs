use std::env;
use ureq::{Error, Response};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args
{
    #[arg(short, long)]
    command: String,
    #[arg(short, long)]
    username: Option<String>,
    #[arg(short, long)]
    group_name: Option<String>,
    #[arg(short, long)]
    new_admin: Option<String>,
}

fn register_user(username: String) -> Result<Response, Error> {
    let addr = env::var("URL").expect("env var url must exist");
    ureq::post(format!("{}/registr-user", addr).as_str())
        .send_json(ureq::json!({ "username": username }))
}

fn create_group(username: String, group_name: String) -> Result<Response, Error> {
    let addr = env::var("URL").expect("env var url must exist");
    ureq::post(format!("{}/create-group", addr).as_str())
        .send_json(ureq::json!({ "username": username, "group_name": group_name }))
}

fn group_members(username: String, group_name: String) -> Result<Response, Error> {
    let addr = env::var("URL").expect("env var url must exist");
    ureq::get(format!("{}/group-members", addr).as_str())
        .send_json(ureq::json!({ "username": username, "group_name": group_name }))
}

fn join_group(username: String, group_name: String) -> Result<Response, Error> {
    let addr = env::var("URL").expect("env var url must exist");
    ureq::post(format!("{}/join-group", addr).as_str())
        .send_json(ureq::json!({ "username": username, "group_name": group_name }))
}

fn delete_group(username: String, group_name: String) -> Result<Response, Error> {
    let addr = env::var("URL").expect("env var url must exist");
    ureq::post(format!("{}/delete-group", addr).as_str())
        .send_json(ureq::json!({ "username": username, "group_name": group_name }))
}

fn get_recipient_name(username: String, group_name: String) -> Result<Response, Error> {
    let addr = env::var("URL").expect("env var url must exist");
    ureq::get(format!("{}/get-recipient-name", addr).as_str())
        .send_json(ureq::json!({ "username": username, "group_name": group_name }))
}

fn add_admin(username: String, group_name: String, new_admin: String) -> Result<Response, Error> {
    let addr = env::var("URL").expect("env var url must exist");
    ureq::post(format!("{}/add-admin", addr).as_str())
    .send_json(ureq::json!({ "username": username, "group_name": group_name, "new_admin": new_admin }))
}

fn start_secret_santa(username: String, group_name: String) -> Result<Response, Error> {
let addr = env::var("URL").expect("env var url must exist");
ureq::post(format!("{}/start-secret-santa", addr).as_str())
    .send_json(ureq::json!({ "username": username, "group_name": group_name }))
}

fn revoke_admin_rights(username: String, group_name: String) -> Result<Response, Error> {
let addr = env::var("URL").expect("env var url must exist");
ureq::post(format!("{}/revoke-admin-rights", addr).as_str())
    .send_json(ureq::json!({ "username": username, "group_name": group_name }))
}

fn get_groups(username: String) -> Result<Response, Error> {
let addr = env::var("URL").expect("env var url must exist");
ureq::get(format!("{}/get-groups", addr).as_str())
    .send_json(ureq::json!({ "username": username }))
}

fn handle_response(response: Result<Response, Error>) {
match response {
    Ok(response) => {
        println!("Response: {response:?}");
    }
    Err(Error::Status(code, response)) => {
        println!("Status code: {code} {0:?}", response.status_text());
        println!("Response: {response:?}");
    }
    Err(_) => {}
}
}

fn main() {
    let args = Args::parse();
    let resp = match args.command.as_str() {
        "registr-user" => register_user(args.username.unwrap()),
        "create-group" => create_group(args.username.unwrap(), args.group_name.unwrap()),
        "group-members" => group_members(args.username.unwrap(), args.group_name.unwrap()),
        "join-group" => join_group(args.username.unwrap(), args.group_name.unwrap()),
        "delete-group" => delete_group(args.username.unwrap(), args.group_name.unwrap()),
        "get-recipient-name" => get_recipient_name(args.username.unwrap(), args.group_name.unwrap()),
        "add-admin" => add_admin(args.username.unwrap(), args.group_name.unwrap(), args.new_admin.unwrap()),
        "start-secret-santa" => start_secret_santa(args.username.unwrap(), args.group_name.unwrap()),
        "revoke-admin-rights" => revoke_admin_rights(args.username.unwrap(), args.group_name.unwrap()),
        "get-groups" => get_groups(args.username.unwrap()),
        _ => panic!("unexpected request")
    };
    handle_response(resp);
}