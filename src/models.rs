use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, Clone)]
pub struct Ativo {
    pub id: i32,
    pub usuario_id: i32,
    pub ticker: String,
    pub quantidade: Decimal,
    pub preco_atual: Decimal,
}

impl Ativo {
    // Processa dinamicamente os subtotais patrimoniais de cada ativo financeiro
    pub fn calcular_subtotal(&self) -> Decimal {
        self.quantidade * self.preco_atual
    }
}
