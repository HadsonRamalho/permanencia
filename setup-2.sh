### PARTE 2

#chmod +x setup-2.sh
source "$HOME/.cargo/env"
# Verifica e edita o arquivo pg_hba.conf
PG_HBA_PATH=$(sudo find / -name "pg_hba.conf" 2>/dev/null | head -n 1)

if [ -z "$PG_HBA_PATH" ]; then
    echo "==> Arquivo pg_hba.conf não encontrado!"
    exit 1
else
    echo "==> Editando $PG_HBA_PATH para usar 'trust'..."
    sudo sed -i 's/^\(host.*\)\(md5\|peer\|password\)$/\1trust/' "$PG_HBA_PATH"
fi

# Reinicia o PostgreSQL para aplicar mudanças
sudo systemctl restart postgresql

echo "==> Executando diesel setup..."
diesel setup

echo "==> Rodando o projeto..."
cargo run
