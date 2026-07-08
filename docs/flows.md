# Flujos y Seguridad

Resumen de los flujos principales del programa Huellazo y las validaciones de seguridad.

---

## Flujo 1 — Visita a Landmark (misión principal)

```
Usuario (app)                 Backend (confiable)            Contrato (Solana)
    │                              │                              │
    │── POST (user, spot, lat, lon, ts) ──►                       │
    │                              │                              │
    │   (valida GPS dentro de spot.radius)                        │
    │                              │                              │
    │◄── firma ed25519(message) ───                              │
    │                              │                              │
    │── visit_landmark(ts, signature) ───────────────────────────►│
    │                              │                       (verifica ed25519-dalek contra backend_admin)
    │                              │                       (crea GeoAchievement, otorga XP, +1 stamp, +1 visitor)
    │                              │                              │
    │◄── confirmation ────────────────────────────────────────────│
```

**Mensaje firmado:** `user ‖ spot ‖ timestamp ‖ latitude ‖ longitude` (todos little-endian).

**PDA único** `["geo_ach", user, spot]` ⇒ el usuario no puede mintear dos veces el mismo logro.

---

## Flujo 2 — Share Achievement (viralidad)

```
share_achievement()
  → marca shared_on_social = true (una sola vez)
  → bonus = xp_earned × 10%   // share_bonus (BPS, entero)
  → apply_xp(passport, bonus)
```

`SHARE_BONUS_BPS = 1_000`, `BPS_DENOMINATOR = 10_000` ⇒ +10 %.

---

## Flujo 3 — Check-in en comercio + descuento + misión

```
check_in_business(amount_spent, rating, use_discount, discount_id, mission_id, timestamp)
    │
    ├─ valida rating ∈ [1,5] y business.is_active
    ├─ CPI system_program::transfer(user → business_wallet, amount_spent)
    │
    ├─ si use_discount:
    │    valida discount.is_active, no expirado, min_purchase, cupos
    │    discount_applied = percent
    │    available_units -= 1 (si no unlimited); is_active=false al agotar
    │
    ├─ si mission_id:
    │    valida mission.is_active, no expirada, business ∈ targets
    │    mission_progress.check_ins += 1
    │    si ≥ required_check_ins:
    │        completed = true
    │        xp += MISSION_BONUS_XP (500)
    │
    ├─ apply_xp(passport, xp)            // xp base = 100
    ├─ business.rating_sum += rating; rating_count += 1
    └─ guarda BusinessCheckIn
```

**Seed del check-in:** `["biz_checkin", user, business, timestamp]` ⇒ distintos timestamps = distintos check-ins.

---

## Mapa de PDAs

| Cuenta | Seed | Cardinalidad |
|--------|------|--------------|
| `Config` | `["config"]` | 1 global |
| `Passport` | `["passport", user]` | 1 por usuario |
| `TouristSpot` | `["tourist_spot", id]` | 1 por landmark |
| `GeoAchievement` | `["geo_ach", user, spot]` | 1 por (user, spot) |
| `BusinessAccount` | `["business", authority]` | 1 por comercio |
| `BusinessMission` | `["biz_mission", id]` | 1 por misión |
| `UserBusinessMissionProgress` | `["biz_progress", user, mission_id]` | 1 por (user, mission) |
| `BusinessCheckIn` | `["biz_checkin", user, business, ts]` | 1 por (user, business, momento) |
| `Discount` | `["discount", business, id]` | 1 por (comercio, descuento) |

---

## Constantes (`constants.rs`)

| Constante | Valor | Significado |
|-----------|-------|-------------|
| `XP_PER_LEVEL` | 1_000 | XP necesario por nivel |
| `SPOT_XP_DEFAULT` | 1_000 | XP mínimo por landmark |
| `BUSINESS_XP_DEFAULT` | 100 | XP por check-in de comercio |
| `MISSION_BONUS_XP` | 500 | Bonus al completar misión |
| `SHARE_BONUS_BPS` | 1_000 | 10 % (sobre 10_000) |
| `MAX_TARGET_BUSINESSES` | 10 | Limite de `target_businesses` |
| `RATING_MIN/MAX` | 1 / 5 | Rango válido de calificación |

---

## Seguridad

### Anti-spoofing GPS
El contrato **no** confía en el GPS del teléfono. Solo acepta `location_proof` firmada por `Config.backend_admin` (ed25519). `verify_location_proof` usa `ed25519-dalek` con verificación on-chain.

### Replays
`timestamp` va dentro del mensaje firmado. El backend debe rechazar timestamps viejos; el contrato valida la firma completa.

### Re-inicialización
- `Config` y `Passport` usan `init` (falla si ya existe).
- No hay instrucciones que reseteen estado sensible.

### Overflow
`overflow-checks = true` en `[profile.release]`. Cada `+=` crítico usa `checked_add` con error `Overflow`.

### Listas `Vec`
`target_businesses` ≤ 10 (`MAX_TARGET_BUSINESSES`). Evita desbordes de heap y costos altos.

### Autorización por PDA
- `business_wallet` (que recibe el pago) debe ser `== business_account.authority`.
- `update_backend_admin` y `create_tourist_spot` requieren `config.admin == signer`.

### Descuentos agotables
`available_units` baja en cada canje; `is_active = false` al llegar a 0. `unlimited = true` solo si se creó con 0.

### Misiones completables una vez
`UserBusinessMissionProgress.completed = true` bloquea nuevos bonus con `MissionAlreadyCompleted`.

---

## Atribución y cuidado

- Metadata pesada (imágenes, descripciones largas) → URIs IPFS/Arweave (`achievement_uri`, `code`).
- Fricción cero: una sola firma del usuario por visita/check-in; el programa firma sus propias actualizaciones de estado via PDAs.

## Archivos

- `programs/first-project/src/constants.rs`
- `programs/first-project/src/utils.rs` (apply_xp, share_bonus)
- `programs/first-project/src/proof.rs` (verify_location_proof)
- `programs/first-project/src/error.rs` (códigos de error)