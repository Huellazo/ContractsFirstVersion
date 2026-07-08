# Business Mission

Misión secundaria publicada por un comercio o grupo: "Visita N comercios aliados y obtén un descuento".

## Cuenta

**Seed:** `["biz_mission", id]` (id como `u64` LE)
**Espacio:** `8 + BusinessMission::INIT_SPACE`

```rust
pub struct BusinessMission {
    pub id: u64,
    pub business_group: Pubkey, // publicador (comercio o grupo)
    pub name: String,            // max 64
    pub required_check_ins: u8,  // nº de comercios a visitar
    pub target_businesses: Vec<Pubkey>, // max 10
    pub reward_discount: u8,     // % de descuento al completar
    pub expires_at: i64,         // 0 = sin expiración
    pub is_active: bool,
    pub bump: u8,
}
```

| Atributo | Tipo | Descripción |
|----------|------|-------------|
| `id` | `u64` | Identificador (seed). |
| `business_group` | `Pubkey` | Publicador de la misión. |
| `name` | `String` (≤64) | Ej. "Ruta del Café". |
| `required_check_ins` | `u8` | Comercios a visitar (≥1). |
| `target_businesses` | `Vec<Pubkey>` (≤10) | Comercios válidos para la misión. |
| `reward_discount` | `u8` | % de descuento al completar. |
| `expires_at` | `i64` | Timestamp de expiración (0 = sin límite). |
| `is_active` | `bool` | Estado. |
| `bump` | `u8` | Bump del PDA. |

## Instrucciones

### `create_business_mission(id, name, required_check_ins, target_businesses, reward_discount, expires_at)`
- **Signers:** `business_group` (payer).
- Valida `name ≤ 64`, `target_businesses.len() ≤ 10`, `required_check_ins > 0`.
- Crea la misión activa.

### Progreso
El avance se registra en [`UserBusinessMissionProgress`](business-progress.md), actualizado por `check_in_business`.

## Limites y validación

- `MAX_TARGET_BUSINESSES = 10` — evita desbordes de heap.
- `check_in_business` valida `is_active`, `expires_at > ts` (si ≠0) y que el comercio esté en `target_businesses`.
- Al llegar `check_ins >= required_check_ins`, `UserBusinessMissionProgress.completed = true` + bonus XP.

## Archivos

- `programs/first-project/src/state.rs` → `BusinessMission`.
- `programs/first-project/src/instructions/create_business_mission.rs`.