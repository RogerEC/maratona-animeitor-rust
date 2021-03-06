use std::env;
use tokio;

use lib_server::*;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Expected 2 arguments: {:?}", args);
        return;
    }
    let server_port :u16= match args[1].parse() {
        Ok(t) => t,
        Err(e) => panic!("Could not parse port {}", e),
    };
    let url_base = args[2].clone();

    let secret = random_path_part();

    
    println!("Maratona Rustreimator rodando!");
    println!(
        "-> Runs em http://localhost:{}/seed/runspanel.html",
        server_port
    );
    println!(
        "-> Placar automatizado em http://localhost:{}/seed/automatic.html",
        server_port
    );
    println!(
        "-> Placar interativo em http://localhost:{}/seed/stepping.html",
        server_port
    );
    println!(
        "-> Timer em http://localhost:{}/seed/timer.html",
        server_port
    );
    println!(
        "-> Painel geral em http://localhost:{}/seed/everything.html",
        server_port
    );
    println!(
        "-> Reveleitor em http://localhost:{}/seed/reveleitor.html?secret={}",
        server_port, secret
    );
    
    
    serve_simple_contest(url_base, server_port, &secret).await;

}


// // #[tokio::main]
// async fn main() {
//     let args: Vec<String> = env::args().collect();
//     if args.len() != 3 {
//         eprintln!("Expected 2 arguments: {:?}", args);
//         return;
//     }
//     let server_port = match args[1].parse() {
//         Ok(t) => t,
//         Err(e) => panic!("Could not parse port {}", e),
//     };
//     let url_base = args[2].clone();


//     println!("Maratona Rustreimator rodando!");
//     println!(
//         "-> Runs em http://localhost:{}/seed/runspanel.html",
//         server_port
//     );
//     println!(
//         "-> Placar automatizado em http://localhost:{}/seed/automatic.html",
//         server_port
//     );
//     println!(
//         "-> Placar interativo em http://localhost:{}/seed/stepping.html",
//         server_port
//     );
//     println!(
//         "-> Timer em http://localhost:{}/seed/timer.html",
//         server_port
//     );
//     println!(
//         "-> Painel geral em http://localhost:{}/seed/everything.html",
//         server_port
//     );
//     println!(
//         "-> Reveleitor em http://localhost:{}/seed/reveleitor.html#{}",
//         server_port, secret
//     );
    
//     warp::serve(routes).run(([0, 0, 0, 0], server_port)).await;
// }
