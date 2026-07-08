# Business Account

Comercio aliado que recibe pagos y check-ins. Guarda el promedio de calificaciones.

## Cuenta

**Seed:** `["business", authority]` (authority = wallet del comercio)
**Espacio:** `8 + BusinessAccount::INIT_SPACE`

```rust
pub struct BusinessAccount {
    pub authority: Pubkey,    // wallet del comercio (recibe pagos)
    pub name: String,          // max 64
    pub rating_sum: u64,       // suma de todos los ratings
    pub rating_count: u64,     // nº de check-ins con rating
    pub is_active: bool,
    pub bump: u8,
}
```

| Atributo | Tipo | Descripción |
|----------|------|-------------|
| `authority` | `Pubkey` | Wallet que recibe los pagos; valida `check_in_business`. |
| `name` | `String` (≤64) | Nombre del comercio. |
| `rating_sum` | `u64` | Σ ratings para el promedio. |
| `rating_count` | `u64` | Nº de ratings. |
| `is_active` | `bool` | Si acepta check-ins. |
| `bump` | `u8` | Bump del PDA. |

**Promedio (off-chain):** `rating_sum / rating_count`.

## Instrucciones

### `create_business(name)`
- **Signers:** `authority` (el propio comercio, payer).
- Crea la cuenta activa con `rating_sum = 0`, `rating_count = 0`.

### `set_business_active(active: bool)`
- **Signers:** `authority`.
- Activa/desactiva el comercio.

### Actualización (interna)
`check_in_business` suma el rating: `rating_sum += rating`, `rating_count += 1`.

## Seguridad

- La PDA está derivada de `authority`, así que el comercio no puede crear dos cuentas.
- En `check_in_business`, `business_wallet` (que recibe el pago) debe ser `== business_account.authority`.

## Archivos

- `programs/first-project/src/state.rs` → `BusinessAccount`.
- `programs/first-project/src/instructions/create_business.rs`.