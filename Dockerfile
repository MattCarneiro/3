# Etapa de Construção
    FROM --platform=linux/arm64 rust:1.81 AS builder

    # Instalar dependências necessárias
    RUN apt-get update && apt-get install -y \
        pkg-config \
        libssl-dev \
        musl-tools \
        musl-dev \
        gcc-aarch64-linux-gnu \
        && rm -rf /var/lib/apt/lists/*

    # Definir o diretório de trabalho
    WORKDIR /app

    # Copiar arquivos do projeto
    COPY . .

    # Adicionar o target musl para compilação estática
    RUN rustup target add aarch64-unknown-linux-musl

    # Compilar a aplicação em modo release para o target especificado
    RUN CC_aarch64_unknown_linux_musl=aarch64-linux-gnu-gcc \
        OPENSSL_DIR=/usr/lib/ssl \
        cargo build --release --target aarch64-unknown-linux-musl

    # Otimizar o binário
    RUN strip target/aarch64-unknown-linux-musl/release/check_downloadable

    # Etapa Final
    FROM --platform=linux/arm64 alpine:latest

    # Instalar dependências necessárias
    RUN apk add --no-cache ca-certificates

    # Definir o diretório de trabalho
    WORKDIR /app

    # Copiar o binário da etapa de build
    COPY --from=builder /app/target/aarch64-unknown-linux-musl/release/check_downloadable ./

    # Expor a porta necessária
    EXPOSE 3000

    # Definir a variável de ambiente PORT (se necessário)
    ENV PORT=3000

    # Executar a aplicação
    CMD ["./check_downloadable"]
