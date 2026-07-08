# Config

Cuenta global de configuración del programa Huellazo. PDA única (una sola instancia).

## Cuenta

**Program:** `first_project`
**Seed:** `["config"]`
**Espacio:** `8 + Config::INIT_SPACE`

```rust
pub struct Config {
    pub admin: Pubkey,        // admin del programa (crea spots, etc.)
    pub backend_admin: Pubkey, // clave pública del backend que firma el GPS
    pub bump: u8,
}
```

| Atributo | Tipo | Descripción |
|----------|------|-------------|
| `admin` | `Pubkey` | Wallet que puede crear/desactivar TouristSpots y cambiar el backend_admin. |
| `backend_admin` | `Pubkey` | Pubkey del backend. Verifica `location_proof` en `visit_landmark`. |
| `bump` | `u8` | Bump del PDA. |

## Instrucciones

### `init_config(backend_admin: Pubkey)`
- **Signers:** `admin` (payer).
- Inicializa la única cuenta `Config`.
- `admin = ctx.accounts.admin`, `backend_admin = backend_admin`.

### `update_backend_admin(backend_admin: Pubkey)`
- **Signers:** `admin`.
- Requiere `config.admin == admin`.
- Rota la clave del backend (anti-spoofing).

## Seguridad

- PDA determinista `["config"]` → solo una instancia.
- `update_backend_admin` verificaQUIén es el admin antes de rotar.
- `backend_admin` se valida en cada `visit_landmark` con `ed25519-dalek`.

## Archivos

- `programs/first-project/src/state.rs` → struct `Config`.
- `programs/first-project/src/instructions/init_config.rs` → lógica.
- `programs/first-project/src/proof.rs` → verificación de firma.