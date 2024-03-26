use eqo::commands::{listen::Listen, run::Run, server::Server};

async fn spawn_server() {
    let host = "127.0.0.1";
    let port = 8151;
    let quite = false;

    let server = Server::new(host.to_string(), port, quite);

    tokio::spawn(server.run());
}

#[tokio::test]
async fn test_server() {
    let server = spawn_server().await;

    dbg!("asd");
}

// async fn spawn_listen(secret: Option<String>) {
//     let host = "127.0.0.1";
//     let port = 8151;
//     let id = 1;
//     let clear = false;
//     let once = false;
//     let command = "echo 25";
//     let quite = false;

//     let client = Listen::new(
//         host.to_owned(),
//         port,
//         command.to_owned(),
//         id,
//         quite,
//         clear,
//         once,
//         secret,
//     );

//     tokio::spawn(client.run());
// }

// async fn spawn_run(secret: Option<String>) {
//     let host = "127.0.0.1";
//     let port = 8151;
//     let id = 1;

//     let run = Run::new(host.to_owned(), port, id, secret);

//     tokio::spawn(run.run());
// }
