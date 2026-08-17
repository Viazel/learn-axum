use axum::Router;
use axum::routing::get;

use crate::handlers::*;
use crate::models::AppState;

pub fn app(state: AppState) -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/health", get(ok))
        .route("/greet/:name", get(greet))
        .route("/calculator", get(calculator))
        .route("/products", get(products).post(create_product))
        .route("/products/:id", get(get_product).delete(delete_product))
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use crate::models::Product;

    use super::*; // Importe la fonction app() et tes handlers
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use http_body_util::BodyExt; // Pour collecter le body de la réponse
    use tower::ServiceExt; // Pour la méthode `oneshot`

    #[tokio::test]
    async fn test_root() {
        let state = Arc::new(Mutex::new(Vec::new()));

        let app = app(state);

        // On crée une fausse requête HTTP entrante
        let request = Request::builder().uri("/").body(Body::empty()).unwrap();

        // On l'envoie au routeur sans réseau (oneshot)
        let response = app.oneshot(request).await.unwrap();

        // Vérification du statut HTTP
        assert_eq!(response.status(), StatusCode::OK);

        // Récupération et vérification du body
        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(&body[..], b"Bienvenue sur mon API Axum !");
    }

    #[tokio::test]
    async fn test_health() {
        let state = Arc::new(Mutex::new(Vec::new()));

        let app = app(state);

        let request = Request::builder()
            .uri("/health")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(&body[..], b"OK");
    }

    #[tokio::test]
    async fn test_greet() {
        let state = Arc::new(Mutex::new(Vec::new()));

        let app = app(state);
        let request = Request::builder()
            .uri("/greet/Bob")
            .body(Body::empty())
            .unwrap();
        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(&body[..], b"Bonjour, Bob !");
    }

    #[tokio::test]
    async fn test_calculator() {
        let state = Arc::new(Mutex::new(Vec::new()));

        let app = app(state);
        let request = Request::builder()
            .uri("/calculator?a=15&b=7")
            .body(Body::empty())
            .unwrap();
        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(&body[..], b"15 + 7 = 22");
    }

    #[tokio::test]
    async fn test_calculator_bad_request() {
        let state = Arc::new(Mutex::new(Vec::new()));

        let app = app(state);
        // Il manque le paramètre 'b', ou 'a' n'est pas un nombre !
        let request = Request::builder()
            .uri("/calculator?a=dix")
            .body(Body::empty())
            .unwrap();
        let response = app.oneshot(request).await.unwrap();

        // Axum doit automatiquement rejeter la requête
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    use axum::http::header;
    use std::sync::{Arc, Mutex};

    #[tokio::test]
    async fn test_create_and_get_products() {
        // 1. Initialisation de l'état partagé (vide au départ)
        let state = Arc::new(Mutex::new(Vec::new()));
        let app = app(state.clone());

        // 2. On envoie un POST /products avec du JSON
        let request_post = Request::builder()
            .method("POST")
            .uri("/products")
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(r#"{"id": 1, "name": "Clavier", "price": 50}"#))
            .unwrap();

        // ServiceExt::oneshot consomme le routeur, on doit donc le cloner
        // (Cloner un Router Axum est très peu coûteux)
        let response_post = app.clone().oneshot(request_post).await.unwrap();

        // On attend un 201 Created
        assert_eq!(response_post.status(), StatusCode::CREATED);

        // 3. On vérifie que l'état a bien été modifié en mémoire
        assert_eq!(state.lock().unwrap().len(), 1);

        // 4. On appelle GET /products pour récupérer la liste JSON
        let request_get = Request::builder()
            .uri("/products")
            .body(Body::empty())
            .unwrap();

        let response_get = app.oneshot(request_get).await.unwrap();
        assert_eq!(response_get.status(), StatusCode::OK);

        // On vérifie que le JSON retourné correspond bien à ce qu'on a inséré
        let body = response_get.into_body().collect().await.unwrap().to_bytes();
        let json_value: serde_json::Value = serde_json::from_slice(&body).unwrap();

        // json_value est un tableau contenant 1 objet
        assert_eq!(json_value[0]["name"], "Clavier");
        assert_eq!(json_value[0]["price"], 50);
    }

    #[tokio::test]
    async fn test_get_product_by_id() {
        // On initialise l'état avec des produits
        let state = Arc::new(Mutex::new(vec![
            Product {
                id: 1,
                name: "Clavier".into(),
                price: 50,
            },
            Product {
                id: 2,
                name: "Souris".into(),
                price: 25,
            },
        ]));
        let app = app(state);

        // Test 1: Produit existant
        let req = Request::builder()
            .uri("/products/1")
            .body(Body::empty())
            .unwrap();
        let res = app.clone().oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);

        // Test 2: Produit inexistant
        let req_404 = Request::builder()
            .uri("/products/999")
            .body(Body::empty())
            .unwrap();
        let res_404 = app.oneshot(req_404).await.unwrap();
        assert_eq!(res_404.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_delete_product() {
        let state = Arc::new(Mutex::new(vec![Product {
            id: 1,
            name: "Clavier".into(),
            price: 50,
        }]));
        let app = app(state.clone());

        // Test 1: Suppression réussie
        let req = Request::builder()
            .method("DELETE")
            .uri("/products/1")
            .body(Body::empty())
            .unwrap();
        let res = app.clone().oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::NO_CONTENT); // 204

        // Vérifie qu'il a bien été supprimé en mémoire
        assert_eq!(state.lock().unwrap().len(), 0);

        // Test 2: Suppression d'un produit qui n'existe plus
        let req_404 = Request::builder()
            .method("DELETE")
            .uri("/products/1")
            .body(Body::empty())
            .unwrap();
        let res_404 = app.oneshot(req_404).await.unwrap();
        assert_eq!(res_404.status(), StatusCode::NOT_FOUND);
    }
}
