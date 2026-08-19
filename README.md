# 🦀 Mon API Axum (Rust)

Bienvenue sur ce projet d'API REST développée en **Rust** en utilisant le framework web [Axum](https://github.com/tokio-rs/axum) et l'écosystème asynchrone [Tokio](https://tokio.rs/).

## 🚀 Fonctionnalités

- **Routage de base** : Points de terminaison simples pour vérifier l'état de l'API.
- **Gestion des paramètres** : Extraction de paramètres depuis l'URL (Path) et la chaîne de requête (Query).
- **État partagé (State)** : Gestion en mémoire d'une liste de produits via `Arc<Mutex<Vec<Product>>>` pour un accès concurrent sécurisé.
- **CRUD Produits** : Création, lecture et suppression de produits avec gestion des conflits (409 Conflict) et des éléments introuvables (404 Not Found).
- **Tests unitaires exhaustifs** : Le routage et les handlers sont couverts par des tests natifs sans nécessiter de serveur réseau (via `tower::ServiceExt::oneshot`).

## 📂 Structure du projet

- `main.rs` : Point d'entrée de l'application, initialise l'état partagé et lance le serveur TCP sur le port `3000`.
- `router.rs` : Définition des routes de l'API et tests unitaires intégrés.
- `handlers.rs` : Logique métier et contrôleurs pour chaque route (réponses HTTP, manipulation du JSON).
- `models.rs` : Définition des structures de données (`Product`, `Calculator`, `AppState`) et sérialisation/désérialisation avec `serde`.

## 🛣️ Points de terminaison (Endpoints)

### 🛠 Utilitaires
- `GET /` : Retourne un message de bienvenue.
- `GET /health` : Vérifie la santé de l'API (Retourne `OK`).
- `GET /greet/:name` : Salue l'utilisateur avec le nom fourni dans l'URL.
- `GET /calculator?a={nombre}&b={nombre}` : Additionne deux nombres passés en paramètres de requête.

### 📦 Produits (CRUD)
- `GET /products` : Récupère la liste de tous les produits au format JSON.
- `POST /products` : Ajoute un nouveau produit. 
  - **Corps (JSON)** : `{"id": 1, "name": "Clavier", "price": 50}`
  - **Réponses** : `201 Created` ou `409 Conflict` (si le produit existe déjà).
- `GET /products/:id` : Récupère les détails d'un produit spécifique via son ID.
  - **Réponses** : `200 OK` (avec le JSON du produit) ou `404 Not Found`.
- `DELETE /products/:id` : Supprime un produit spécifique.
  - **Réponses** : `204 No Content` ou `404 Not Found`.

## 🛠️ Installation & Exécution

Assurez-vous d'avoir [Rust et Cargo](https://rustup.rs/) installés sur votre machine.

```bash
# Lancer l'API
cargo run
```
Le serveur démarrera sur `http://0.0.0.0:3000`.

## 🧪 Lancer les tests

Le projet inclut une suite complète de tests asynchrones utilisant `tokio::test`. 
```bash
# Exécuter les tests unitaires
cargo test
```
