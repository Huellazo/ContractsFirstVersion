# Huellazo

Plataforma de turismo gamificado en Solana (estilo Pokémon GO) con dos pilares:

- **Misiones Principales (Landmarks):** visitar lugares icónicos → alto puntaje y logros coleccionables.
- **Misiones Secundarias (Comercios):** consumir en locales, pagar con la app, calificar y canjear descuentos → economía circular y fidelización.

Programa (`first_project`) construido con **Anchor 1.1.2** sobre **Surfpool**.

- **Program ID:** `88P3XfMfHcnWedUa1u5VJswc6eMQgTKCcBguHBDw6Ztv`
- **Red local:** Surfnet (`http://127.0.0.1:8899`)
- **Lenguaje:** Rust / Solana BPF
- **Frontend (WIP):** HTML local en `app/`

---

## Estructura

```
first-project/
├─ Anchor.toml              # config de Anchor (provider, scripts)
├─ Cargo.toml              # workspace
├─ programs/first-project/ # contrato (programa)
├─ app/                    # frontend HTML
├─ docs/                   # documentación de cuentas e instrucciones
├─ runbooks/               # IaC de despliegue (Surfpool txtx)
└─ .surfpool/              # logs / estado de Surfnet
```

---

## Documentación

Cada contrato/cuenta tiene su propia doc. Accede desde aquí:

| Doc | Cuenta / Módulo | Descripción |
|-----|-----------------|-------------|
| [Config](docs/config.md) | `Config` | Admin y clave del backend (anti-spoofing) |
| [Passport](docs/passport.md) | `Passport` | Pasaporte del usuario (nivel, XP, sellos) |
| [Tourist Spot](docs/tourist-spot.md) | `TouristSpot` | Landmark geolocalizado ("Poképarada") |
| [Geo Achievement](docs/geo-achievement.md) | `GeoAchievement` | Registro de visita a un landmark |
| [Business Account](docs/business-account.md) | `BusinessAccount` | Comercio con promedio de calificaciones |
| [Business Mission](docs/business-mission.md) | `BusinessMission` | Misión secundaria (ruta de comercios) |
| [Business Check-in](docs/business-checkin.md) | `BusinessCheckIn` | Registro de consumo + pago + rating |
| [Business Progress](docs/business-progress.md) | `UserBusinessMissionProgress` | Progreso del usuario en una misión |
| [Discount](docs/discount.md) | `Discount` | Promoción publicada por un comercio |

Flujos completos y seguridad en [docs/flows.md](docs/flows.md).

---

## Requisitos

```bash
# toolchain (ya presente en este repo)
rustc 1.89.0
anchor-cli 1.1.2
solana-cli 3.1.10
surfpool 1.4.0
```

Instalación de Surfpool (si no está):

```bash
curl -sL https://run.surfpool.run/ | bash
```

---

## Build

```bash
anchor build
```

Genera:

- `target/deploy/first_project.so` → binario BPF
- `target/idl/first_project.json` → IDL (usado por el frontend)
- `target/types/first_project.ts` → tipos TypeScript

---

## Tests

Test unitario en `programs/first-project/tests/test_initialize.rs` (usa `litesvm`):

```bash
cargo test --package first-project --test test_initialize
```

Cubre el flujo completo: init config → passport → tourist spot → business → mission → discount → check-in (descuento + misión).

---

## Despliegue

### 1) Levantar Surfnet

```bash
surfpool start --watch
```

- `--watch`: recompila y redespliega el programa automáticamente al cambiar el código.
- RPC local: `http://127.0.0.1:8899`

### 2) Desplegar via Runbook (Surfpool IaC)

```bash
surfpool run deployment
```

El runbook está en `runbooks/deployment/main.tx` y despliega el programa con `authority`/`payer` (ver `signers.localnet.tx`). Para devnet/mainnet copia `signers.*.tx.example` a `signers.{network}.tx` y completa las keys.

### 3) Listar runbooks

```bash
surfpool ls
```

> **Importante:** nunca commitees `signers.*.tx` (claves privadas). El `.gitignore` ya los excluye.

---

## Uso rápido (en Surfnet)

1. `init_config(backend_admin)` — solo la primera vez. El `backend_admin` es la Pubkey del backend que firmará el GPS.
2. `initialize_passport()` — cada usuario lo hace una vez.
3. `create_tourist_spot(...)` — solo admin.
4. `visit_landmark(timestamp, location_proof)` — el `location_proof` es la firma ed25519 del backend sobre `user || spot || timestamp || lat || lon`.
5. `share_achievement()` — bonus +10 % XP.
6. `create_business(name)` — el comercio crea su cuenta.
7. `create_business_mission(...)` y `create_discount(...)` — el comercio publica misiones y descuentos.
8. `check_in_business(amount_spent, rating, use_discount, discount_id, mission_id, timestamp)` — transfiere el pago, actualiza rating, otorga XP y avanza misiones.

---

## Seguridad

- **Anti-spoofing GPS:** el contrato NO confía en el GPS del teléfono. Solo acepta `location_proof` firmada por `Config.backend_admin` (verificado on-chain con `ed25519-dalek`).
- **Revisitas:** el PDA `GeoAchievement` es único por `(user, spot)` — no se puede mintear dos veces el mismo logro.
- **Descuentos:** uso limitado por `available_units`; al agotarse, `is_active = false`.
- **Misiones:** `expires_at` y `is_active` validados en cada check-in; no se puede completar dos veces.
- **Listas `Vec`** limitadas a 10 (`MAX_TARGET_BUSINESSES`) para evitar desbordes de heap.
- **Overflow checks** activados (`overflow-checks = true` en release).

Más detalle en [docs/flows.md](docs/flows.md).

---

## Frontend

`app/` contendrá el HTML local. Consumirá `target/idl/first_project.json` vía `@coral-xyz/anchor` (browser). WIP.

---

## Recursos

- [Surfpool docs](https://docs.surfpool.run/)
- [Anchor CLI basics](https://solana.com/es/docs/intro/installation/anchor-cli-basics)
- [Surfpool CLI basics](https://solana.com/es/docs/intro/installation/surfpool-cli-basics)