# Business Check-in

Registro unificado de visita + pago + calificación en un comercio. Se crea en cada `check_in_business`.

## Cuenta

**Seed:** `["biz_checkin", user, business_wallet, timestamp]` (timestamp `i64` LE)
**Espacio:** `8 + BusinessCheckIn::INIT_SPACE`

```rust
pub struct BusinessCheckIn {
    pub user: Pubkey,
    pub business: Pubkey,
    pub timestamp: i64,
    pub amount_spent: u64,        // lamports
    pub rating: u8,               // 1..=5
    pub xp_earned: u64,
    pub used_discount: bool,
    pub discount_applied: u8,    // % aplicado (0 si ninguno)
    pub bump: u8,
}
```

| Atributo | Tipo | Descripción |
|----------|------|-------------|
| `user` | `Pubkey` | Turista. |
| `business` | `Pubkey` | Wallet del comercio. |
| `timestamp` | `i64` | Hora del check-in. |
| `amount_spent` | `u64` | Monto transferido (lamports). |
| `rating` | `u8` | Calificación 1..=5. |
| `xp_earned` | `u64` | XP otorgado (base + bonus misión si aplica). |
| `used_discount` | `bool` | Si canjeó un `Discount`. |
| `discount_applied` | `u8` | % de descuento aplicado. |
| `bump` | `u8` | Bump del PDA. |

## Instrucción

### `check_in_business(amount_spent, rating, use_discount, discount_id: Option<u64>, mission_id: Option<u64>, timestamp)`

**Cuentas:**

| Cuenta | Mut | Descripción |
|--------|-----|-------------|
| `user` | ✓ | Signer, paga. |
| `business_wallet` | ✓ | Recibe el pago. Requiere `== business_account.authority`. |
| `business_account` | ✓ | PDA del comercio (rating). |
| `passport` | ✓ | PDA del usuario (XP). |
| `check_in` | init | Nuevo PDA. |
| `discount` | ✓ opcional | PDA del descuento (si `use_discount`). |
| `mission` | opcional | PDA de la misión (si `mission_id`). |
| `mission_progress` | init_if_needed | PDA del progreso. |
| `system_program` | | Transfer SOL. |

**Lógica en orden:**

1. **Valida rating** (1..=5) y `business_account.is_active`.
2. **Transfer** SOL `user → business_wallet` (CPI `system_program::transfer`).
3. **Descuento** (si `use_discount`):
   - Verifica `discount.is_active`, no expirado, `amount_spent >= min_purchase`, cupos > 0.
   - `discount_applied = discount_percent`; si no ilimitado: `available_units -= 1` (y `is_active=false` al agotar).
4. **Misión** (si `mission_id`):
   - Verifica activa, no expirada, `business_wallet ∈ target_businesses`.
   - `mission_progress.check_ins += 1`; si ≥ `required_check_ins` → `completed = true` + `xp += MISSION_BONUS_XP` (500).
5. **XP:** `apply_xp(passport, xp)` (base 100 + bonus).
6. **Rating:** `business_account.rating_sum += rating`, `rating_count += 1`.
7. **Guarda** el `BusinessCheckIn`.

> El timestamp va en la seed ⇒ un usuario puede hacer múltiples check-ins al mismo comercio en distintos momentos (cada uno con timestamp distinto).

## Filosofía

- **Fricción cero:** una sola tx/firma del usuario hace pago + rating + descuento + misión.
- El contrato **registra** el descuento usado; el frontend aplica el precio final visual.

## Archivos

- `programs/first-project/src/state.rs` → `BusinessCheckIn`.
- `programs/first-project/src/instructions/check_in_business.rs`.