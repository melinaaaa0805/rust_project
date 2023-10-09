use std::env;
use warp::{Filter, Rejection, Reply};
use env_logger::Env;
use warp::http::{HeaderMap, StatusCode};
use serde_json::{Map, Value};

// Handler pour la route "/ping"
async fn ping_handler(headers: HeaderMap) -> Result<impl Reply, Rejection> {
    // Convertir les en-têtes HTTP en un format JSON
    let headers_json = headers
        .iter()
        .map(|(name, value)| {
            (
                name.as_str().to_string(),
                Value::String(value.to_str().unwrap_or("").to_string()),
            )
        })
        .collect::<Map<String, Value>>();

    // Créer une réponse JSON avec les en-têtes
    let response_json = warp::reply::json(&headers_json);

    Ok(response_json)
}

async fn not_found_handler() -> Result<impl Reply, Rejection> {
    Ok(warp::reply::with_status(warp::reply::html(""), StatusCode::NOT_FOUND))
}

#[tokio::main]
async fn main() {
    // Configurer la journalisation des messages d'enregistrement
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // Lire la variable d'environnement PING_LISTEN_PORT ou utiliser 8080 par défaut
    let listen_port = env::var("PING_LISTEN_PORT")
        .unwrap_or_else(|_| "8080".to_string());

    // Créer une chaîne de format pour spécifier le port
    let addr = format!("127.0.0.1:{}", listen_port);

    // Créer une route pour gérer les requêtes GET sur "/ping"
    let ping_route = warp::path!("ping")
        .and(warp::get())
        .and(warp::header::headers_cloned())
        .and_then(ping_handler);

    // Créer une route de secours pour les routes non prises en charge (404)
    let not_found_route = warp::any()
        .and_then(not_found_handler);

    // Combiner les routes
    let _routes = ping_route.or(not_found_route);

    // Démarrer le serveur Warp sur l'adresse spécifiée
    warp::serve(ping_route)
        .run(addr.parse::<std::net::SocketAddr>().expect("Invalid address format"))
        .await;
}
