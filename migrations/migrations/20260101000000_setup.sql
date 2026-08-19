CREATE TABLE IF NOT EXISTS usuarios (
    id SERIAL PRIMARY KEY,
    nome VARCHAR(100) NOT NULL,
    email VARCHAR(150) UNIQUE NOT NULL
);

CREATE TABLE IF NOT EXISTS ativos (
    id SERIAL PRIMARY KEY,
    usuario_id INT NOT NULL REFERENCES usuarios(id) ON DELETE CASCADE,
    ticker VARCHAR(10) NOT NULL,
    quantidade NUMERIC(16, 8) NOT NULL,
    preco_atual NUMERIC(12, 2) NOT NULL
);

-- Inserindo usuário de teste assinado
INSERT INTO usuarios (id, nome, email) 
VALUES (1, 'Mmaicon95', 'maicon@apexcarteira.com')
ON CONFLICT DO NOTHING;

-- Inserindo ativos de teste para validação dos cálculos
INSERT INTO ativos (usuario_id, ticker, quantidade, preco_atual) VALUES 
(1, 'PETR4', 100.00000000, 36.50),
(1, 'VALE3', 50.00000000, 62.20),
(1, 'IVVB11', 10.00000000, 280.10)
ON CONFLICT DO NOTHING;
