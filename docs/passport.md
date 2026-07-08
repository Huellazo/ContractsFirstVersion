# Passport

Pasaporte digital del usuario. Acumula nivel, XP total y sellos (visitas a landmarks). Un PDA por usuario.

## Cuenta

**Seed:** `["passport", user]`
**Espacio:** `8 + Passport::INIT_SPACE`

```rust
pub struct Passport {
    pub owner: Pubkey,     // usuario dueño
    pub level: u64,        // nivel = total_xp / XP_PER_LEVEL
    pub total_xp: u64,     // XP acumulado
    pub stamps: u64,       // nº de landmarks visitados
    pub last_spot: Pubkey, // último TouristSpot visitado
    pub bump: u8,
}
```

| Atributo | Tipo | Descripción |
|----------|------|-------------|
| `owner` | `Pubkey` | Wallet del turista. |
| `level` | `u64` | Derivado: `total_xp / 1_000`. |
| `total_xp` | `u64` | XP total (landmark + business + bonus). |
| `stamps` | `u64` | Contador de `GeoAchievement`. |
| `last_spot` | `Pubkey` | Último spot visitado (`Pubkey::default()` si ninguno). |
| `bump` | `u8` | Bump del PDA. |

## Instrucciones

### `initialize_passport()`
- **Signers:** `user` (payer).
- Crea la cuenta. Inicializa todo en 0.

### Actualizaciones (internas)
- `visit_landmark` → `apply_xp(passport, xp)` + `stamps += 1` + `last_spot = spot`.
- `share_achievement` → `apply_xp(passport, bonus)`.
- `check_in_business` → `apply_xp(passport, business_xp)`.

## Progresión

Definida en `utils.rs`:

```rust
pub fn apply_xp(passport, xp) {
    passport.total_xp += xp;
    passport.level = total_xp / XP_PER_LEVEL; // 1_000
}
```

Entera, sin floats. `overflow-checks = true` en release.

## Archivos

- `programs/first-project/src/state.rs` → `Passport`.
- `programs/first-project/src/instructions/initialize_passport.rs`.
- `programs/first-project/src/utils.rs` → `apply_xp`, `compute_level`.