
use rocket::{launch, routes, Build, Rocket};
//high perf server for managing game servers

//api routes

//start
//stop
//restart
//set_config
//get_config
//add_mod
//remove_mod
//list_mods

fn _kill_server() {
    //kill the server
}
fn _start_server() {
    //start the server
}
fn _restart_server() {
    //restart the server
}
fn _set_config() {
    //set the server config
}
fn _get_config() {
    //get the server config
}
fn _add_mod() {
    //add a mod to the server
}
fn _remove_mod() {
    //remove a mod from the server
}
fn _list_mods() {
    //list all mods on the server
}

//https://rocket.rs/guide/v0.5/overview/#overview

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().mount("/files", routes![])
}