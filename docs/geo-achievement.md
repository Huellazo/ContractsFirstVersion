# Geo Achievement

Registro NFT-like de la visita a un landmark. Se "mintea" (como cuenta PDA) cuando el usuario está físicamente en el lugar, verificado por el backend.

## Cuenta

**Seed:** `["geo_ach", user, spot]`
**Espacio:** `8 + GeoAchievement::INIT_SPACE`

```rust
pub struct GeoAchievement {
    pub user: Pubkey,
    pub spot: Pubkey,
    pub timestamp: i64,
    pub location_proof: [u8; 64], // firma ed25519 del backend
    pub xp_earned: u64,
    pub shared_on_social: bool,
    pub bump: u8,
}
```

| Atributo | Tipo | Descripción |
|----------|------|-------------|
| `user` | `Pubkey` | Dueño del pasaporte. |
| `spot` | `Pubkey` | `TouristSpot` visitado. |
| `timestamp` | `i64` | Fecha de la visita. |
| `location_proof` | `[u8; 64]` | Firma del backend sobre `user ‖ spot ‖ ts ‖ lat ‖ lon`. |
| `xp_earned` | `u64` | XP total de esta visita (base). |
| `shared_on_social` | `bool` | Si ya fue compartido (bonus). |
| `bump` | `u8` | Bump del PDA. |

## Instrucciones

### `visit_landmark(timestamp, location_proof: [u8; 64])`
- **Signers:** `user`.
- **Cuentas:** `user`, `Config`, `TouristSpot` (mut), `Passport` (mut), `GeoAchievement` (init), `System`.
- **Lógica:**
  1. Verifica `spot.is_active`.
  2. Reconstruye el mensaje: `user ‖ spot ‖ timestamp ‖ latitude ‖ longitude`.
  3. Verifica `location_proof` con `Config.backend_admin` usando `ed25519-dalek`.
  4. Crea el `GeoAchievement` (PDA único ⇒ no se puede revisitar el mismo spot).
  5. `xp = base_xp_reward.max(1000)`.
  6. `apply_xp(passport, xp)`, `stamps += 1`, `last_spot = spot`, `total_visitors += 1`.

### `share_achievement()`
- **Signers:** `user`.
- Marca `shared_on_social = true` (una sola vez), otorga `bonus = xp_earned × 10 %` al pasaporte.

## Anti-spoofing

El contrato **no** confía en el GPS del teléfono. Flujo:

1. App móvil envía `(user, spot_id, lat, lon, ts)` al backend.
2. Backend valida que `lat/lon` estén dentro de `spot.radius` del landmark.
3. Backend firma el payload con su clave privada ed25519.
4. App envía `visit_landmark` con esa firma.
5. El contrato verifica la firma on-chain contra `Config.backend_admin`.

Sin un backend confiable, la firma sola no prueba nada; la latencia y el replay se mitigan con `timestamp` en el mensaje firmado.

## Archivos

- `programs/first-project/src/state.rs` → `GeoAchievement`.
- `programs/first-project/src/instructions/visit_landmark.rs`.
- `programs/first-project/src/instructions/share_achievement.rs`.
- `programs/first-project/src/proof.rs` → `verify_location_proof`.