#!/bin/bash

# Parar script se algum comando falhar
set -e

echo "==> Instalando Rust..."
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"

echo "==> Instalando Diesel CLI..."
cargo install diesel_cli --no-default-features --features postgres

echo "==> Clonando repositório..."
git clone --branch pratica --single-branch https://github.com/HadsonRamalho/permanencia.git
cd permanencia || exit

# Verificar existência do arquivo diesel.toml e corrigir se necessário
if [ ! -f diesel.toml ]; then
    echo "==> Arquivo diesel.toml não encontrado em $(pwd)."
    echo "     Por favor, verifique se ele está em outro local e mova ou crie o symlink."
    exit 1
fi

echo "==> Instalando dependências de compilação..."
sudo zypper in -y cmake gcc

echo "==> Instalando PostgreSQL..."
sudo zypper in -y postgresql postgresql-server postgresql-contrib

echo "==> Iniciando serviço do PostgreSQL..."
sudo systemctl start postgresql
sudo systemctl enable postgresql
#sudo systemctl status postgresql

echo "==> Configurando usuário postgres..."
echo "==> Copie e cole os seguintes passos:"
echo "1 => psql"
echo "2 => ALTER USER postgres WITH PASSWORD '1234';"
echo "3 => exit"
echo "4 => exit"
echo "5 => ./setup-2.sh"
# Executar alteração de senha 
sudo su postgres
psql
ALTER USER postgres WITH PASSWORD '1234';
echo "==> CONFIGURADO"

#sudo -u postgres psql -c "ALTER USER postgres WITH PASSWORD '1234';"

# Executar: sudo su postgres
# Executar: psql

# Alternativo: sudo -u postgres psql

# Alternativo: su -
# Executar: sudo -u postgres psql

# ALTER USER postgres WITH PASSWORD '1234';