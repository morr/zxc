---
name: live-app
description: Use when a change has to be verified in the actually running game, not just by cargo test — launching ZXC in the background, querying the live world over Bevy Remote Protocol (http://127.0.0.1:15702), driving it via resource mutation, reading its trace log, and shutting it down. Covers the post-refactor smoke-test loop.
---

# Running ZXC and inspecting it live

`main.rs` registers `RemotePlugin` + `RemoteHttpPlugin`, so every run exposes a
JSON-RPC 2.0 endpoint on `http://127.0.0.1:15702/`. Combined with the trace-level
log on stdout, that is enough to verify behaviour without touching the window.

## When to use this

- After a refactor that compiles and passes tests but could break at runtime
  (observer/event migrations, state-tag hooks, plugin registration, UI spawns).
- To confirm entities actually get spawned / states actually transition.
- To inspect real values of `bevy_*` components (Transform, Node, ZIndex, Text).

Do **not** boot the game for something a unit test in `tests/` already covers.

## 1. Launch

```
Bash(command: "cargo run 2>&1", run_in_background: true)
```

- **Always background.** Foreground blocks until the window is closed.
- The tool reply gives the output file:
  `/private/tmp/claude-501/-Users-morr-develop-bevy-zxc/<session>/tasks/<task_id>.output`.
  Everything below greps that file — save the path.
- `2>&1` matters: Bevy logs to stderr.
- Warm build → window in ~5 s. Cold / dependency change → 3–6 min; poll the
  output file instead of guessing.
- **One instance at a time.** A second process cannot bind 15702 and its BRP
  calls silently answer from the *old* game. Stop the previous task first.

## 2. Waiting

Foreground `sleep` is blocked by the harness. Use:

```bash
perl -e 'select(undef,undef,undef,12)'   # 12 s
```

12–15 s after launch is enough for asset loading (`AppState::Loading` →
`Playing`) plus a few seconds of simulation. For behaviour that needs game time
(farming, hunger, rest) wait longer or raise the time scale (§6).

## 3. Health check first

```bash
f=/private/tmp/.../tasks/<task_id>.output
grep -cE 'panic|ERROR' $f
tail -20 $f
```

A build error also lands in this file — a zero-byte or `error[E` output means
nothing is listening on 15702, so don't waste turns curling.

## 4. BRP calls

```bash
brp() { curl -s -m 5 -X POST http://127.0.0.1:15702/ \
  -H 'Content-Type: application/json' \
  -d "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"$1\",\"params\":$2}"; }
```

Or use the bundled helper: `.claude/skills/live-app/scripts/brp.sh world.query '{...}'`.

**Bevy 0.19 renamed every method** (`bevy/query` → `world.query`). Current set —
`rpc.discover` re-prints it from the running app:

| Method | Params |
|---|---|
| `world.query` | `{"data":{"components":[],"option":[],"has":[]},"filter":{"with":[],"without":[]},"strict":false}` |
| `world.get_components` | `{"entity":N,"components":[..],"strict":false}` |
| `world.list_components` | *omit params entirely* → all reflect-registered types; `{"entity":N}` → the components on that entity |
| `world.spawn_entity` / `world.despawn_entity` / `world.insert_components` / `world.remove_components` / `world.mutate_components` / `world.reparent_entities` | see `rpc.discover` |
| `world.list_resources` / `world.get_resources` `{"resource":".."}` | |
| `world.insert_resources` / `world.remove_resources` / `world.mutate_resources` `{"resource":..,"path":".field.sub","value":..}` | |
| `world.trigger_event` `{"event":..,"value":..}` / `world.write_message` | registered reflect events only |
| `world.get_components+watch`, `world.list_components+watch`, `world.observe+watch` | streaming; need `curl -N` |
| `registry.schema` `{"with_crates":[..],"without_crates":[..]}` | |
| `schedule.list`, `schedule.graph` `{"schedule_label":"Update"}` | |

Names are full reflect type paths: `zxc::pawn::components::Pawn`,
`bevy_ui::widget::text::Text`, `bevy_time::time::Time<bevy_time::virt::Virtual>`.

Responses are `{"result":...}` **or** `{"error":{"code":..,"message":..}}` — always
handle both, or the parsing python throws `KeyError`/`NoneType` and hides the
real message:

```bash
| python3 -c "import json,sys; d=json.load(sys.stdin); r=d.get('result'); print(len(r)) if r is not None else print(d['error']['message'])"
```

## 5. What is actually readable — read this before believing an empty result

ZXC's own components mostly do **not** derive `Reflect`. Consequences:

- `world.query` on `zxc::…` **counts** entities but returns
  `"components": {}` — values are not readable.
- `world.get_components` on them returns `errors: {… "isn't reflectable"}`.
- `filter.with` / `filter.without` **do** work with non-reflect components, so
  state tags are fully usable for counting (see recipes).
- `world.list_components` *without params* lists only reflect-registered types
  (~307, none of them `zxc::`). To learn a real ZXC type path, list the
  components of a known entity instead — that returns them all.
- `"strict": true` turns an unknown/unreflectable name into an error instead of
  an empty result. Use it whenever "0 rows" would be a meaningful answer, so a
  typo in the type path can't masquerade as "feature broken".
- `world.trigger_event` fails with `Unknown event type` for ZXC events — they are
  not in the reflect event registry. Verify events through the log (§7), not BRP.

Bevy's own components are fully readable — that is the fast path for UI/transform
assertions.

## 6. Recipes

Count entities of a type (the workhorse):

```bash
brp world.query '{"data":{"components":["zxc::carryable::components::Carryable"]},"strict":true}' \
  | python3 -c "import json,sys; print('carryables:', len(json.load(sys.stdin).get('result',[])))"
```

Count by state tag — verifies a state machine actually moves:

```bash
for tag in PawnStateIdleTag PawnStateExecutingCommandTag PawnStateRestingTag; do
  printf '%-32s' $tag
  brp world.query "{\"data\":{\"components\":[\"zxc::pawn::components::Pawn\"]},\"filter\":{\"with\":[\"zxc::pawn::components::pawn_state::$tag\"]}}" \
    | python3 -c "import json,sys; d=json.load(sys.stdin); r=d.get('result'); print(len(r) if r is not None else d['error']['message'][:60])"
done
```

Discover the exact type paths on a live entity (grab an entity id from any query):

```bash
brp world.list_components '{"entity":4294966761}' | python3 -m json.tool | grep zxc
```

Read a Bevy component value — e.g. assert the FPS overlay offset:

```bash
brp world.query '{"data":{"components":["bevy_ui::ui_node::GlobalZIndex","bevy_ui::ui_node::Node"]}}' | python3 -c "
import json,sys
for row in json.load(sys.stdin).get('result',[]):
    c=row['components']
    if c['bevy_ui::ui_node::GlobalZIndex']==2147483615:
        print('overlay left', c['bevy_ui::ui_node::Node'].get('left'), 'top', c['bevy_ui::ui_node::Node'].get('top'))
"
```

Read / drive game time — the cheapest way to fast-forward a simulation test:

```bash
brp world.get_resources '{"resource":"bevy_time::time::Time<bevy_time::virt::Virtual>"}'
# → {"value":{"context":{"relative_speed":25.0,"paused":true,...},"elapsed_secs":428.7,...}}

brp world.mutate_resources '{"resource":"bevy_time::time::Time<bevy_time::virt::Virtual>","path":".context.relative_speed","value":100.0}'
brp world.mutate_resources '{"resource":"bevy_time::time::Time<bevy_time::virt::Virtual>","path":".context.paused","value":false}'
```

Restore the value afterwards if the run continues — the user may be watching the
same window.

Avoid unbounded dumps: `Transform` alone matches ~289 000 entities (tilemap).
Always aggregate in python; never pipe a full query into the transcript.

## 7. Log-based verification (usually stronger than BRP here)

`LogPlugin` runs with `filter: "info,zxc=trace"`, and `log_state_change!` /
`log_event!` emit every transition and every event. Counting log lines proves the
things BRP cannot see:

```bash
f=/private/tmp/.../tasks/<task_id>.output
echo "panics:          $(grep -cE 'panic|ERROR' $f)"
echo "tasks scheduled: $(grep -c 'ScheduleTaskEvent' $f)"
echo "work started:    $(grep -c 'Idle => BeingWorked' $f)"
echo "commands done:   $(grep -c 'CommandCompleteEvent' $f)"
```

Useful shapes: `Event <Name>` (from `log_event!`), `<From> => <To>` (from
`log_state_change!`). To see what a healthy run looks like, histogram it:

```bash
grep -oE 'Event [A-Za-z]+' $f | sort | uniq -c | sort -rn | head
grep -oE '=> [A-Za-z]+'     $f | sort | uniq -c | sort -rn | head
```

A healthy ~30 s run shows hundreds of `EntityStateChangeEvent`, dozens of
`MoveToCommand` / `CommandCompleteEvent`, and non-zero `=> Idle`. All-zero
counters after a refactor mean observers stopped firing even though the build is
green.

## 8. One-shot smoke test

Launch in one call, then a single verification call that waits, greps and queries:

```bash
perl -e 'select(undef,undef,undef,15)'
f=/private/tmp/.../tasks/<task_id>.output
echo "panics: $(grep -cE 'panic|ERROR' $f)"
echo "tasks scheduled: $(grep -c 'ScheduleTaskEvent' $f)"
echo "work started: $(grep -c 'Idle => BeingWorked' $f)"
brp world.query '{"data":{"components":["zxc::carryable::components::Carryable"]},"strict":true}' \
  | python3 -c "import json,sys; print('carryables in world:', len(json.load(sys.stdin).get('result',[])))"
```

## 9. Shutdown

```
TaskStop(task_id: "<task_id>")
```

Stop the game before rebuilding or relaunching. Leaving it running holds 15702,
keeps a window on the user's screen, and makes the next run's BRP answers come
from the stale process. If the user asked for the game to stay open, say so and
leave it.

## 10. Gotchas

- `cargo run` occasionally dies with `failed to build archive … libbevy_*.rlib:
  failed to open object file` — an artifact of `dynamic_linking` after an
  interrupted build. Re-run `cargo build`; it succeeds. A concurrent cargo shows
  as `Blocking waiting for file lock on build directory`.
- Entity ids come back as packed u64 (`4294966761`), not indices. Pass them back
  verbatim.
- `world.list_components` with `{}` errors `missing field 'entity'`; omit the
  `params` key entirely for the global list.
- BRP runs in the `RemoteLast` schedule — calls see world state between frames,
  so a value can be one frame stale. Never build a tight polling loop; wait, then
  ask once.
- `Time<Virtual>` is reported `paused: true` while the game window is unfocused;
  don't read that as a bug.
