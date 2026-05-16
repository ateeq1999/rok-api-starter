# rok-api-test

Production-ready REST API starter built on the [rok](https://rok-rs.dev) ecosystem — Axum + SQLx + PostgreSQL.

## Stack

| Layer | Crate |
|-------|-------|
| HTTP  | [Axum 0.8](https://docs.rs/axum/0.8) |
| ORM   | [rok-orm 0.1](https://crates.io/crates/rok-orm) (Postgres) |
| Auth  | [rok-auth 0.1](https://crates.io/crates/rok-auth) (JWT + Magic Link) |
| DB    | [SQLx 0.8](https://docs.rs/sqlx/0.8) / PostgreSQL |
| Validation | [rok-validate 0.1](https://crates.io/crates/rok-validate) |
| Config | [rok-config 0.1](https://crates.io/crates/rok-config) (env-driven) |
| Testing | [rok-testing 0.1](https://crates.io/crates/rok-testing) |

## Quick start

### Option A — Docker (recommended)

```bash
cp .env.example .env

# Start Postgres + run migrations + boot the API
make up
make migrate

# Or step by step:
docker compose up -d postgres
docker compose run --rm db-migrate
docker compose up -d app

# Tail logs
make log

# Stop everything
make down
```

### Option B — Local development

```bash
# 1. Start Postgres (or use docker-compose for just the DB)
docker compose up -d postgres

# 2. Environment
cp .env.example .env

# 3. Apply migrations
make migrate-dev
# or: bash database/migrate.sh

# 4. Run
cargo run
# Listening on http://0.0.0.0:3000

# 5. Test
cargo test
```

## Endpoints

### Auth (`/auth`)

| Method | Path | Body | Auth | Description |
|--------|------|------|------|-------------|
| POST | `/auth/register` | `{ email, password, password_confirmation }` | — | Register & return JWT |
| POST | `/auth/login` | `{ email, password }` | — | Login & return JWT |
| POST | `/auth/logout` | — | Bearer | Invalidate session |
| GET | `/auth/me` | — | Bearer | Current user profile |
| POST | `/auth/forgot-password` | `{ email }` | — | Request reset link |
| POST | `/auth/reset-password` | `{ token, password, password_confirmation }` | — | Reset password |
| POST | `/auth/magic-link` | `{ email }` | — | Request magic link |
| GET | `/auth/magic-link/callback` | `?token=...` | — | Verify magic link & login |

### Admin — Users (`/api/v1/users`)

| Method | Path | Body | Auth | Description |
|--------|------|------|------|-------------|
| GET | `/api/v1/users` | — | Bearer | List all users |
| POST | `/api/v1/users` | `{ email, password }` | Bearer | Create user |
| GET | `/api/v1/users/{id}` | — | Bearer | Get user by ID |
| PUT | `/api/v1/users/{id}` | `{ email? }` | Bearer | Update user |
| DELETE | `/api/v1/users/{id}` | — | Bearer | Delete user |

## Example requests

```bash
# Register
curl -s http://localhost:3000/auth/register \
  -H 'Content-Type: application/json' \
  -d '{"email":"alice@example.com","password":"secret123","password_confirmation":"secret123"}' | jq

# Login
curl -s http://localhost:3000/auth/login \
  -H 'Content-Type: application/json' \
  -d '{"email":"alice@example.com","password":"secret123"}' | jq

# Token=$(curl -s ... | jq -r '.token')
TOKEN="eyJ..."

# Profile
curl -s http://localhost:3000/auth/me \
  -H "Authorization: Bearer $TOKEN" | jq
```

## Configuration (environment variables)

| Variable | Default | Description |
|----------|---------|-------------|
| `APP_NAME` | — | Application name |
| `LISTEN_ADDR` | `0.0.0.0:3000` | Bind address |
| `DATABASE_URL` | — | PostgreSQL connection string |
| `DB_MAX_CONNECTIONS` | `10` | Connection pool size |
| `JWT_SECRET` | — | JWT signing key |

## Docker Compose services

| Service | Image | Description |
|---------|-------|-------------|
| `postgres` | `postgres:16-alpine` | Database with persistent volume (`pgdata`) |
| `app` | _built_ | The Axum API server |
| `db-migrate` | _built_ | One-shot migration runner (exit after applying) |

```bash
# Available commands
make up        # docker compose up -d
make down      # docker compose down
make build     # docker compose build app
make migrate   # docker compose run --rm db-migrate
make migrate-dev # Run migrations against local Postgres
make log       # docker compose logs -f app
make dev       # cargo run
```

## Project structure

```
src/
├── main.rs              # Server bootstrap
├── lib.rs               # Library root
├── config/              # Env-driven config (rok-config)
│   ├── app.rs
│   ├── auth.rs
│   └── database.rs
├── state.rs             # AppState — shared pool + auth handle
├── routes/              # Router definitions
│   ├── mod.rs
│   ├── auth.rs
│   └── api.rs
└── app/
    ├── controllers/     # Request handlers
    ├── models/          # ORM models (rok-orm)
    └── validators/      # Request validation (rok-validate)
database/
└── migrations/          # SQL migration files
tests/
├── common/              # Test helpers
└── auth.rs              # Integration tests
```

## Architecture notes

- **Task-local pool** — `OrmLayer` scopes the database connection pool via `tokio::task_local`. Model queries like `User::filter(...).first().await` resolve the pool automatically without passing it explicitly. In tests use `rok_orm::pool::with_pool()`.
- **State vs Extensions** — Handlers that need the pool directly use `State<AppState>`. Handlers using `Ctx` (from `rok-auth::axum`) resolve the pool through the task-local scope set by `OrmLayer`.
- **Validation** — Request bodies use `rok_validate::Valid` extractor with `#[derive(Validate)]` for declarative validation.
