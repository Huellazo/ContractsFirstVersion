# User Business Mission Progress

Progreso de un usuario en una [`BusinessMission`](business-mission.md). PDA por `(user, mission_id)`.

## Cuenta

**Seed:** `["biz_progress", user, mission_id]` (mission_id `u64` LE)
**Espacio:** `8 + UserBusinessMissionProgress::INIT_SPACE`

```rust
pub struct UserBusinessMissionProgress {
    pub user: Pubkey,
    pub mission: Pubkey,
    pub check_ins: u8,
    pub completed: bool,
    pub bump: u8,
}
```

| Atributo | Tipo | Descripción |
|----------|------|-------------|
| `user` | `Pubkey` | Turista. |
| `mission` | `Pubkey` | `BusinessMission` referenciada. |
| `check_ins` | `u8` | Nº de check-ins válidos en esta misión. |
| `completed` | `bool` | Si alcanzó `required_check_ins`. |
| `bump` | `u8` | Bump del PDA. |

## Creación

Se crea con `init_if_needed` dentro de `check_in_business` (cuando se pasa `mission_id`). No hay instrucción standalone.

## Actualización

En `check_in_business`:

```rust
progress.check_ins += 1;
if progress.check_ins >= mission.required_check_ins {
    progress.completed = true;
    xp += MISSION_BONUS_XP; // 500
}
```

## Validaciones

- Si `completed == true`, `check_in_business` la rechaza con `MissionAlreadyCompleted` (no se puede farmear bonus).
- `mission_id` en la seed ⇒ progreso aislado por misión.

## Archivos

- `programs/first-project/src/state.rs` → `UserBusinessMissionProgress`.
- `programs/first-project/src/instructions/check_in_business.rs` (init_if_needed + lógica).