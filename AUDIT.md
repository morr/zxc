# ZXC Codebase Audit: Flaws & Bad Coding Habits

## Context

Full audit of the ZXC Bevy 0.18 game project to identify flaws, bad coding habits, and Bevy antipatterns. The project is a 2D top-down simulation with pawns, farming, pathfinding, and an AI task system.

---

## Previously Fixed

---

## Previously Evaluated — WON'T FIX

---

## Open Issues

---

### [DESIGN] AI priority order starves farming at high time scale

**Files:** `src/ai/mod.rs:38–45`, `src/feedable/mod.rs:78–90`, `src/restable/mod.rs:100–112`

**Severity:** High — game becomes unplayable at ~50x+ time scale

#### Problem

The AI priority order in `ai_idle_pawns` is:
1. **Feed** (hunger >= 100 AND food > 0)
2. **ToRest** (fatigue >= 100)
3. **Work** (task queue)
4. **Wander**

Both `progress_hunger` and `progress_fatigue` use a threshold-crossing pattern:

```rust
// feedable
if wasnt_overflowed && feedable.is_overflowed() && food_stock.amount > 0 {
    commandable.set_queue(FeedCommand, ...);
}

// restable
if wasnt_overflowed && restable.is_overflowed() {
    commandable.set_queue(ToRestCommand, ...);
}
```

These fire once per overflow event and then go silent (value is clamped at 100).
After that, `ai_idle_pawns` is the sole driver of survival commands.

At high time scales virtual delta per real frame is large (300x → ~5 virtual sec/frame).
Survival overflows happen every ~24 virtual seconds (hunger) and ~17 virtual seconds
(fatigue). The pawn cycles Feed → ToRest → Feed → ToRest continuously, never reaching
the Work branch. All 10 starting food units are consumed reactively. No farming occurs.
Once food hits 0 the pawn either starves during sleep (hunger accumulates while ToRest
cycle runs, hitting 500 before sleep completes) or hits the deadlock in Bug 1 above.

**Concrete numbers at 300x with config defaults:**
- Hunger overflow every ~5 real frames (0.083s), 10 food units last ~50 real frames (0.83s)
- After food exhaustion, death by starvation occurs ~24 real frames (0.4s) later unless the pawn is in an active sleep cycle

#### Fix options

**Option A — Proactive need anticipation (recommended):** Track time-since-last-fed and
time-since-last-rested. When pawn is Idle with sufficient reserves (e.g. hunger < 50,
fatigue < 50), assign Work regardless of near-future overflow. Reserve one "safety eat" and
"safety rest" cycle before the next expected overflow.

**Option B — Separate survival commands from AI:** Keep `progress_hunger` and
`progress_fatigue` as the sole drivers for Feed/ToRest. Remove those branches from
`ai_idle_pawns` so it only assigns Work and Wander. Survival commands would interrupt
work via `set_queue` (which already calls `drain_queue`, re-queuing tasks via
`ReleaseCommandResourcesEvent`/`CompleteTask`). This is simpler but requires Bug 1
to be fixed first, and requires `progress_*` systems to query all entities regardless
of pawn state (they already do — no `PawnStateIdleTag` filter).

**Option C — Threshold hysteresis:** Only queue Feed/ToRest if hunger/fatigue is above
threshold AND has been there for > N frames. Prevents rapid re-trigger cycles that crowd
out work at high time scales.

Any fix for this issue should be preceded by fixing Bug 1 above — a FeedCommand reaching
food = 0 will deadlock the pawn regardless of how well work is scheduled.

