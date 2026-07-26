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

- [x] Инвентаризация выполнена. **Стали `EntityEvent` (21):** 8 `*Command` (target `commandable_entity`), `PawnDeathEvent`, `EntityStateChangeEvent<T>` (tuple, `#[event_target]` на поле 0), `CommandCompleteEvent`, `ExternalCommandInterruptEvent`, `WorkCompleteEvent` (target `workable_entity` — чтобы ферма могла наблюдать себя), `RestCompleteEvent`, `MovableReachedDestinationEvent`, `PathfindRequestEvent`, `PathfindAnswerEvent`, `FarmProgressEvent`, `FarmTendedEvent`, `StoreCarryableEvent`, `MergeCarryablesEvent` (target `entity_to_merge`). **Остались `Event` (15):** Hover/ClickStage0/ClickStage1 (координаты), `StateChangeEvent<T>`, `FoodConsumedEvent` (только amount), `OccupationChangeEvent`, `ScheduleTaskEvent`, `UserSelectionEvent`, `UserSelectionCommand` (нет entity), Rebuild/NewDay/NewSeason/NewYear, `SpawnCarryableEvent`, а также `InternalCommandInterruptEvent`/`ReleaseCommandResourcesEvent` (несут целый `CommandType`, не entity).
- [x] Derive-переключение + `#[event_target]` там, где поле называется не `entity`. Поля НЕ переименовывались — существующие имена (`commandable_entity` и т.п.) остались, дублей не было.
- [x] `commands.trigger(...)` и наблюдатели не потребовали изменений — `On<E>` дерефится к событию, чтение цели через поле работает как раньше.
- [x] `CommandType::trigger` работает без правок (диспатчит те же структуры).
- [x] Entity-scoped `.observe()` — отложено осознанно: все текущие наблюдатели глобальные и фильтруют по полю события; переводить на per-entity регистрацию имеет смысл вместе с 1.3 (relationships), когда появятся структурные связи. Возможность открыта самой миграцией.
- [x] Skill `.claude/skills/message-to-event/SKILL.md` дополнен разделом «Event vs EntityEvent» с правилами выбора target-поля.

**Критерии:**

- [x] Все события с целевой сущностью — `EntityEvent` (21 шт.).
- [x] Ни одно событие не несёт `Entity`-поле, дублирующее target.
- [x] Поведение игры без изменений — смоук-тест: полная цепочка `MoveToCommand → MovableReachedDestination → WorkOn → WorkComplete → FarmProgress → CommandComplete` + прерывания в логе, паник нет; подтверждено пользователем.

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

- [x] Инвентаризация выполнена. Реальные Entity↔Entity связи: **bed↔pawn** (`Bed.owner` + `Pawn.owned_bed`) и **carryable↔pawn** (`Pawn.inventory: HashMap`). НЕ применимо: pawn↔task (Task — value-объект в ресурсе `TasksQueue`, не сущность), occupation (данные в ресурсе `Navmesh`), storage↔item (склад — точка на карте, предметы лежат тайлом, связи-компонента нет).
- [x] **Bed↔Pawn**: пара `BedOwnedBy(Entity)` (на кровати) ↔ `OwnedBed` (на пешке). `claim_by`/`unclaim_by` удалены — claim это `insert(BedOwnedBy(pawn))`. Счётчик `AvailableBeds` ведётся наблюдателями `On<Add/Remove, BedOwnedBy>` — не дрейфует, кто бы ни снял связь. Ручная зачистка в `on_pawn_death` сведена к `remove::<BedOwnedBy>` (мёртвые пешки не despawn'ятся, поэтому явный remove нужен); `.expect` на bed-запросе исчез.
- [x] **Carryable↔Pawn**: пара `CarriedBy(Entity)` ↔ `Carrying`. `Pawn.inventory: HashMap<Entity, Carryable>` удалён целиком — kind/amount больше не зануляются при подборе, «в инвентаре» = наличие `CarriedBy`. Вариант `CarryableKind::InInventory` удалён. `pick_up_item`/`drop_item` переехали с `Pawn` на `Carryable::pick_up`/`drop_off`. `on_food_consumed` теперь явно исключает несомые предметы через `Without<CarriedBy>`.
- [x] Сценарий «смерть пешки с предметом»: как и раньше — очередь команд чистится, отложенный `DropCarriedItem` дропает предмет через `on_release_resources` (guard переписан на `Carrying::contains`). Despawn любой стороны рвёт связь автоматически.
- [x] Зафиксирована остаточная гонка claim'а кроватей: два `ToRestCommand` в одном флаше могут выбрать одну кровать — победит последний insert, проигравший переклеймит в следующий цикл отдыха; счётчик не дрейфует (комментарий в `src/structure/bed/mod.rs`). Старый код был immediate и без гонки, но с ручной синхронизацией двух полей.

**Критерии:**

- [x] Despawn пешки/предмета не оставляет висячих `Entity`-ссылок (класс багов типа коммита `d70286b`).
- [x] Ручные системы очистки удалены или сокращены (`claim_by`/`unclaim_by`, HashMap-стэш, `.expect` на bed-запросе).

**Риски:** немутабельность relationship-компонентов — учесть в местах «переложить предмет» (re-insert, а не мутация). Семантику `linked_spawn` не включать без осознанного решения (см. справку).

### 1.4. bevy_picking для hover и выделения — ЗАКРЫТО: НЕ ПРИМЕНИМО

**Решение (2026-07-24, подтверждено пользователем):** не внедрять. Исходная посылка анализа оказалась неверной — «ручного hit-test» в проекте нет. `mouse_input` делает одно преобразование `viewport_to_world → world_pos_to_grid`; вся интеракция тайловая: hover-панель и циклическое выделение берут обитателей тайла из navmesh-индекса, клик по пустому тайлу — приказ движения на тайл. Sprite picking сюда не ложится: (1) клики по земле потребовали бы `Pickable` на тысячах тайловых спрайтов, которые исчезнут в 2.3 (TilemapChunk); (2) picking отдаёт верхний спрайт, а UX показывает всех обитателей тайла — navmesh всё равно остался бы источником; (3) заменяемого кода ~10 строк координатной математики. Текущая схема «координата → grid → пространственный индекс» — идиоматичный подход для тайловых игр.

Если в будущем понадобится по-спрайтовая подсветка при наведении — рассмотреть гибрид: `Pickable` + `Hovered` только на пешках/структурах, тайловая логика без изменений.

<details>
<summary>Исходное ТЗ (не выполнялось)</summary>

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

</details>

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

- [x] Найдены все места `.single().expect(...)` — их оказалось 12 (simulation_state ×2, items_stock ×2, debug/info ×2, debug/tasks_queue ×2, hovered, user_select, input ×2-кратный дубль camera/window).
- [x] Update-системы с маркером → параметр `Single<Entity, With<Marker>>` (8 систем). В `mouse_input` `Single` для камеры и окна заодно убрал дублирование запросов в click-ветке.
- [x] Наблюдатели (`on_hover` в hovered, `on_user_selection` в user_select) — явный `let Ok(..) = query.single() else { return }` вместо `Single`-магии в observer-параметрах.
- [x] `.unwrap()` на `Assets` в noise → `sync_noise_texture` возвращает `Result`, обе вызывающие системы стали fallible (`-> Result` + `?`), `.ok_or(...)?` вместо unwrap.
- [x] `.single_mut()` в `debug/info.rs` (KeyH) → `if ... && let Ok(..)` — при отсутствии help-блока остальные клавиши продолжают работать.

**Критерии:**

- [x] `.expect(` на Query-результатах — 0.
- [x] Поведение UI не изменилось (смоук-тест: hover/click/selection-события и панели работают, clippy чистый, 29 тестов зелёные).

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

- [x] Инвентаризация: группы `pawn_state` (3 тега), `workable_state` (2), `farm_state` (4) — все генерятся макросами; одиночный `MovableStateMovingTag`. У `Commandable` тегов нет (только поле state).
- [x] Написан общий макрос `exclusive_state_tags!` (`src/lib.rs`, tt-muncher): для каждого тега генерит `#[component(immutable, on_add = ...)]`-хук, снимающий всех соседей группы через `DeferredWorld::commands()`. Все три state-мода переведены на него — дублировавшиеся объявления тегов в трёх макросах исчезли.
- [x] Все теги — `#[component(immutable)]`, включая одиночный `MovableStateMovingTag`.
- [x] `remove_old_state_component` удалён из всех трёх `change_state` — старый тег снимает хук нового. `log_state_change!` остался в `change_state` (он логирует enum-переход, не тег).
- [x] `ensure_state!`/`continue_unless!` не потребовали изменений (читают enum-поле, не теги).

**Критерии:**

- [x] Невозможно получить сущность с двумя тегами одной группы — unit-тест `state_tags_are_exclusive` в `src/pawn/components.rs` (30-й тест).
- [x] Существующие переходы работают: смоук-запуск — 569 переходов состояний в логе, 0 паник; same-state ре-вставки (re-insert immutable-тега) легальны, `on_add` при них не дёргается.

**Риски:** хук срабатывает немедленно при вставке — убедиться, что наблюдатели `EntityStateChangeEvent` не зависят от наличия старого тега после вставки нового.

---

## Фаза 2 — состояния, рендер, инфраструктура

### 2.1. `DespawnOnExit` для debug-слоёв

**Справка по API:** `bevy_state::state_scoped` (в prelude): `DespawnOnExit<S: States>(pub S)` и `DespawnOnEnter<S: States>(pub S)` — компоненты; сущность автоматически despawn'ится при выходе из/входе в указанное состояние.

```rust
commands.spawn((DespawnOnExit(DebugNavmeshState::On), Sprite { .. }, ...));
```

**Что сделать:**

- [x] Navmesh-тайлы спавнятся с `DespawnOnExit(DebugNavmeshState::Visible)`; Hidden-ветка наблюдателя с ручным despawn-циклом удалена.
- [x] Noise-mesh спавнится с `DespawnOnExit(<текущее состояние>)` — при переключении Height→Humidity→Props→Hidden старый меш снимает state-машина. `despawn_noise_texture` удалена целиком.
- [x] Бонус: `on_rebuild_map_complete` больше не делает despawn+respawn меша — хэндлы текстуры/материала стабильны, мутация ассетов обновляет видимый меш на месте.
- [x] Grid и movepath — gizmos (не сущности), не трогались.
- [x] Нюанс задокументирован: спавн идёт в наблюдателе `StateChangeEvent` (немедленно при флаше), а `DespawnOnExit` срабатывает в `StateTransition`-расписании — при переключении между noise-слоями старый и новый меш сосуществуют один кадр (незаметно для debug-слоя).

**Критерии:**

- [x] Ручных despawn-систем для debug-слоёв нет; clippy/тесты зелёные, старт без паник. Тумблеры g/n/p — проверить вручную в игре (клавиатурные пути не покрываются headless-смоуком).

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

- [x] Проверены все 7 мест записи `NextState`. Итог: **код не требует изменений.**
  - Тумблеры в `debug/info.rs` (g/n/p/m) — все через `match current { A => set(B), B => set(A) }`, значение всегда другое.
  - Init-замыкания в `noise/mod.rs` и `navmesh.rs` (`OnExit(AppState::Loading)`) — одноразовые, переводят из дефолтного Hidden.
  - `AppState::Quiting` в `main.rs:126` — сразу за ним `AppExit` + `std::process::exit(0)`, второй Esc физически невозможен.

**Критерии:**

- [x] Повторное «включение включённого» невозможно по построению (все записи — переключения); `set_if_neq` не понадобился.

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

**Выполняется в ветке `tilemap-chunk`** (пользователь хочет сравнить производительность с master).

**План (детализирован 2026-07-26, по коммиту на шаг):**

- [x] **Шаг 0 — разведка:** (а) tile-occupants в логике занятости: единственный логический потребитель — `has_occupants_except_of::<Pawn>` в to_rest, и из-за вездесущих `Tile` он **всегда true** (латентный баг: ветка «спать на месте» недостижима); `find_empty_grid_tile` работает по `is_passable`, occupants не смотрит. Удаление Tile-occupants безопасно и чинит баг (поведенческое изменение: пешка сможет спать на месте, если клетка реально пуста). Прочие `get_occupants::<Tile>` — только hover-панель и `on_hover` (мигрируют на ресурс). (б) `HoverMarker` вешается/снимается, но не имеет ни одного потребителя — мёртвый механизм, удаляем вместе с `map::on_hover`. **Выбран вариант B.**
- [x] **Шаг 1 — хранилище тайлов:** вариант B выполнен — ресурс `TileGrid` (плоский `Vec<Tile>`, собственная проверка границ: `Navmesh::is_in_range` не годится — он строгий и исключает крайний столбец −250). `Tile` больше не `Component`, тайл-сущности и `add_occupant::<Tile>` удалены.
- [x] **Шаг 2 — тайлсет:** `src/map/tileset.rs` — array texture из 9 png напрямую через `Image::new` (depth_or_array_layers=9), слой = `TileKind as u16`. Грабли: png разноформатные — 8-бит sRGB и 16-бит; `Image::convert` не читает 16-битные источники → ручная конверсия каналов, причём с linear→sRGB преобразованием (Rgba16Unorm — линейный формат, старый пайплайн рисовал его «как есть»; без преобразования менялась цветовая гамма).
- [x] **Шаг 3 — чанки:** 400 чанков 25×25, компонент `MapChunk{chunk_x,chunk_y}` для rebuild. `TilemapChunkPlugin` НЕ добавлять — уже в DefaultPlugins (паника «already added»). Порядок данных: row-major от НИЖНЕГО-левого тайла (проверено эмпирически — при top-left каждый чанк зеркалится по вертикали). Меш чанка центрирован на Transform.
- [x] **Шаг 4 — rebuild:** `on_rebuild_map` пересобирает `TileGrid` и мутирует `TilemapChunkTileData` всех чанков; сущности живут вечно. (Рантайм-путь rebuild — кнопка в egui-окне перлина; проверить при случае.)
- [x] **Шаг 5 — потребители:** hovered — `Option<Res<TileGrid>>` (события спавна пешек летят до флаша generate_map — жёсткий `Res` роняет валидацию с паникой); debug-noise итерирует ресурс; `HoverMarker` + `map::on_hover` удалены; `get_occupants::<Tile>` — 0; `render_tile_ui` без entity id (заголовок — координата), новый хелпер `render_node_bunlde`.
- [x] **Шаг 6 — верификация:** 30 тестов, clippy чистый. Пользователь подтвердил визуально: карта корректна, hover работает, цвета совпали с master. **Результаты: FPS ×2–3, память приложения 1.58GB → ~755MB.** Noise-циклы (`p`) и rebuild из egui — проверить при игре.

**Риски:** логика занятости (гл. риск → вариант A); png тайлов должны быть одного размера; стыки чанков (nearest-фильтрация уже стоит — ок).

**Критерии:**

- [ ] Визуально карта идентична (включая debug-оверлеи noise поверх).
- [ ] Количество сущностей на старте упало на ~размер карты.
- [ ] Rebuild карты работает.

**Риски:** самая инвазивная задача фазы — рендер и структура данных карты связаны с navigation и debug-слоями. Допустимо временно оставить старый путь за cfg/константой до стабилизации.

### 2.4. Reflect auto-registration: убрать ручные `register_type`

**Справка по API:** фича `reflect_auto_register` включена по умолчанию (входит в `default_app`). Все конкретные типы с `#[derive(Reflect)]` регистрируются автоматически при старте приложения. **Исключение — generic-типы**: их конкретизации (`EntityStateChangeEvent<PawnState>` и т.п.) по-прежнему требуют ручного `register_type::<...>()` (цитата из доков: "this does not apply to types with generics: their desired monomorphized representations must be registered manually").

**Что сделать:**

- [x] Удалены все 14 активных вызовов `.register_type::<T>()` (12 файлов) — все были для конкретных типов.
- [x] Generic-конкретизаций, требующих ручной регистрации, не оказалось (`EntityStateChangeEvent<T>` — событие, не компонент, в реестре типов не нуждается).
- [x] Проверено через BRP (`world.query` по строковым именам типов): `Pawn`, `Feedable`, `Bed`, `Carryable`, `BedOwnedBy`, `Tile` (250k строк) резолвятся — реестр типов полный; инспектор использует тот же `TypeRegistry`. Заодно подтверждено, что BRP-методы в 0.19 переименованы: `bevy/query` → `world.query`.
- [x] Попутно удалён опустевший `StoragePlugin` (в нём оставался только register_type).
- [x] Для 2.8: `reflect_auto_register` входит в `default_app`, который входит в `2d` — не потеряется.

**Критерии:**

- [x] BRP работает (проверено запросами), ручных `register_type` — 0.

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

- [x] Разобраны все 12 мест. Итог: настоящих конверсий три — остальные девять уже были на `children![...]` внутри, а их внешний `with_children` **легитимен**: `render_*_ui`-хелперы дописывают секции в существующий общий контейнер (`&mut EntityCommands`), куда несколько систем добавляют детей по очереди — `children![]` для этого не предназначен (он спавнит вместе с бандлом).
- [x] Конвертированы: `simulation_state` (spawn + with_children → один spawn с `children![]`), `items_stock` (`spawn_item(&mut root, ...)` переписан в `item_node(...) -> impl Bundle` + `children![]` в корне), `storage` (двухуровневая вложенность замыканий → `children![]`-дерево в стиле остальных панелей).
- [x] `SpawnIter`/`SpawnWith` не понадобились — логики в замыканиях не было.

**Критерии:**

- [x] UI-дерево идентично: смоук-запуск без паник, BRP насчитал те же 14 `Text`-нод; замыкания остались только для дописывания в общий контейнер.

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

- [x] `Text2d` над пешками — решено НЕ переводить на `Gizmos::text_2d`: это игровой UI (всегда включён, кастомный шрифт fira, событийное обновление через `Text2dWriter`), а гизмос-текст — ASCII stroke-шрифт с перерисовкой каждый кадр, т.е. даунгрейд. API `gizmos.text_2d(...)` остаётся в справке для будущих чисто отладочных подписей.
- [x] `FpsOverlayPlugin` включён за фичей `debug_ui` (14px, полупрозрачный белый, `..default()` для новых полей 0.19 — `refresh_interval`, `frame_time_graph_config`). Мёртвый закомментированный блок удалён.
- [x] Дополнительно по просьбе пользователя: оверлей сдвинут на 100px вправо и 10px вниз (у `FpsOverlayConfig` нет поля позиции — система `offset_fps_overlay` в `PostStartup` находит корень по `GlobalZIndex == FPS_OVERLAY_ZINDEX` и ставит `left`/`top`). Проверено пользователем визуально.

**Критерии:**

- [x] FPS-оверлей включается в debug-сборке (проверено BRP: нода оверлея существует, `left = 100px`); мёртвый закомментированный код удалён.

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

- [x] Инвентаризация 47 `add_observer`: гейтить — геймплейные (команды, смерть, фермы, навигация, задачи, carryable, hover/selection UI); не гейтить — инвариантные счётчики и инфраструктуру.
- [x] 40 наблюдателей получили `.run_if(in_state(AppState::Playing))` через `ObserverSystemExt` (он уже в prelude, импорты не понадобились).
- [x] Без гейта осознанно (7): счётчики кроватей `on_bed_claimed/unclaimed` (инвариант должен работать всегда), debug-слои noise ×2 / navmesh ×1 (инициализируются из `OnExit(Loading)`), map `on_rebuild_map`/`on_hover` (инфраструктура карты).
- [x] Проверен тонкий момент: события стартовой сцены (`SpawnCarryableEvent`, `ScheduleTaskEvent`, фермы) триггерятся из `OnExit(AppState::Loading)` — к моменту флаша `State<AppState>` уже `Playing`, гейт их пропускает (подтверждено смоуком: еда заспавнилась, 24 задачи, пешки работают).
- [x] Попутно починен тест `state_changed_to_death_by_event`: тестовый App теперь добавляет `StatesPlugin` + `insert_state(AppState::Playing)` — иначе гейт корректно молчит (что тест и поймал — гейтинг работает).

**Критерии:**

- [x] Триггер геймплейного события вне `Playing` не выполняет логику (доказано упавшим-и-починенным тестом). Поведение в `Playing` не изменилось (смоук: полный цикл задач/работы/еды жив).

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

- [x] `bevy`: `default-features = false`, `features = ["2d", "ui", "dynamic_linking", "bevy_dev_tools", "bevy_remote"]`. Выброшены `3d` и `audio` (звука в проекте нет). `sprite_picking` не включён (1.4 закрыт); гизмосы/png/шрифты/winit/state приходят внутри `2d`+`ui`.
- [x] Ничего доключать не пришлось — `2d`+`ui` покрыли всё с первой сборки.
- [x] Сторонние крейты проверены: `bevy-inspector-egui` дефолтами тянул весь 3d-стек (`bevy_pbr`) через свою фичу `bevy` — переведён на `default-features = false, features = ["2d", "bevy_gizmos", "documentation", "egui_clipboard"]`. Остальные (pancam, asset_loader, egui) чисты.
- [x] Известное ограничение: `bevy_pbr` всё же остаётся в графе — это **жёсткая** (не optional) зависимость `bevy_dev_tools` 0.19; уйдёт только вместе с FPS-оверлеем.
- [x] Замер: граф зависимостей 585 → 551 крейта (−34, включая bevy_audio, bevy_gltf, rodio-стек). Полная пересборка после смены фич — 6м00с (M1 Max, dev + opt-level 3 для зависимостей). Честного replaу «до» нет — базовую чистую сборку прервал пользователь как слишком долгую; сопоставимая метрика — число крейтов.

**Критерии:**

- [x] Игра собирается и работает: смоук — окно/рендер/egui/BRP живые, еда спавнится, пешки берут задачи и работают (7 `Idle => BeingWorked`), 15 UI-нод, паник нет. 3D-стек вырезан из bevy и инспектора (кроме `bevy_pbr`, приколоченного к dev_tools).

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

- [x] Кандидаты разобраны (progress_hunger, progress_fatigue, move_moving_entities, progress_work, progress_pawn_dying). Фильтры совместимы (With/Without), sparse-компонентов нет — НО тела всех систем — ветвления с `Commands`/событиями на сущность, а не плотная арифметика: слайсы там ничего не дают.
- [x] **Решение: системы НЕ конвертировать.** Вместо этого в репо оставлен образец применения — `tests/contiguous_bench.rs` (`cargo test --test contiguous_bench -- --ignored --nocapture`).
- [x] Бенчмарк (release, M1 Max, 2000 сущностей, чистая арифметика hunger/fatigue): обычный `iter_mut` 1.96µs/проход, `contiguous_iter_mut` 1.12µs/проход — **×1.75**, но абсолютная экономия ~0.8µs на тик (тики раз в 0.25с) — не влияет ни на что. Пригодится, если появится массовая плотная математика по 10k+ сущностей каждый кадр.

**Критерии:**

- [x] Поведение тиков не менялось вовсе (код систем не тронут); 30 тестов зелёные.
- [x] Числа записаны в «Решения».

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

- [x] `bevy_ui_widgets` уже был включён (входит в коллекцию `ui`), а `UiWidgetsPlugins` уже в DefaultPlugins 0.19 (повторное добавление — паника «already added»). `bevy_feathers` НЕ включался — осознанно: это BSN-сцены и своя тема, для debug-панелей избыточно; headless-виджетов достаточно для вердикта.
- [x] Прототип — новая панель `src/ui/debug/toggles.rs`: 4 кнопки (grid/navmesh/noise/movepath) на `bevy_ui_widgets::Button`, каждая с entity-scoped `.observe(On<Activate>)` (первое применение возможности, открытой в 1.2). Статус — подсветка активной кнопки + текст текущего noise-слоя (система `update_toggle_buttons` читает те же `State<...>`, что и клавиши — источник истины один). `Checked` не использовался: наши тумблеры — это bevy-стейты, а не UI-состояние.
- [x] Сравнение сделано (см. вердикт в «Решения»). Известное ограничение: клик по bevy-ui кнопке проходит и в мир (ClickStage0) — bevy-ui не участвует в `EguiWantsFocus`-гейте; для debug-кнопок терпимо, для игрового UI понадобится гейт по `Interaction`/hover.
- [x] Вердикт записан в «Решения». Панель тумблеров остаётся в игре как рабочая фича, не только прототип.

**Критерии:**

- [x] Панель работает (подтверждено пользователем: клики и индикация).
- [x] Вердикт записан.

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

- [x] `RootConfig` вставляется как Resource при старте (клон того же снапшота, что уходит в `OnceLock`). Всё дерево конфига получило `Reflect` (+`NoiseDistortion`, `DebugNoiseState`), `RootConfig`/`MapGeneratorConfig` — `#[reflect(Resource)]` для BRP. `config()` оставлен.
- [x] Вместо lifecycle-наблюдателя — система `on_root_config_changed` с `run_if(resource_changed::<RootConfig>)`: BRP-мутации меняют ресурс на месте (Insert не срабатывает), а `resource_changed` ловит и вставку, и мутации. Внутри `bypass_change_detection()` — иначе пересчёт зациклил бы сам себя. Попутно исправлена неидемпотентность `RestableConfig::calculate_derived_fields` (делил raw-поля на месте — повторный пересчёт портил значения): введены отдельные `*_per_second` derived-поля, потребители переведены.
- [x] Семантика задокументирована в `config.rs`: static — неизменяемый стартовый снапшот, Resource — живой источник для инспектора/BRP, рантайм-правки видят только читатели `Res<RootConfig>`; static честно не обновляется.
- [x] Зафиксировано в док-комментарии `RootConfig`.

**Критерии:**

- [x] Проверено через BRP: `world.get_resources` отдаёт всё дерево, `world.mutate_resources` меняет поля.
- [x] Живой тест: мутация `food_consumption_per_day` 1→6 → `hunger_drain_per_hour` 0.4167→2.5 (ровно ×6), лог «RootConfig changed» без зацикливания, `resting_cost_per_second` не дрейфует.
- [x] Работают (30 тестов, смоук).

**Риски:** двойной источник правды static vs Resource — обязательно явно задокументировать выбранную семантику в `config.rs`.

---

## Фаза 4 — структура модулей и система импортов

Отдельная от Bevy-модернизации тема: соответствие структуры проекта конвенциям Rust. **Разбивка по каталогам корректна** (фича-модуль = плагин, внутри `components.rs`/`systems.rs` — идиоматично для Bevy; один крейт на 9.5k строк — верно; `Cargo.toml` с `default-features = false` и точечными фичами — аккуратно). Проблема — исключительно в **схеме импортов**.

### Диагностика (причина, а не симптом)

`use_modules!` (`src/lib.rs:7`, вызов `:25`) разворачивается для каждого из 21 top-level модуля в:

```rust
pub mod pawn;
pub use crate::pawn::*;   // ← вот это и есть корень проблемы
```

`expose_submodules!` (`src/lib.rs:16`) делает `mod x; pub use self::x::*;` — поднимает содержимое `components.rs`/`systems.rs` в `mod.rs`. Итог: **почти каждый `pub`-элемент крейта живёт в одном плоском namespace `zxc::`**.

Далее 68 файлов делают `use super::*`. По правилам видимости приватный `use crate::*;` в родительском `mod.rs` создаёт приватные привязки, видимые во всех дочерних модулях, и `use super::*` их подхватывает. Цепочка: лист → `super::*` → `crate::*` → bevy prelude + все 21 модуль. Поэтому `src/movable/systems.rs` имеет ровно один импорт `use super::*` и при этом использует `Commands`, `Query`, `Res<Time>`, `Pawn`, `ArcNavmesh`.

Текущая раскладка импортов на 88 файлов модулей: 68 × `use super::*`, 18 × `use crate::*`, 1 файл вообще без импортов (`src/pawn/debug_components.rs`), явных путевых импортов внутри крейта — один (`src/config.rs:6`).

Конвенция Rust: **каждый файл объявляет свои импорты явно**; glob допустим только для `prelude` внешних крейтов и в тестовых модулях (`use super::*` внутри `#[cfg(test)] mod tests` — нормально и остаётся).

Последствия — не гипотетические:

1. **Коллизии имён уже есть.** Три:

   | имя | места |
   |---|---|
   | `generate` | `src/map/generator/empty.rs:3`, `src/map/generator/perlin_noise.rs:42` |
   | `on_new_day` | `src/pawn/systems.rs:117`, `src/structure/farm/systems.rs:156` |
   | `on_pawn_death` | `src/pawn/systems.rs:152`, `src/movable/systems.rs:119` |

   Rust не диагностирует glob-коллизию до момента использования имени. Поэтому в `src/pawn/mod.rs:9-10` уже пришлось писать `self::systems::on_pawn_death` / `self::systems::on_new_day` — это обход проблемы, созданной схемой импортов. С ростом кода такие ошибки будут вылезать не там, где причина.

2. **Источник имени не виден.** В `src/ai/mod.rs` встречаются `pawn_state::PawnStateIdleTag`, `ArcNavmesh`, `Commandable` — по коду не определить, откуда. Grep не помогает, работает только LSP.

3. **`dead_code` не работает.** Всё `pub` + реэкспортировано в корень → компилятор считает всё используемым. Мёртвые функции не находятся, сузить API модуля до `pub(crate)` нельзя.

4. **Скрытые циклы зависимостей.** `src/config.rs:6` — `use crate::DebugNoiseState;`: базовый конфиг тянет тип из `ui/debug/noise/`. Видно только потому, что там единственный явный импорт; остальные такие связи глобы прячут.

### 4.1. Разгрузить `lib.rs`

**Цель:** `lib.rs` — карта крейта (объявления модулей + prelude), а не файл с логикой. Сейчас там 193 строки: 2 макроса объявления модулей, 2 типа событий, `AppState`, 6 экспортируемых макросов.

- [ ] `src/macros.rs` (или `src/macros/mod.rs`, если разрастётся): `ensure_state!` (`:64`), `continue_unless!` (`:115`), `return_unless!` (`:132`), `exclusive_state_tags!` (`:153`), `log_state_change!` (`:180`), `log_event!` (`:187`). Внимание: `#[macro_export]` кладёт макрос в корень крейта независимо от файла, так что пути вызова (`crate::exclusive_state_tags!`) не меняются; но `mod macros;` должен быть объявлен **до** модулей, которые макросы используют, — иначе для не-`macro_export` макросов будет ошибка порядка. Проще всего: `#[macro_use] mod macros;` первой строкой либо оставить все макросы `#[macro_export]`, как сейчас.
- [ ] `src/state.rs`: `AppState` (`:55`).
- [ ] `src/events.rs`: `StateChangeEvent<T>` (`:49`), `EntityStateChangeEvent<T>` (`:52`).
- [ ] В `lib.rs` остаются: объявления модулей, `pub mod prelude` (см. 4.3) и больше ничего.

**Критерии:** `lib.rs` < 60 строк; сборка и 30 тестов зелёные; диффа поведения нет.

**Риски:** порядок объявления модулей относительно макросов — единственная реальная ловушка. Если сборка ломается на «cannot find macro», добавить `#[macro_use]` перед `mod macros;` и поднять его выше всех прочих `mod`.

### 4.2. Явные `mod`/`pub use` вместо макросов объявления модулей

**Цель:** вернуть IDE-навигацию по модулям, `cargo doc` и грепаемость. `use_modules!`/`expose_submodules!` экономят ~20 строк, но ломают «go to module», прячут структуру крейта от читателя и от инструментов. В Rust список `mod` — это не бойлерплейт, а карта.

- [ ] Раскрыть `use_modules!` вручную в `lib.rs`: 21 строка `pub mod x;` (реэкспорты `pub use crate::x::*;` пока **оставить** — их снимает 4.5, иначе всё сломается сразу).
- [ ] Раскрыть `expose_submodules!` в 20 `mod.rs`-файлах: `mod components; pub use self::components::*;` и т.д. Список файлов: `grep -rln 'expose_submodules!' src/`.
- [ ] Удалить оба `macro_rules!` из `lib.rs`.
- [ ] Проверить, что диффа в собранном API нет: `cargo build` + тесты.

**Критерии:** `grep -rn 'use_modules!\|expose_submodules!' src/` — 0 совпадений; сборка идентична по поведению.

**Риски:** нулевые — механическая развёртка, компилятор поймает опечатки.

### 4.3. `crate::prelude` — явный и короткий

**Цель:** дать один законный glob вместо двадцати одного. В prelude попадает только то, что нужно **действительно почти везде**, а не «всё сразу».

```rust
// src/prelude.rs
pub use bevy::prelude::*;
pub use crate::{
    config::{config, RootConfig, TILE_Z_INDEX, PAWN_Z_INDEX /* ... */},
    events::{EntityStateChangeEvent, StateChangeEvent},
    state::AppState,
};
```

- [ ] Создать `src/prelude.rs`, объявить `pub mod prelude;` в `lib.rs`.
- [ ] Кандидаты в prelude — по факту частоты использования, а не «на всякий случай». Явно **не** включать: `Pawn`, `Movable`, `ArcNavmesh`, UI-хелперы и прочее доменное — это импортируется по месту.
- [ ] Убрать из `lib.rs` корневые `pub use bevy::prelude::*;` (`:1`), `pub use bevy_asset_loader::prelude::*;` (`:2`), `pub use bevy_inspector_egui::prelude::*;` (`:3`), `pub use std::sync::LazyLock;` (`:4`) — крейт не должен реэкспортировать чужие prelude в свой корень: при апгрейде Bevy изменения его prelude поедут по всему коду непредсказуемо. `bevy_asset_loader` нужен фактически только в `main.rs` и `assets.rs` — импортировать там точечно. `bevy_inspector_egui::prelude::*` — только в файлах, где реально используется `InspectorOptions`.
- [ ] `main.rs`: `use zxc::*;` → `use zxc::prelude::*;` + явные импорты плагинов и того, что нужно `close_on_esc`.

**Критерии:** `src/prelude.rs` — явный список без `crate::<module>::*`; `main.rs` не использует `use zxc::*`.

**Риски:** есть соблазн сделать prelude свалкой и получить ту же проблему под новым именем. Правило: если элемент нужен < 10 файлам — он не в prelude.

### 4.4. Проход A: явные импорты во всех файлах модулей (основная работа)

**Цель:** убрать зависимость файлов от плоского корневого namespace, **не ломая сборку ни на одном шаге**. Корневые реэкспорты на этом этапе остаются на месте — они и обеспечивают инкрементальность.

Порядок важен: если сначала снять реэкспорт из корня (4.5), сломаются все модули-потребители одновременно. Поэтому сначала переводим файлы на канонические пути, и только потом убираем реэкспорты — тогда 4.5 собирается с первой попытки.

**Процедура на один файл** (на примере `src/movable/systems.rs`):

1. Заменить `use super::*;` на `use crate::prelude::*;`.
2. `cargo build` → компилятор перечислит все неразрешённые имена с готовой подсказкой `help: consider importing this struct` и корректным путём.
3. Добавить явные импорты по подсказкам: `use crate::{navigation::ArcNavmesh, pawn::Pawn};`, `use super::{Movable, MovableStateMovingTag};` (внутримодульное — через `super`, это нормально и остаётся).
4. Использовать **канонический** путь `crate::pawn::Pawn`, а не корневой реэкспорт `crate::Pawn`, — иначе 4.5 сломает импорт заново. Важно: сабмодули объявлены приватными (`mod components;`), поэтому канонический путь — `crate::pawn::Pawn`, а не `crate::pawn::components::Pawn`.
5. `#[cfg(test)] mod tests { use super::*; }` (`src/pawn/components.rs:127`) — **не трогать**, это идиоматично.

**Порядок обхода модулей** (от наименее связанных к самым связанным, чтобы разбираться с растущим числом импортов постепенно):

- [ ] Одиночные файлы: `daylight.rs`, `camera.rs`, `async_queue.rs`, `config.rs`, `assets.rs`. Здесь же исправить `src/camera.rs:1` — `use super::*` в top-level модуле (`super` == корень крейта) — неконсистентно, должно быть `use crate::prelude::*`.
- [ ] `story_time/`, `tasks_queue/`
- [ ] `map/` (включая `generator/`), `navigation/`
- [ ] `carryable/`, `workable/`, `restable/`, `feedable/`, `movable/`
- [ ] `structure/` (включая `farm/`, `bed/`, `storage/`)
- [ ] `pawn/`, `commandable/` (включая 9 `*_command/`), `ai/`
- [ ] `user_selection/`, `input/`
- [ ] `ui/` — самый связанный, идёт последним

Коммит на модуль (для `ui/` и `commandable/` — на подмодуль). Каждый коммит должен собираться и проходить тесты.

**Измерение прогресса:**

```bash
cargo clippy -- -W clippy::wildcard_imports 2>&1 | grep -c 'wildcard_imports'   # должно падать до 0
grep -rn 'use super::\*;\|use crate::\*;' src/ | grep -v 'cfg(test)'            # остаточный список
```

_(проверить при выполнении: `clippy::wildcard_imports` по умолчанию не ругается на пути, последний сегмент которых — `prelude`; если ругается — настроить исключение в `clippy.toml`.)_

**Критерии:**

- [ ] `grep -rn 'use crate::\*;' src/` — 0 совпадений.
- [ ] `use super::*` остаётся только в `#[cfg(test)] mod tests`.
- [ ] Поведение не менялось: 30 тестов зелёные, игра запускается (smoke-тест по скиллу `live-app`).

**Риски:** объём. 88 файлов — это не один сеанс. Обязательно коммитить по модулю; недоделанная фаза в любой момент — валидное состояние репо (реэкспорты ещё на месте, всё собирается).

### 4.5. Проход B: снять плоские реэкспорты и сузить видимость

Делать **только после** полного завершения 4.4.

- [ ] Удалить все 21 `pub use crate::<module>::*;` из `lib.rs`. Сборка должна пройти сразу — если нет, значит какой-то файл в 4.4 пропущен; чинить его, а не откатывать.
- [ ] Прогнать `cargo clippy -- -D warnings` — появится волна новых предупреждений `dead_code`, ранее подавленных реэкспортами. Разобрать: удалить мёртвое, оставшееся не-публичное сузить `pub` → `pub(crate)` → приватное.
- [ ] Пересмотреть фасады модулей: `pub use self::systems::*;` обычно не нужен вовсе — системы регистрируются внутри `mod.rs` того же модуля. Заменить на `mod systems;` без реэкспорта там, где системы не вызываются снаружи.
- [ ] Разобраться с находкой №4 из диагностики: `src/config.rs:6` `use crate::DebugNoiseState;` → `use crate::ui::debug::noise::DebugNoiseState;`, и решить, правильно ли базовый конфиг зависит от UI. Скорее всего `DebugNoiseState` должен переехать в `config` (или стать своим типом там), а UI — его использовать, а не наоборот.
- [ ] Закрепить результат: добавить в `Cargo.toml` `[lints.clippy] wildcard_imports = "warn"` (или `"deny"`), чтобы проблема не вернулась.

**Критерии:**

- [ ] `lib.rs` не содержит ни одного `pub use crate::...::*`.
- [ ] `cargo clippy -- -D warnings` чист.
- [ ] Число `pub` элементов заметно упало (зафиксировать до/после: `grep -rc '^pub fn \|^pub struct \|^pub enum ' src/ | ...`).
- [ ] Smoke-тест игры пройден.

**Риски:** самый вероятный сюрприз — `dead_code` укажет на функции, которые на самом деле живые, но вызываются только через строковую регистрацию/рефлексию (Bevy `register_type`, BRP). Не удалять слепо: проверять `grep` по имени и по имени типа перед удалением.

### 4.6. Развести коллизии имён (по желанию, но рекомендуется)

После 4.4 коллизии перестают быть ошибками — импорты явные. Но одинаковые имена в разных модулях всё равно мешают грепу и чтению стектрейсов/логов.

- [ ] `on_new_day` (`pawn/systems.rs:117`, `structure/farm/systems.rs:156`) → например `on_new_day_age_pawns` / `on_new_day_grow_farms`.
- [ ] `on_pawn_death` (`pawn/systems.rs:152`, `movable/systems.rs:119`) → второй по смыслу `on_pawn_death_release_occupation` (или как окажется по коду).
- [ ] `generate` (`map/generator/empty.rs:3`, `perlin_noise.rs:42`) — **оставить как есть**: это две реализации одного контракта, вызываются как `empty::generate()` / `perlin_noise::generate()`, имя корректно квалифицируется модулем. Единственное имя из трёх, которое коллизией не является по сути.
- [ ] После переименований убрать костыль `self::systems::` в `src/pawn/mod.rs:9-10`.

**Критерий:** `grep -rhn '^pub fn \|^pub struct \|^pub enum \|^pub trait ' src/ | sed -E 's/.*pub (fn|struct|enum|trait) ([A-Za-z0-9_]+).*/\2/' | sort | uniq -d` — только `generate`.

### 4.7. Одинокие `x/mod.rs` → `x.rs`

**Цель:** убрать 26 каталогов, содержащих ровно один `mod.rs` без сабмодулей. Плюс сам стиль `mod.rs` считается устаревшим начиная с Rust 2018 — предпочитают `foo.rs` рядом с каталогом `foo/`.

Полный список получить так:

```bash
find src -name mod.rs | while read f; do d=$(dirname "$f");
  n=$(find "$d" -maxdepth 1 -name '*.rs' | wc -l | tr -d ' ');
  s=$(find "$d" -mindepth 1 -maxdepth 1 -type d | wc -l | tr -d ' ');
  [ "$n" = 1 ] && [ "$s" = 0 ] && echo "$f"; done
```

Сейчас это: `input/`, `feedable/`, `tasks_queue/`, `restable/`, `ai/`, все 9 `commandable/*_command/`, все 10 `ui/*/`, `structure/bed/`, `structure/storage/`.

- [ ] Переименовать через `git mv src/ai/mod.rs src/ai.rs && rmdir src/ai` (и т.д. по списку) — `git mv` сохранит историю файла.
- [ ] Модули с сабмодулями (`map/`, `navigation/`, `pawn/`, `structure/`, `ui/`, `ui/debug/`, `commandable/`, `movable/`, `story_time/`, `carryable/`, `workable/`, `user_selection/`) — по желанию тоже перевести на форму `pawn.rs` + `pawn/`; это уже вкусовщина, есть clippy-линты в обе стороны (`mod_module_files` / `self_named_module_files`). Решить один раз и применить единообразно.
- [ ] Правки в `mod`-объявлениях не нужны — путь `mod ai;` резолвится в оба варианта.

**Критерий:** одиноких `mod.rs` нет; сборка зелёная; выбранный стиль применён единообразно.

### 4.8. Гигиена импортов (мелочи)

- [ ] `pub use std::sync::LazyLock;` объявлен дважды — `src/lib.rs:4` и `src/config.rs:3`. Оставить один (или ни один — импортировать по месту).
- [ ] `pub use bevy_inspector_egui::prelude::*;` (`src/lib.rs:3`) — безусловный, хотя это инструмент отладки. Гейтить `#[cfg(feature = "debug_ui")]` либо импортировать точечно в файлах с `InspectorOptions`. Проверить, что сборка `--no-default-features` при этом проходит (`derive(InspectorOptions)` в `components.rs` тоже придётся гейтить — оценить объём до начала).
- [ ] `src/pawn/debug_components.rs` — единственный файл без импортов вообще; после 4.4 должен получить свои явные `use`.

### 4.9. Побочная находка (не про импорты)

- [ ] `src/main.rs`, `close_on_esc`: `next_state.set(AppState::Quiting)` и `event_writer.write(AppExit::Success)` ничего не успевают сделать — следующая строка `std::process::exit(0)` убивает процесс без размотки стека и без `Drop`. Либо убрать `exit(0)` и дать Bevy завершиться штатно через `AppExit` (тогда `AppState::Quiting` тоже отработает), либо убрать первые два вызова как мёртвый код. Первое предпочтительнее: async-задачи пафиндинга и файловые хендлы закроются корректно.

---

## Порядок и зависимости

- Фаза 0 — сразу, одним коммитом.
- 1.1, 1.5, 1.6, 2.1, 2.2, 2.4, 2.5, 2.6 — независимые, можно в любом порядке.
- 1.2 (EntityEvent) делать до 2.7 (run conditions на наблюдателях) — чтобы не переделывать регистрацию дважды.
- ~~1.4 (picking) требует правку `Cargo.toml` — удобно объединить по времени с 2.8 (features `2d`)~~ — 1.4 закрыт как не применимо; `sprite_picking` в 2.8 не включать.
- 1.3 (relationships) лучше после 1.2 — события уже несут target штатно.
- 2.3 (TilemapChunk) — самая инвазивная, делать изолированно от других задач.
- Фаза 3 — в конце; 3.2 — исследовательская.
- Фаза 4 — независима от фаз 0–3 (это про Rust-конвенции, а не про Bevy API), но внутри фазы порядок **жёсткий**:
  - 4.1 и 4.2 — независимы друг от друга, можно первыми, риск нулевой.
  - 4.3 (prelude) — обязательно до 4.4, иначе не на что заменять глобы.
  - 4.4 → 4.5 — строго в этом порядке. Снять реэкспорты раньше = сломать всё сразу.
  - 4.6 — можно до или после 4.4 (независима).
  - 4.7 и 4.8 — в любой момент.
  - 4.9 — отдельная мелочь, к фазе отношения не имеет.
- Фазу 4 лучше делать после 3.3, а не параллельно: 3.3 меняет `config.rs`, который в 4.5 тоже правится (зависимость от `DebugNoiseState`).

## Прогресс по задачам

- [x] 0.1 Удалить закомментированный легаси Message-эпохи
- [x] 0.2 Удалить `log_message!`
- [x] 0.3 Переименовать `render_debug_ui_window_node_bundle`
- [x] 0.4 `Val::Px` → `px()`/`percent()` (не применимо, см. «Решения»)
- [x] 0.5 Опечатка `DebugUiContainerarker`
- [x] 1.1 `#[require(...)]` для Pawn
- [x] 1.2 `EntityEvent`-миграция
- [x] 1.3 Кастомные relationships
- [x] 1.4 bevy_picking — закрыто как не применимо (тайловая интеракция, см. пункт 1.4)
- [x] 1.5 Fallible systems + `Single<T>`
- [x] 1.6 Hooks + immutable для `*Tag`
- [x] 2.1 `DespawnOnExit` для debug-слоёв
- [x] 2.2 Аудит `NextState` (поведение 0.18) — изменений не потребовалось
- [x] 2.3 `TilemapChunk` (ветка `tilemap-chunk`)
- [x] 2.4 Reflect auto-registration
- [x] 2.5 `children![...]`
- [x] 2.6 `Gizmos::text_2d` + `FpsOverlayPlugin`
- [x] 2.7 Run conditions на наблюдателях
- [x] 2.8 Feature-коллекция `2d`
- [x] 3.1 `contiguous_iter` + бенчмарк — системы не конвертированы (обоснованно), бенчмарк в tests/
- [x] 3.2 Прототип headless-виджетов/Feathers
- [x] 3.3 Resources-as-entities для `RootConfig`
- [ ] 4.1 Разгрузить `lib.rs` (макросы / стейт / события по файлам)
- [ ] 4.2 Явные `mod`/`pub use` вместо `use_modules!`/`expose_submodules!`
- [ ] 4.3 `crate::prelude` + убрать реэкспорт чужих prelude из корня
- [ ] 4.4 Проход A: явные импорты в 88 файлах модулей (основная работа, коммит на модуль)
- [ ] 4.5 Проход B: снять плоские реэкспорты, сузить `pub`, разобрать `dead_code`
- [ ] 4.6 Развести коллизии `on_new_day` / `on_pawn_death`
- [ ] 4.7 26 одиноких `x/mod.rs` → `x.rs`
- [ ] 4.8 Гигиена: дубль `LazyLock`, cfg-гейт inspector prelude
- [ ] 4.9 `std::process::exit(0)` в `close_on_esc`

## Решения

- **1.4 (bevy_picking)**: закрыто как не применимо — интеракция в игре тайловая (navmesh-индекс), «ручного hit-test» для замены не существует, а picking по тайлам конфликтует с 2.3. Подробное обоснование в теле пункта 1.4.
- **0.4 (Val::Px → px())**: не применимо для статиков `UI_SCREEN_EDGE_PX_OFFSET`/`UI_WINDOWS_GAP` — `px()` не является `const fn`, статики требуют const-инициализатор. `Val::Px` там остаётся намеренно. Остальной UI-код уже использует `px()`.

- **2.8 (features)**: зависимости 585 → 551 крейта; выброшены 3d (кроме `bevy_pbr` — жёсткая зависимость dev_tools) и audio. Пересборка после смены фич: 6м00с.

- **2.3 (TilemapChunk)**: −250k сущностей (тайлы стали ресурсом `TileGrid` + 400 чанк-мешей), FPS ×2–3 (замер пользователя), память 1.58GB → ~755MB. Побочный багфикс: `has_occupants_except_of::<Pawn>` перестал быть всегда-true — пешка теперь может спать на месте, если клетка реально пуста.

- **3.1 (contiguous_iter)**: чистая итерация ×1.75 быстрее (1.96µs → 1.12µs на 2000 сущностей), но тик-системы — ветвления с Commands, абсолютный выигрыш ~0.8µs/тик — конверсия не оправдана. Бенчмарк-образец: `tests/contiguous_bench.rs`.

- **3.2 (виджеты)**: вердикт — **остаёмся на egui для сложных панелей** (инспектор, будущие таблицы/деревья: immediate mode даёт состояние и layout бесплатно), но **первопартийные headless-виджеты пригодны для простых игровых кнопок**: ноль новых зависимостей, `Activate` + entity-scoped `.observe` читаются отлично, стиль — свои же UI-хелперы. Панель тумблеров (`ui/debug/toggles.rs`) оставлена в игре. Feathers (BSN) не пробовали — пересмотреть, когда BSN станет основным способом описания UI. Ограничение: bevy-ui клики проходят в мир (нет аналога `EguiWantsFocus`-гейта).

- **Фаза 4 (структура и импорты)**: аудит от 2026-07-26. Вердикт — структура каталогов идиоматична и остаётся, схема импортов является анти-паттерном и переделывается. Причина: `use_modules!` реэкспортирует все 21 модуль в корень крейта плоско, `use super::*` в 68 файлах транзитивно затягивает весь этот namespace. Цена уже уплачена: 3 коллизии имён, костыль `self::systems::` в `pawn/mod.rs:9-10`, нерабочий `dead_code`, скрытая зависимость `config.rs` → `ui/debug/noise`. Метод — два прохода: сначала явные импорты по каноническим путям при живых реэкспортах (всегда собирается), потом снятие реэкспортов одним коммитом.

_(осталось: семантика static vs Resource (3.3); вся Фаза 4)_
