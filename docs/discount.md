# Discount

Promoción/oferta publicada por un comercio. El usuario "desbloquea" el descuento al hacer check-in.

## Cuenta

**Seed:** `["discount", business, id]` (business = authority del comercio, id `u64` LE)
**Espacio:** `8 + Discount::INIT_SPACE`

```rust
pub struct Discount {
    pub id: u64,
    pub business: Pubkey,
    pub code: String,           // max 32, opcional
    pub discount_percent: u8,  // 0..=100
    pub min_purchase: u64,     // monto mínimo (lamports)
    pub available_units: u32, // 0 = ilimitado
    pub unlimited: bool,       // = available_units == 0
    pub valid_until: i64,      // 0 = sin vencimiento
    pub is_active: bool,
    pub bump: u8,
}
```

| Atributo | Tipo | Descripción |
|----------|------|-------------|
| `id` | `u64` | Identificador (seed). |
| `business` | `Pubkey` | Comercio ofertante (`authority`). |
| `code` | `String` (≤32) | Código promocional (opcional). |
| `discount_percent` | `u8` | % de descuento (≤100). |
| `min_purchase` | `u64` | Monto mínimo para aplicar. |
| `available_units` | `u32` | Cupos restantes. `0` + `unlimited=true` ⇒ sin límite. |
| `unlimited` | `bool` | `true` si se creó con 0 unidades. |
| `valid_until` | `i64` | Vencimiento (0 = nunca). |
| `is_active` | `bool` | Disponibilidad. |
| `bump` | `u8` | Bump del PDA. |

## Instrucción

### `create_discount(id, code, discount_percent, min_purchase, available_units, valid_until)`
- **Signers:** `business` (authority del comercio).
- **Cuentas:** `business` (signer), `business_account` (PDA, valida authority), `discount` (init), `System`.
- Valida `code.len() ≤ 32`, `discount_percent ≤ 100`.
- `unlimited = (available_units == 0)`.

## Canje

El canje **no** es una instrucción standalone; ocurre dentro de `check_in_business` cuando `use_discount = true`:

1. Verifica `discount.is_active`.
2. Verifica `valid_until > timestamp` (si `valid_until ≠ 0`).
3. Verifica `amount_spent >= min_purchase`.
4. Si no `unlimited`: `available_units > 0` y luego `available_units -= 1`.
5. Si `available_units == 0` tras restar → `is_active = false`.
6. Registra `discount_applied` en el `BusinessCheckIn`.

> El contrato **no** calcula el precio final; solo valida y registra. El frontend aplica el % al cobro.

## Archivos

- `programs/first-project/src/state.rs` → `Discount`.
- `programs/first-project/src/instructions/create_discount.rs`.
- Lógica de canje en `programs/first-project/src/instructions/check_in_business.rs`.