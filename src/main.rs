use axum::{extract::State, response::IntoResponse, routing::get, Router};
use dotenvy::dotenv;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::net::SocketAddr;

mod models;
mod templates;

use templates::DashboardTemplate;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    // Customização de logs internos com assinatura solicitada
    println!("=======================================================");
    println!("   🚀 INICIALIZANDO ECOSSISTEMA: APEX CARTEIRA");
    println!("   🛠️  Desenvolvido por: Mmaicon95");
    println!("   📦 Status: Backend Core rodando sob Tokio Runtime");
    println!("=======================================================");

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password123@localhost:5432/apex_carteira".to_string());

    println!("🔄 Conectando ao banco de dados PostgreSQL...");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;
    println!("✅ Conexão com o banco de dados estabelecida com sucesso!");

    // Configuração das rotas do Axum
    let app = Router::new()
        .route("/", get(html_dashboard_handler))
        .with_state(pool);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    println!("🌍 Servidor Apex Carteira online em http://localhost:3000");

    axum::serve(listener, app).await?;
    Ok(())
}

// Handler que busca dados do banco e renderiza a view do Askama
async fn html_dashboard_handler(State(pool): State<PgPool>) -> impl IntoResponse {
    // Busca os ativos do usuário mockado (ID 1)
    let ativos = sqlx::query_as::<_, models::Ativo>(
        "SELECT id, usuario_id, ticker, quantidade, preco_atual FROM ativos WHERE usuario_id = $1"
    )
    .bind(1)
    .fetch_all(&pool)
    .await
    .unwrap_or_default();

    // Instancia o template do Askama
    DashboardTemplate {
        usuario_nome: "Mmaicon95".to_string(),
        ativos,
    }
}
