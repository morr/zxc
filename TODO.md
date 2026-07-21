# ТЗ: модернизация под идиомы Bevy 0.19

Проект уже мигрирован на Bevy 0.19 и компилируется без deprecated API. Задачи ниже — внедрение новых API 0.15–0.19, которые проект ещё не использует, плюс чистка легаси-хвостов.

Справка по API в каждом пункте выверена по исходникам bevy 0.19.0 из локального cargo registry (`~/.cargo/registry/src/index.crates.io-*/bevy_*-0.19.0/`) — при сомнениях смотреть туда, а не в память модели.

**Верификация после каждой задачи** (из CLAUDE.md):

```bash
cargo build --verbose
cargo test --verbose
cargo clippy -- -D warnings
# fmt только изменённых файлов
```

Задачи сгруппированы по фазам. Внутри фазы порядок рекомендованный, но задачи в основном независимы. Каждую задачу — отдельным коммитом (или серией коммитов для крупных). Вся работа — в `master`.

---

## Фаза 0 — чистка (быстрые, безрисковые)

### 0.1. Удалить закомментированный легаси-код Message-эпохи

- [x] Удалить 4 закомментированные строки `// .add_message::<...>()` (найти: `grep -rn 'add_message' src/`).
- [x] Удалить `// Camera2dBundle::default()` в `src/camera.rs:48`.
- [x] Дополнительно (найдено при выполнении): удалены все закомментированные функции и строки Message-эпохи с `MessageReader`/`MessageWriter`/`log_message!` — `update_pawn_color`, `pathfinding_on_click`, `on_release_resources` (work_on), `to_pathfinding`, старые `progress_work`/`start_pawn_working`/`complete_pawn_working` и ~10 одиночных строк-параметров. Отключённое перепланирование задач — осознанное решение (коммит `f72ea4d`), комментарии-напоминания не сохранялись.
- [x] **Критерий:** `grep -rn 'add_message\|Camera2dBundle' src/` — 0 совпадений (кроме `MessageWriter<AppExit>`, см. 0.6).

### 0.2. Удалить неиспользуемый макрос `log_message!`

- [x] Проверить перед удалением: `grep -rn 'log_message!' src/` — если есть живые вызовы, сначала перевести их на `log_event!` (`src/lib.rs:165`). _(живых вызовов не было — только в комментариях, удалены в 0.1)_
- [x] Удалить макрос `log_message!` (`src/lib.rs:156`) — остался от Message-эпохи, после конвертации в Event+Observer не используется.

### 0.3. Переименовать `render_debug_ui_window_node_bundle`

- [x] `src/ui/debug/mod.rs:33` — функция возвращает современный кортеж `(Node, BackgroundColor, T)`, слово `bundle` в имени вводит в заблуждение. Новое имя: `debug_ui_window_node` (или аналогичное без `bundle`). Переименовать через LSP rename, чтобы обновить все вызовы. _(переименовано через LSP, 3 места)_

### 0.4. `Val::Px` → хелперы `px()`/`percent()`

**Справка по API:** свободные функции в `bevy_ui::geometry` (доступны из prelude):

```rust
pub fn px<T: ValNum>(value: T) -> Val       // px(10) == Val::Px(10.0), принимает и int, и float
pub fn percent<T: ValNum>(value: T) -> Val  // percent(50) == Val::Percent(50.0)
// есть также auto() и вариации на GridTrack/UiRect и т.п.
```

- [x] Заменить 2 использования `Val::Px(..)` (статики в `src/ui/mod.rs:18-19`) на `px(..)`. _(НЕ ПРИМЕНИМО: `px()` — не `const fn`, а это статики с const-инициализатором. `Val::Px` в статиках — единственный вариант, оставлено как есть; см. «Решения»)_
- [x] Проверить остальной UI-код: `grep -rn 'Val::' src/` — привести к хелперам единообразно. _(остальной код уже использует `px()` — например `src/ui/debug/info.rs`)_

### 0.5. Исправить опечатку `DebugUiContainerarker`

- [x] `src/ui/debug/tasks_queue.rs:17`, `src/ui/debug/info.rs:35` и все прочие вхождения → `DebugUiContainerMarker`. Переименовать через LSP rename. _(переименовано через LSP, 4 места)_

### 0.6. НЕ ТРОГАТЬ: `MessageWriter<AppExit>`

`src/main.rs:96` — это корректный API 0.19 (`AppExit` — буферизованный Message). Оставить как есть; фиксируется здесь, чтобы не «дочистили» по ошибке. Чекбокса нет — делать ничего не нужно.

---

## Фаза 1 — ядро ECS (высокая ценность)

### 1.1. `#[require(...)]` для композиции Pawn

**Цель:** спавн пешки одним компонентом, невозможность «забыть компонент».

**Справка по API:** атрибут `#[require(...)]` на `#[derive(Component)]` (и на `#[derive(Resource)]`). Варианты синтаксиса:

```rust
#[derive(Component, Default)]
struct Health(f32);

#[derive(Component)]
#[require(
    Movable,                       // юнит/Default: вставит Movable::default()
    Health = Health(100.0),        // выражение, дающее значение компонента
    Sprite { custom_size: Some(Vec2::ONE), ..default() },  // struct-литерал
    Feedable(make_feedable()),     // tuple-struct / вызов конструктора
    SomeEnum::Variant,             // enum-вариант
)]
struct Pawn;
```

Required-компоненты вставляются рекурсивно (depth-first); дубликаты в одном атрибуте — ошибка компиляции. Если компонент уже указан в кортеже спавна явно — явное значение побеждает required-дефолт.

**Что сделать:**

- [x] На `Pawn` (`src/pawn/components.rs`) добавить `#[require(...)]`: `PawnStateIdleTag`, `Commandable`, `Restable`, `Feedable`, `Movable`, `Name`. _(`Carryable` на пешку не вешается — инвентарь это `HashMap` внутри `Pawn`, не компонент)_
- [x] Для компонентов с нетривиальными стартовыми значениями использовать value-форму: `Movable = Movable::new(config().pawn.speed * config().tile.size)`, `Name = Name::new("Pawn")`.
- [x] Упростить `spawn_pawns` (`src/pawn/systems.rs`): в кортеже остались только `Pawn::default()`, `Mesh2d`, `MeshMaterial2d`, `Transform` (зависят от ассетов/позиции).
- [x] Структуры: у `Warehouse`/`House`/`Well`/`Storage` require уже был — дополнен именованным `Name`; `Bed` получил `#[require(Name, Sprite, Transform)]`; `Farm` — `Name` + стартовый тег `farm_state::NotPlanted`. `Workable` фермы остаётся явным (значение зависит от состояния). Дубли `Name` из спавнов убраны.

**Критерии:**

- [x] Спавн пешки не перечисляет required-компоненты вручную.
- [x] Игра запускается, пешки ведут себя как раньше (смоук-тест: пешки спавнятся, берут задачи фермы, MoveTo/WorkOn выполняются, паник нет).
- [x] Тесты зелёные (29 passed), clippy чистый.

**Риски:** required-значения вычисляются при вставке компонента — убедиться, что `config()` уже инициализирован к моменту первого спавна (спавн идёт в `OnExit(AppState::Loading)`, конфиг грузится раньше — ок, но проверить тесты, где App собирается вручную).

### 1.2. Миграция entity-таргетированных событий на `EntityEvent`

**Цель:** использовать механизм таргетирования Bevy вместо ручной передачи `Entity` в payload; открыть дорогу entity-scoped наблюдателям и propagation.

**Контекст:** сейчас все ~40 событий — `#[derive(Event)]`, цель передаётся вручную: `EntityStateChangeEvent(pub Entity, pub T)` (`src/lib.rs:53`), все команды (`MoveToCommand`, `FeedCommand`, ...) несут `Entity` в поле.

**Справка по API:**

```rust
// Глобальное (нетаргетированное) событие — как сейчас:
#[derive(Event)]
struct NewDayEvent;
app.add_observer(|on: On<NewDayEvent>| { ... });
commands.trigger(NewDayEvent);

// Entity-таргетированное:
#[derive(EntityEvent)]
struct PawnDeathEvent { entity: Entity }       // поле по имени `entity`,
// или: struct PawnDeathEvent(Entity);         // или единственное поле,
// или: { #[event_target] pawn: Entity, ... }  // или помеченное атрибутом.
// Unit-структуры запрещены — target обязателен.

commands.trigger(PawnDeathEvent { entity });
app.add_observer(|death: On<PawnDeathEvent>, mut commands: Commands| {
    commands.entity(death.entity).despawn();   // On<E>: Deref<Target = E> — поля читаются напрямую
});
// entity-scoped: наблюдатель только для конкретной сущности:
commands.entity(e).observe(|death: On<PawnDeathEvent>| { ... });
```

Важно: у `On<E>` **нет** метода `.target()` — цель читается через поле события либо `EntityEvent::event_target(&self)`. Прочие методы `On`: `.event()`, `.event_mut()`, `.observer()`, `.original_event_target()`, `.propagate(bool)`.

Propagation (если понадобится всплытие по иерархии): `#[entity_event(propagate)]` — обход по `ChildOf` по умолчанию; `#[entity_event(propagate, auto_propagate)]` — автоматическое всплытие; кастомный обход: `propagate = &'static SomeRelation`.

**Что сделать:**

- [ ] Инвентаризация: разделить события на две группы.
  - **Entity-таргетированные** (есть целевая сущность): все `*Command` из `commandable/`, `EntityStateChangeEvent<T>`, `PawnDeathEvent`, `CommandCompleteEvent`, `InternalCommandInterruptEvent`/`ExternalCommandInterruptEvent`, `ReleaseCommandResourcesEvent`, `FarmProgressEvent`, `FarmTendedEvent`, `WorkCompleteEvent`, `RestCompleteEvent`, `FoodConsumedEvent` и т.п.
  - **Глобальные** (нет одной целевой сущности): `NewDayEvent`, `RebuildMapEvent`/`RebuildMapCompleteEvent`, `ScheduleTaskEvent`, `StateChangeEvent<T>`, `SpawnCarryableEvent` и т.п. — остаются `#[derive(Event)]`.
- [ ] Для entity-таргетированных: `#[derive(Event)]` → `#[derive(EntityEvent)]`; поле с целью привести к имени `entity` либо пометить `#[event_target]`; убрать дублирующие Entity-поля.
- [ ] Обновить `commands.trigger(...)` и наблюдателей: чтение цели через поле события.
- [ ] Обновить `CommandType::trigger(&mut commands)` (`src/commandable/components.rs:20-30`).
- [ ] Там, где наблюдатель обслуживает одну конкретную сущность, рассмотреть entity-scoped регистрацию `commands.entity(e).observe(...)` вместо глобального `add_observer` (например, наблюдатели структур: ферма/кровать/склад).
- [ ] Обновить skill `.claude/skills/message-to-event/SKILL.md`: дополнить паттерном Event vs EntityEvent.

**Критерии:**

- [ ] Все события с целевой сущностью — `EntityEvent`.
- [ ] Ни одно событие не несёт `Entity`-поле, дублирующее target.
- [ ] Поведение игры без изменений (команды, смерть пешки, прерывания).

**Риски:** самая объёмная задача фазы. Делать по образцу прошлой миграции — по одному событию/команде на коммит (см. историю `b9add30`...`299b0ab`). Начать с одного простого (`PawnDeathEvent`), отладить паттерн, затем катить остальные.

### 1.3. Кастомные relationships для связей сущностей

**Цель:** двунаправленные связи с автоочисткой при despawn вместо `Entity`-полей и ручной зачистки.

**Справка по API:**

```rust
#[derive(Component)]
#[relationship(relationship_target = Carrying)]
pub struct CarriedBy(pub Entity);        // сторона-источник: единственное Entity-поле

#[derive(Component)]
#[relationship_target(relationship = CarriedBy)]
pub struct Carrying(Vec<Entity>);        // обратная сторона, поддерживается движком
```

- Вставка `CarriedBy(pawn)` на предмет автоматически добавляет предмет в `Carrying` пешки; удаление/despawn любой стороны рвёт связь с обеих сторон.
- Relationship-компонент **принудительно немутабелен**: сменить связь = re-insert компонента (`commands.entity(item).insert(CarriedBy(other_pawn))`).
- Флаг `linked_spawn` на `relationship_target` означает «related-сущности despawn'ятся вместе с владельцем» (как `Children`). Для task/carryable **НЕ использовать** — предмет/задача должны переживать смерть пешки; связь просто рвётся.
- `allow_self_referential` — разрешает связь сущности с самой собой (не нужен здесь).

**Что сделать:**

- [ ] Инвентаризация `Entity`-полей в компонентах/ресурсах: `grep -rn ': Entity\|<Entity>' src/ --include='components.rs'` + просмотр `tasks_queue`, `carryable`, `commandable`, `navigation/occupation`.
- [ ] Кандидаты на пары relationship/relationship_target:
  - **pawn ↔ task**: назначенная задача ↔ исполнитель (`AssignedTo` / `AssignedTasks`).
  - **carryable ↔ carrier/storage**: предмет в руках/на складе (`CarriedBy` / `Carrying`, `StoredIn` / `Stores`).
  - **occupation**: сущность ↔ занимаемый тайл, если тайлы — сущности; если тайлы в ресурсе `Navmesh` — не трогать.
- [ ] Для каждой пары: завести компоненты-связи, заменить ручные вставки/удаления, удалить ставший ненужным код зачистки (в т.ч. проверить, что покрывает `ReleaseCommandResourcesEvent`, и сократить его зону ответственности).
- [ ] Проверить сценарий «смерть пешки с активной задачей и предметом в руках»: задача должна вернуться в очередь (это бизнес-логика — оставить наблюдателю), связь — очиститься автоматически.

**Критерии:**

- [ ] Despawn пешки/предмета не оставляет висячих `Entity`-ссылок (класс багов типа коммита `d70286b`).
- [ ] Ручные системы очистки удалены или сокращены.

**Риски:** немутабельность relationship-компонентов — учесть в местах «переложить предмет» (re-insert, а не мутация). Семантику `linked_spawn` не включать без осознанного решения (см. справку).

### 1.4. bevy_picking для hover и выделения

**Цель:** заменить ручной raycast/hit-test в `ui/hovered`, `user_selection` на первопартийный picking.

**Справка по API:**

- Cargo-фича: `sprite_picking`. Плагин `SpritePickingPlugin` (+ `SpritePickingSettings`, `SpritePickingMode`, `SpritePickingCamera`) из `bevy_sprite::picking_backend`.
- События — обёртка `Pointer<E>` (это `EntityEvent` с `auto_propagate` по иерархии `ChildOf`), внутренние типы: `Over`, `Out`, `Press`, `Release`, `Click`, `Move`, `DragStart`, `Drag`, `DragEnd`, `Scroll` и др. Именно **`Press`/`Release`**, не `Pressed`/`Released`.
- Компонент `Hovered(pub bool)` (`bevy_picking::hover`) — движок сам поддерживает его на сущностях, если он вставлен.

```rust
use bevy::picking::prelude::*;

commands.spawn((Sprite { .. }, Pickable::default())).observe(
    |click: On<Pointer<Click>>| {
        info!("clicked {}", click.entity);   // Pointer<E> несёт поле entity
    },
);

// исключить из picking (тайлы карты):
Pickable::IGNORE  // == Pickable { should_block_lower: false, is_hoverable: false }
```

**Что сделать:**

- [ ] Включить фичу `sprite_picking` у bevy в `Cargo.toml`, добавить `SpritePickingPlugin`.
- [ ] Заменить генерацию `HoverEvent`, `ClickStage0Event`/`ClickStage1Event` из сырого ввода мыши (`src/input/mod.rs`) на наблюдатели `On<Pointer<Over>>`, `On<Pointer<Out>>`, `On<Pointer<Click>>` на спрайтах пешек/структур/предметов.
- [ ] `Pickable` вешать только на выделяемые сущности; тайлы карты — `Pickable::IGNORE` (иначе перф просядет и тайлы будут перехватывать хиты).
- [ ] Гейтинг по egui: сохранить `EguiWantsFocus`-гейт (`src/input/mod.rs:19`) — picking должен молчать, когда egui захватывает мышь (проверить, не решает ли это `SpritePickingSettings`/порядок бэкендов; если нет — run condition на наблюдателях, см. 2.7).
- [ ] Многостадийный клик (циклический выбор из нескольких сущностей под курсором) — воспроизвести поверх `Pointer<Click>`: `should_block_lower: false` даёт хиты по всем слоям, логику стадий оставить свою.
- [ ] Удалить ставший ненужным код ручного hit-test.

**Критерии:**

- [ ] Hover-подсветка и выделение работают как раньше, включая перекрывающиеся сущности.
- [ ] Клики сквозь egui-панели не проходят в мир.
- [ ] Ручной raycast-код удалён.

**Риски:** средний объём. Сначала прототип на одной сущности (пешка), потом миграция остального. Если picking не покроет сценарий многостадийного выбора — допустимо оставить гибрид (picking для hover, ручной цикл для стадий), зафиксировав это в коде комментарием.

### 1.5. Fallible systems + `Single<T>` вместо `.single().expect()`

**Цель:** убрать паники из систем.

**Справка по API:**

```rust
// Single<D, F> — параметр системы; система МОЛЧА ПРОПУСКАЕТСЯ исполнителем,
// если сущностей 0 или больше одной. Deref к item'у.
fn update_ui(mut text: Single<&mut Text, With<FoodStockTextUIMarker>>) {
    text.0 = "...".into();
}

// Option<Single<...>> — система выполняется всегда, при 0/több совпадениях будет None.
// Populated<D, F> — как Query, но система пропускается, если результат пуст.

// Fallible system: bevy_ecs::error::Result = Result<(), BevyError>, `?` работает:
fn sys(assets: Res<Assets<ColorMaterial>>, q: Query<&Handle<ColorMaterial>>) -> Result {
    let handle = q.single()?;              // ошибка уйдёт в error handler, не паника
    let mat = assets.get(handle).ok_or("material missing")?;
    Ok(())
}
// Обработчик по умолчанию настраивается ресурсом FallbackErrorHandler(fn(BevyError, ErrorContext));
// готовые: bevy::ecs::error::{panic, error, warn, info, ignore}.
```

**Что сделать:**

- [ ] Найти все ~7 мест `.single().expect(...)`: `src/ui/simulation_state/mod.rs:80`, `src/ui/items_stock/mod.rs:117`, `src/ui/debug/info.rs:89` и остальные (`grep -rn 'single().expect\|single_mut().expect' src/`).
- [ ] Для систем «ровно одна сущность с маркером» — заменить `Query<..., With<Marker>>` + `.single()` на параметр `Single<..., With<Marker>>`.
- [ ] Там, где `Single` не подходит (нужна ветвящаяся логика) — система возвращает `Result`, внутри `?` на `single()`.
- [ ] Пройтись по `.unwrap()` на `Assets` (`src/ui/debug/noise/systems.rs:123-124`) — перевести на `?` в fallible-системе.
- [ ] Единственный `.single_mut()` (`src/ui/debug/info.rs:118`) — та же схема.

**Критерии:**

- [ ] `grep -rn '\.expect(' src/` в системах по Query — 0 (допустимы expect в setup-коде с осмысленным сообщением).
- [ ] Поведение UI не изменилось.

### 1.6. Component hooks + immutable components для `*Tag`-стейт-машин

**Цель:** инвариант «ровно один state-tag на стейт-машину» держится на уровне ECS.

**Справка по API:**

```rust
use bevy::ecs::{lifecycle::HookContext, world::DeferredWorld};

#[derive(Component)]
#[component(immutable, on_add = on_add_idle_tag)]   // ключи: on_add, on_insert, on_replace, on_remove, on_despawn
struct PawnStateIdleTag;

// сигнатура хука: type ComponentHook = for<'w> fn(DeferredWorld<'w>, HookContext);
fn on_add_idle_tag(mut world: DeferredWorld, ctx: HookContext) {
    // ctx: { entity: Entity, component_id: ComponentId, caller, relationship_hook_mode }
    world.commands().entity(ctx.entity).remove::<(PawnStateWorkingTag, PawnStateSleepingTag /* ... */)>();
}
```

Формы атрибута: `on_add` (берёт `Self::on_add`), `on_add = path_к_fn`, `on_add = выражение_возвращающее_hook()`. Хуки срабатывают немедленно в момент вставки (до наблюдателей `Add`/`Insert`). `#[component(immutable)]` запрещает `&mut`-доступ к компоненту.

**Что сделать:**

- [ ] Инвентаризация тегов: `PawnState*Tag`, `MovableState*Tag`, `CommandableState*Tag` и др. (`grep -rn 'Tag;' src/`).
- [ ] Для каждой группы тегов добавить `on_add`-хук, снимающий остальные теги группы (одна функция или макрос на группу — по образцу из справки).
- [ ] Пометить теги `#[component(immutable)]` — у них нет данных, мутировать нечего.
- [ ] Удалить ручные `remove::<OldTag>()` в системах переходов, где их теперь делает хук. Внимание: `log_state_change!` — если логирование привязано к ручному переходу, оставить его в системах или перенести в хук.
- [ ] Проверить взаимодействие с `ensure_state!`/`continue_unless!` — они читают теги, менять не нужно.

**Критерии:**

- [ ] Невозможно получить сущность с двумя тегами одной группы (добавить unit-тест: вставить второй тег, убедиться, что первый снят).
- [ ] Существующие переходы состояний работают, логи состояний сохранились.

**Риски:** хук срабатывает немедленно при вставке — убедиться, что наблюдатели `EntityStateChangeEvent` не зависят от наличия старого тега после вставки нового.

---

## Фаза 2 — состояния, рендер, инфраструктура

### 2.1. `DespawnOnExit` для debug-слоёв

**Справка по API:** `bevy_state::state_scoped` (в prelude): `DespawnOnExit<S: States>(pub S)` и `DespawnOnEnter<S: States>(pub S)` — компоненты; сущность автоматически despawn'ится при выходе из/входе в указанное состояние.

```rust
commands.spawn((DespawnOnExit(DebugNavmeshState::On), Sprite { .. }, ...));
```

**Что сделать:**

- [ ] При спавне debug-сущностей добавлять `DespawnOnExit(State::On)`:
  - navmesh-тайлы (`src/ui/debug/navmesh.rs`, спавн + ручной despawn `:38`, `:64`);
  - noise-mesh (`src/ui/debug/noise/systems.rs:56/85/107` — `despawn_noise_texture`);
  - grid — если рисуется gizmos, не трогать (gizmos не сущности).
- [ ] Удалить ручные despawn-системы/функции и их регистрацию.

**Критерии:**

- [ ] Тумблеры debug-слоёв работают, ручных despawn-систем для них нет.

### 2.2. Аудит `NextState` после поведенческого изменения 0.18

**Контекст:** с 0.18 установка `NextState` в то же самое значение заново триггерит `OnEnter`/`OnExit`.

**Справка по API:**

```rust
pub enum NextState<S> { Unchanged, Pending(S), PendingIfNeq(S) }
next_state.set(s);         // ВСЕГДА прогоняет OnExit/OnEnter, даже если s == текущее
next_state.set_if_neq(s);  // пропускает transition-схедулы, если s == текущее состояние
```

Каверза из доков: если в этом же кадре уже был вызван `set()` с тем же состоянием, схедулы выполнятся несмотря на последующий `set_if_neq`.

**Что сделать:**

- [ ] Проверить все места записи `NextState` (`src/ui/debug/info.rs:105-114`, inline-замыкания в `src/ui/debug/noise/mod.rs:20`, `src/ui/debug/navmesh.rs:22`, `src/main.rs:95`).
- [ ] Где повторная установка того же значения возможна (тумблеры, вызовы из UI каждый клик) — `set_if_neq` либо гарантированное переключение (`match current { On => Off, Off => On }` — уже безопасно).
- [ ] Особо: `AppState::Quiting` в `main.rs:95` — если Esc можно нажать дважды, повторный `OnEnter(Quiting)` не должен ломать выход.

**Критерии:**

- [ ] Повторное «включение включённого» debug-слоя не пересоздаёт сущности и не дублирует спавн.

### 2.3. `TilemapChunk` для рендера карты

**Цель:** сократить количество сущностей и draw calls: сейчас — отдельный `Sprite` на тайл (`src/map/systems.rs:28`).

**Справка по API:** модуль `bevy_sprite_render::tilemap_chunk` (реэкспорт через `bevy::sprite`), плагин `TilemapChunkPlugin`.

```rust
pub struct TilemapChunk {              // #[component(immutable, on_insert = ...)]
    pub chunk_size: UVec2,             // тайлов в чанке
    pub tile_display_size: UVec2,      // размер тайла в пикселях
    pub tileset: Handle<Image>,        // ARRAY TEXTURE (слоистая текстура), не обычный атлас
    pub alpha_mode: AlphaMode2d,
}
pub struct TileData { pub tileset_index: u16, pub color: Color, pub visible: bool, pub orientation: TileOrientation }
pub struct TilemapChunkTileData(pub Vec<Option<TileData>>);   // None = пустой тайл; Deref к Vec

commands.spawn((
    TilemapChunk { chunk_size: UVec2::splat(16), tile_display_size: UVec2::splat(32),
                   tileset: tileset_handle, alpha_mode: default() },
    TilemapChunkTileData(vec![Some(TileData::from_tileset_index(0)); 256]),
    Transform::from_xyz(x, y, TILE_Z_INDEX),
));
```

Хук `on_insert` проверяет `tile_data.len() == chunk_size.x * chunk_size.y` и строит mesh/материал. Перерисовка тайла — мутация `TilemapChunkTileData` (компонент мутабелен, immutable только `TilemapChunk`). Тайлсет — array texture: слои по `tileset_index`; собрать из отдельных картинок можно на старте (`Image`-манипуляции или заранее подготовленный ассет).

**Что сделать:**

- [ ] Изучить пример использования в исходниках bevy (`bevy_sprite_render-0.19.0/src/tilemap_chunk/`) и примерах реестра.
- [ ] Собрать тайловые текстуры карты в array texture, как требует `tileset`.
- [ ] Переписать спавн карты: вместо N×M спрайт-сущностей — чанки. Логические данные тайлов (`Tile`-компоненты, если по ним есть запросы) — решить: оставить сущности без рендера либо перенести данные в ресурс карты; выбрать по тому, как их используют navigation/farming.
- [ ] `RebuildMapEvent` — перерисовка через мутацию `TilemapChunkTileData` вместо despawn/respawn спрайтов.
- [ ] Z-порядок: чанки на `TILE_Z_INDEX`, пропсы/структуры/пешки — поверх, как сейчас.

**Критерии:**

- [ ] Визуально карта идентична (включая debug-оверлеи noise поверх).
- [ ] Количество сущностей на старте упало на ~размер карты.
- [ ] Rebuild карты работает.

**Риски:** самая инвазивная задача фазы — рендер и структура данных карты связаны с navigation и debug-слоями. Допустимо временно оставить старый путь за cfg/константой до стабилизации.

### 2.4. Reflect auto-registration: убрать ручные `register_type`

**Справка по API:** фича `reflect_auto_register` включена по умолчанию (входит в `default_app`). Все конкретные типы с `#[derive(Reflect)]` регистрируются автоматически при старте приложения. **Исключение — generic-типы**: их конкретизации (`EntityStateChangeEvent<PawnState>` и т.п.) по-прежнему требуют ручного `register_type::<...>()` (цитата из доков: "this does not apply to types with generics: their desired monomorphized representations must be registered manually").

**Что сделать:**

- [ ] Удалить вызовы `.register_type::<T>()` для конкретных (не generic) типов (`grep -rn 'register_type' src/` — сейчас 16 вызовов).
- [ ] Оставить (с комментарием) только generic-конкретизации.
- [ ] Проверить, что инспектор `bevy-inspector-egui` видит компоненты и `bevy_remote` отдаёт типы.
- [ ] Проверить, что фича не отвалилась, если 2.8 (feature-коллекции) уже сделана: `reflect_auto_register` должна остаться в наборе.

**Критерии:**

- [ ] Инспектор и BRP работают как раньше; ручных `register_type` — минимум (только generic-конкретизации).

### 2.5. `children![...]` вместо `with_children`-замыканий

**Справка по API:** макрос `children![...]` (из `bevy_ecs::hierarchy`, в prelude) — элементы-бандлы прямо в кортеже спавна; для динамики — `Children::spawn` + `Spawn`/`SpawnIter`/`SpawnWith` из `bevy_ecs::spawn`:

```rust
commands.spawn((
    Node { .. },
    children![
        (Text::new("Header"), TextFont { .. }),
        (Node { .. }, children![Text::new("nested")]),   // вложенность
    ],
));

// динамическая генерация детей:
commands.spawn((
    Node { .. },
    Children::spawn((
        Spawn((Text::new("static"),)),
        SpawnIter(items.into_iter().map(|i| (Text::new(i.label),))),   // из итератора
        SpawnWith(|parent: &mut RelatedSpawner<ChildOf>| {             // произвольная логика
            if condition { parent.spawn((Text::new("cond"),)); }
        }),
    )),
));
```

**Что сделать:**

- [ ] Все ~12 мест `with_children(|parent| ...)` в `src/ui/*` (включая вложенные `src/ui/storage/mod.rs:41/46/51`) переписать на `children![...]`.
- [ ] Где внутри замыкания есть логика (циклы, условия) — `SpawnIter`/`SpawnWith` либо оставить `with_children` — по читаемости.

**Критерии:**

- [ ] UI-дерево идентично (проверить визуально + инспектором); замыкания остались только там, где есть настоящая логика.

### 2.6. Debug-текст гизмосами + FPS-оверлей

⚠️ Поправка к исходному анализу: типов `DiagnosticsOverlayPlugin`/`DiagnosticsOverlay` в bevy 0.19 **нет** — актуальный API это `FpsOverlayPlugin` из `bevy_dev_tools::fps_overlay` (тот самый, что закомментирован в main.rs — его и использовать, раскомментировав и настроив).

**Справка по API:**

```rust
// Текст гизмосами (bevy_gizmos, только ASCII 32–126):
fn debug_labels(mut gizmos: Gizmos) {
    gizmos.text_2d(Isometry2d::from_translation(pos), "label", 25.0, Vec2::ZERO, Color::WHITE);
    // сигнатура: text_2d(isometry: impl Into<Isometry2d>, text: &str, font_size: f32, anchor: Vec2, color: impl Into<Color>)
    // многосекционный: gizmos.text_sections_2d(iso, &[("a", RED), ("b", GREEN)], 25.0, Vec2::ZERO)
}

// FPS-оверлей (фича bevy_dev_tools уже включена в Cargo.toml):
use bevy::dev_tools::fps_overlay::{FpsOverlayPlugin, FpsOverlayConfig};
app.add_plugins(FpsOverlayPlugin::default());
// или FpsOverlayPlugin { config: FpsOverlayConfig { .. } }; есть также FrameTimeGraphPlugin
```

**Что сделать:**

- [ ] Debug-подписи в мире — через `Gizmos::text_2d()` вместо Text2d-сущностей. Проверить текущие `Text2d` (`src/pawn/systems.rs:65/129`): если это игровой UI (имя/статус пешки) — НЕ трогать; если чисто отладочный — перевести.
- [ ] Раскомментировать и включить `FpsOverlayPlugin` (`src/main.rs:31-41`) за фичей `debug_ui`; выкинуть устаревшие куски закомментированной конфигурации, если не компилируются.

**Критерии:**

- [ ] FPS-оверлей включается в debug-сборке; мёртвый закомментированный код удалён.

### 2.7. Run conditions на наблюдателях

**Справка по API:** два способа (оба — `bevy_ecs::observer`):

```rust
// 1) на замыкании/системе наблюдателя — нужен трейт ObserverSystemExt:
use bevy::ecs::observer::ObserverSystemExt;
app.add_observer((on_feed_command).run_if(in_state(AppState::Playing)));

// 2) через Observer::new (спавн вручную):
world.spawn(Observer::new(on_feed_command).run_if(in_state(AppState::Playing)));

// entity-scoped тоже поддерживает:
commands.entity(e).observe((handler).run_if(cond));
```

`run_if` можно чейнить — условия объединяются по AND.

**Что сделать:**

- [ ] Инвентаризация 45 `add_observer`: какие наблюдатели должны молчать вне `AppState::Playing` (геймплейные команды, смерть, фермы) — сейчас гейтинг есть только у систем.
- [ ] Навесить `.run_if(in_state(AppState::Playing))` при регистрации (способ 1 из справки).
- [ ] Не гейтить: наблюдатели загрузки/карты, работающие в `Loading`.

**Критерии:**

- [ ] Триггер геймплейного события вне `Playing` (например, в `Loading`) не выполняет логику. Поведение в `Playing` не изменилось.

### 2.8. Cargo: feature-коллекция `2d`

**Справка по API:** feature-коллекции bevy 0.19 (из `bevy-0.19.0/Cargo.toml`):

```toml
default = ["2d", "3d", "ui", "audio"]
2d  = ["default_app", "default_platform", "2d_bevy_render", "scene", "picking"]
ui  = ["default_app", "default_platform", "ui_api", "ui_bevy_render", "scene", "picking", "bevy_ui_widgets"]
default_app = ["async_executor", "bevy_asset", "bevy_log", "bevy_state", "reflect_auto_register"]
# отдельные фичи: sprite_picking, bevy_dev_tools, bevy_remote, bevy_feathers, dynamic_linking, ...
```

**Что сделать:**

- [ ] В `Cargo.toml`: `default-features = false`, `features = ["2d", "ui", "audio"?]` + явно нужные: `dynamic_linking`, `bevy_dev_tools`, `bevy_remote`, `sprite_picking` (после 1.4), `bevy_gizmos`-рендер входит в `2d_bevy_render`. `audio` — только если звук реально используется.
- [ ] Подобрать минимальный набор итеративно: собирать и добавлять недостающее по ошибкам компиляции.
- [ ] Проверить сторонние крейты: `cargo tree -e features` — bevy_pancam, bevy_asset_loader, bevy-inspector-egui, bevy_egui могут тянуть свои bevy-фичи.
- [ ] Замерить время чистой сборки до/после (`cargo build --timings`), зафиксировать в описании коммита и в разделе «Решения».

**Критерии:**

- [ ] Игра собирается и работает (окно, рендер, egui, инспектор, BRP); 3D-стек не компилируется.

---

## Фаза 3 — точечные внедрения

### 3.1. `Query::contiguous_iter` для массовых тиков

**Цель:** подготовить горячие системы к росту популяции пешек.

**Справка по API:**

```rust
// bevy_ecs::system::Query, 0.19:
pub fn contiguous_iter(&self) -> Result<QueryContiguousIter<...>, QueryNotDenseError>
pub fn contiguous_iter_mut(&mut self) -> Result<QueryContiguousIter<...>, QueryNotDenseError>

// Требования:
//  - D: ContiguousQueryData — все компоненты запроса с Table (dense) storage;
//    SparseSet-компонент в запросе => Err(QueryNotDenseError) в рантайме.
//  - F: ArchetypeFilter — только With/Without; Changed/Added НЕ допускаются (не скомпилируется).
// Итерирует по таблицам, отдаёт непрерывные слайсы (&[T] / &mut [T]) — дружелюбно к SIMD/автовекторизации.

fn tick(mut q: Query<(&mut Feedable, &Pawn)>) {
    for (feedables, pawns) in q.contiguous_iter_mut().unwrap() {
        for (f, p) in feedables.iter_mut().zip(pawns.iter()) { /* ... */ }
    }
}
```

**Что сделать:**

- [ ] Определить кандидатов — тики по всем пешкам: `src/feedable/mod.rs:63`, `src/restable/mod.rs:88`, `src/movable/systems.rs:9`, `src/workable/systems.rs:10`, `src/pawn/systems.rs:185`. Проверить, что в запросах нет `Changed`/`Added`-фильтров и sparse-компонентов.
- [ ] Переписать итерацию на `contiguous_iter_mut()` там, где контракт выполняется.
- [ ] Бенчмарк: замерить время систем до/после при 1000+ пешек (спавн массовки за флагом/тестом; профилировать через tracy/`FpsOverlayPlugin`). Если выигрыша нет на текущих объёмах — оставить одну систему как образец, остальное не трогать.

**Критерии:**

- [ ] Поведение тиков не изменилось (тесты голода/усталости зелёные).
- [ ] Есть числа до/после, записаны в раздел «Решения».

### 3.2. Прототип headless-виджетов / Feathers для debug UI

**Цель:** оценить замену egui первопартийными виджетами. Исследовательская задача со staged-подходом — НЕ переписывать всё сразу.

⚠️ Поправка к исходному анализу: компонента `EditableText` в 0.19 нет; текстовый ввод — виджет `TextInput` + `EditableTextInputPlugin` из `bevy_ui_widgets::text_input`.

**Справка по API:**

- **`bevy_ui_widgets`** (headless, без стилей; фича `bevy_ui_widgets`, входит в коллекцию `ui`): группа плагинов `UiWidgetsPlugins`; виджеты `Button`, `Checkbox`, `Slider`, `Scrollbar`, `TextInput` и др. События (оба `EntityEvent`): `Activate { entity }` (кнопка нажата), `ValueChange<T> { source, value, is_final }`. Состояния — компоненты из `bevy_ui::interaction_states`: `Pressed`, `Checked`, `Checkable`, `InteractionDisabled`; `Hovered(bool)` — из `bevy_picking`. Фокус ввода — ресурс `InputFocus` (`bevy_input_focus`) — аналог `EguiWantsFocus`-гейта.

```rust
use bevy::ui_widgets::{Button, Activate};
commands.spawn((Button, Node { .. }, children![Text::new("Toggle grid")]))
    .observe(|activate: On<Activate>, mut next: ResMut<NextState<DebugGridState>>| {
        // activate.entity — кнопка
    });
```

- **`bevy_feathers`** (стилизованные виджеты поверх ui_widgets; фича `bevy_feathers`): `FeathersPlugins`, контролы `button`, `checkbox`, `toggle_switch`, `slider`, `text_input`, `number_input`, `listview` и др. Построены на новой сценовой системе BSN (`SceneComponent`, `FeathersButtonProps { caption, variant, corners }`).

**Что сделать:**

- [ ] Включить фичи `bevy_ui_widgets` (входит в `ui`) и `bevy_feathers`; добавить `UiWidgetsPlugins`/`FeathersPlugins`.
- [ ] Перенести одну панель — `ui/debug/info.rs` (кнопки-тумблеры debug-состояний): `Button` + наблюдатель `Activate`, `Checked` для отображения состояния тумблера.
- [ ] Сравнить с egui-версией: объём кода, отзывчивость, внешний вид, работа фокуса (`InputFocus` vs `EguiWantsFocus`).
- [ ] Записать вердикт в раздел «Решения»: мигрируем остальное / остаёмся на egui. Инспектор (`bevy-inspector-egui`) в любом случае остаётся на egui.

**Критерии:**

- [ ] Панель-прототип работает наравне с egui-версией.
- [ ] Вердикт записан.

### 3.3. Resources-as-entities: доступ к конфигу через Resource + реакция на изменения

**Цель:** использовать то, что с 0.19 ресурсы — сущности: сделать конфиг наблюдаемым, сократить зону `static`.

**Контекст:** `pub static CONFIG: OnceLock<RootConfig>` (`src/config.rs:28`) + глобальный `config()`. `MapGeneratorConfig` уже извлечён в Resource (коммит `369be49`).

**Справка по API:** в 0.19 `#[derive(Resource)]` автоматически derive'ит и `Component` (SparseSet) с `#[require(IsResource)]`; значение ресурса хранится как компонент на выделенной сущности с маркером `IsResource`. Следствия:

```rust
// Хуки и require работают прямо на ресурсах:
#[derive(Resource)]
#[component(on_insert = on_config_changed)]      // сработает и на insert_resource, и на замену
struct RootConfigRes(RootConfig);

// Наблюдатели lifecycle-событий компонента ресурса:
app.add_observer(|on: On<Insert, RootConfigRes>, cfg: Res<RootConfigRes>| {
    // ресурс вставлен/заменён — пересчитать derived-поля, залогировать
});

// Запрос к сущности ресурса: world.query::<(Entity, &RootConfigRes, &IsResource)>()
```

Каверзы: НЕЛЬЗЯ derive `Component` и `Resource` на одном типе вручную (конфликт — Resource уже derive'ит Component); одна каноническая сущность на тип ресурса; despawn сущности ресурса не поддерживается; удаление `IsResource` с сущности удаляет ресурс.

**Что сделать:**

- [ ] Вставить `RootConfig` также как Resource при старте (данные те же, что в `OnceLock`). Если на `RootConfig` где-то был ручной `derive(Component)` — убрать (конфликт, см. справку). Глобальный `config()` оставить — массовая замена вызовов не входит в задачу.
- [ ] Повесить наблюдатель `On<Insert, ...>` либо `on_insert`-хук: пересчитывать `calculate_derived_fields()` и логировать применение. Это даёт живую подстройку конфига из инспектора в debug-сборке.
- [ ] Зафиксировать инвариант в комментарии в `config.rs`: `OnceLock` — снапшот на старте для горячих путей; Resource — источник для инспектора/BRP; при рассинхроне приоритет за Resource (или честно задокументировать, что static не обновляется — решить при реализации, что дешевле).
- [ ] Новый код писать через `Res<RootConfig>`, а не `config()`.

**Критерии:**

- [ ] `RootConfig` виден и редактируется в инспекторе/через BRP.
- [ ] Изменение поля конфига в рантайме логируется и пересчитывает derived-поля.
- [ ] Существующие вызовы `config()` работают как раньше.

**Риски:** двойной источник правды static vs Resource — обязательно явно задокументировать выбранную семантику в `config.rs`.

---

## Порядок и зависимости

- Фаза 0 — сразу, одним коммитом.
- 1.1, 1.5, 1.6, 2.1, 2.2, 2.4, 2.5, 2.6 — независимые, можно в любом порядке.
- 1.2 (EntityEvent) делать до 2.7 (run conditions на наблюдателях) — чтобы не переделывать регистрацию дважды.
- 1.4 (picking) требует правку `Cargo.toml` — удобно объединить по времени с 2.8 (features `2d`).
- 1.3 (relationships) лучше после 1.2 — события уже несут target штатно.
- 2.3 (TilemapChunk) — самая инвазивная, делать изолированно от других задач.
- Фаза 3 — в конце; 3.2 — исследовательская.

## Прогресс по задачам

- [x] 0.1 Удалить закомментированный легаси Message-эпохи
- [x] 0.2 Удалить `log_message!`
- [x] 0.3 Переименовать `render_debug_ui_window_node_bundle`
- [x] 0.4 `Val::Px` → `px()`/`percent()` (не применимо, см. «Решения»)
- [x] 0.5 Опечатка `DebugUiContainerarker`
- [x] 1.1 `#[require(...)]` для Pawn
- [ ] 1.2 `EntityEvent`-миграция
- [ ] 1.3 Кастомные relationships
- [ ] 1.4 bevy_picking
- [ ] 1.5 Fallible systems + `Single<T>`
- [ ] 1.6 Hooks + immutable для `*Tag`
- [ ] 2.1 `DespawnOnExit` для debug-слоёв
- [ ] 2.2 Аудит `NextState` (поведение 0.18)
- [ ] 2.3 `TilemapChunk`
- [ ] 2.4 Reflect auto-registration
- [ ] 2.5 `children![...]`
- [ ] 2.6 `Gizmos::text_2d` + `FpsOverlayPlugin`
- [ ] 2.7 Run conditions на наблюдателях
- [ ] 2.8 Feature-коллекция `2d`
- [ ] 3.1 `contiguous_iter` + бенчмарк
- [ ] 3.2 Прототип headless-виджетов/Feathers
- [ ] 3.3 Resources-as-entities для `RootConfig`

## Решения

- **0.4 (Val::Px → px())**: не применимо для статиков `UI_SCREEN_EDGE_PX_OFFSET`/`UI_WINDOWS_GAP` — `px()` не является `const fn`, статики требуют const-инициализатор. `Val::Px` там остаётся намеренно. Остальной UI-код уже использует `px()`.

_(далее по ходу работ: вердикт по Feathers (3.2), семантика static vs Resource (3.3), результаты бенчмарка (3.1), замер сборки (2.8))_
