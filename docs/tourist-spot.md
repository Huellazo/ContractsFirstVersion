# Tourist Spot

Landmark geolocalizado — la "Poképarada". Cada punto de interés tiene su PDA verificable por su ID.

## Cuenta

**Seed:** `["tourist_spot", id]` (id como `u64` little-endian)
**Espacio:** `8 + TouristSpot::INIT_SPACE`

```rust
pub struct TouristSpot {
    pub id: u64,
    pub authority: Pubkey,
    pub name: String,         // max 64
    pub latitude: i64,        // escalada (×10^7), sin floats
    pub longitude: i64,       // escalada (×10^7)
    pub radius: u32,          // radio de validez en metros
    pub base_xp_reward: u64,  // XP base (alto, ej. 1000)
    pub achievement_uri: String, // URI IPFS/Arweave de la medalla
    pub total_visitors: u64,
    pub is_active: bool,
}
```

| Atributo | Tipo | Descripción |
|----------|------|-------------|
| `id` | `u64` | Identificador único (seed). |
| `authority` | `Pubkey` | Admin que creó el spot. |
| `name` | `String` (≤64) | Nombre del lugar. |
| `latitude` | `i64` | Latitud ×10⁷ (ej. 4888588880 para 48.85888°N). |
| `longitude` | `i64` | Longitud ×10⁷. |
| `radius` | `u32` | Radio de validez (m). |
| `base_xp_reward` | `u64` | XP base otorgado al visitar. |
| `achievement_uri` | `String` (≤128) | URI de la imagen/medalla. |
| `total_visitors` | `u64` | Contador global. |
| `is_active` | `bool` | Disponibilidad. |

## Instrucciones

### `create_tourist_spot(id, name, latitude, longitude, radius, base_xp_reward, achievement_uri)`
- **Signers:** `admin` (config.admin).
- Valida `name.len() ≤ 64`, `achievement_uri.len() ≤ 128`.
- Crea el spot activo.

### `set_spot_active(active: bool)`
- **Signers:** `admin`.
- Activa/desactiva el spot (impide nuevas visitas).

## Notas

- La verificación de que el usuario está físicamente dentro de `radius` se hace **off-chain en el backend**; el contrato solo verifica la firma del backend (`location_proof`) en `visit_landmark`.
- Las coordenadas escaladas evitan floats (incompatibles con determinismo on-chain).

## Archivos

- `programs/first-project/src/state.rs` → `TouristSpot`.
- `programs/first-project/src/instructions/create_tourist_spot.rs`.