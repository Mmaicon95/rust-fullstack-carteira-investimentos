use askama::Template;
use rust_decimal::Decimal;
use crate::models::Ativo;

#[derive(Template)]
#[template(path = "dashboard.html")]
pub struct DashboardTemplate {
    pub usuario_nome: String,
    pub ativos: Vec<Ativo>,
}

impl DashboardTemplate {
    // Acoplamento do método matemático agregador direto na renderização do front-end
    pub fn calcular_valor_total_carteira(&self) -> Decimal {
        self.ativos
            .iter()
            .map(|ativo| ativo.calcular_subtotal())
            .sum()
    }
}
