use std::env;
use ureq::{Error};
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

fn main() {
    let args = Args::parse();
    let addr = env::var("URL").expect("env var url must exist");
    let resp = match args.command.as_str() {
        "registr-user" => ureq::post(format!("{}/{}", addr, args.command).as_str())
            .send_json(ureq::json!({
                "username" : args.username
            })),
        "create-group" => ureq::post(format!("{}/{}", addr, args.command).as_str())
            .send_json(ureq::json!({
                "username" : args.username,
                "group_name" : args.group_name
            })),
        "group-members" => ureq::get(format!("{}/{}", addr, args.command).as_str())
            .send_json(ureq::json!({
                "username" : args.username,
                "group_name" : args.group_name
            })),
        "join-group" => ureq::post(format!("{}/{}", addr, args.command).as_str())
            .send_json(ureq::json!({
                "username" : args.username,
                "group_name" : args.group_name
            })),
        "delete-group" => ureq::post(format!("{}/{}", addr, args.command).as_str())
            .send_json(ureq::json!({
                "username" : args.username,
                "group_name": args.group_name
            })),
        "get-recipient-name" => ureq::get(format!("{}/{}", addr, args.command).as_str())
            .send_json(ureq::json!({
                "username": args.username,
                "group_name": args.group_name
            })),
        "add-admin" => ureq::post(format!("{}/{}", addr, args.command).as_str())
            .send_json(ureq::json!({
                "username" : args.username,
                "group_name": args.group_name,
                "new_admin": args.new_admin
            })),
        "start-secret-santa" => ureq::post(format!("{}/{}", addr, args.command).as_str())
            .send_json(ureq::json!({
                "username": args.username,
                "group_name": args.group_name
            })),
        "revoke-admin-rights" => ureq::post(format!("{}/{}", addr, args.command).as_str())
            .send_json(ureq::json!({
                "username": args.username,
                "group_name": args.group_name
            })),
        "get-groups" => ureq::get(format!("{}/{}", addr, args.command).as_str())
            .send_json(ureq::json!({
                "username": args.username
            })),
        _ => panic!("unexpected request")
    };
    match resp {
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
