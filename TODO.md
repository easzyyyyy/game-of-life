# Game of Life - Plan d'Implémentation

Ce document décrit les phases de refactoring prévues pour améliorer l'architecture du projet.

## 📋 Vue d'ensemble

**Objectif** : Séparer le thread de simulation du thread de rendu pour permettre :

- Une simulation indépendante tournant à vitesse variable (1-30 gen/s)
- Un rendu fluide constant à 60 FPS
- Support futur d'une grille infinie

**Principe** : Chaque phase produit du code fonctionnel, testable séparément.

---

## Phase 1 - UI Separation (~2-3h)

**Objectif** : Organiser le code UI en fichiers séparés avec responsabilités claires.

### Structure cible

```
src/ui/
├── mod.rs          // App principal, orchestration
├── grid_view.rs    // Rendu de la grille
├── controls.rs     // Boutons, sliders
└── stats.rs        // FPS, génération
```

### Tâches

- [ ] Créer `src/ui/mod.rs` avec `App` et imports publics
- [ ] Créer `src/ui/grid_view.rs`
  - Structure `GridView { camera, cell_size }`
  - Méthode `render(&self, ui, game)`
  - Méthode `handle_input(&mut self, ui) -> Option<(usize, usize)>` pour clics
- [ ] Créer `src/ui/controls.rs`
  - Enum `ControlEvent { Play, Pause, StepForward, StepBack, SpeedChanged(f32), ZoomChanged(f32) }`
  - Fonction `render(ui, is_playing, speed, zoom, can_step_back) -> Vec<ControlEvent>`
- [ ] Créer `src/ui/stats.rs`
  - Structure `StatsData { fps, generation_count }`
  - Fonction `render(ctx, stats)`
- [ ] Adapter `App::update()` pour utiliser les nouveaux modules
- [ ] Mettre à jour `src/main.rs` : `mod ui;` → `use ui::App;`

### Validation

- ✅ Code compile sans warnings
- ✅ Interface fonctionne identiquement
- ✅ Code mieux organisé, responsabilités claires

---

## Phase 2 - StateManager & Communication (~3-4h)

**Objectif** : Créer une couche d'abstraction pour l'état partagé et la communication.

### Nouvelles structures

#### `src/shared_state.rs`

```rust
pub struct SharedState {
    pub grid: Arc<RwLock<Vec<Vec<bool>>>>,
    pub generation_count: Arc<AtomicUsize>,
}
```

#### `src/state_manager.rs`

```rust
pub enum SimCommand {
    Play,
    Pause,
    StepForward,
    StepBack,
    SetSpeed(f32),
    ToggleCell(usize, usize),
    Shutdown,
}

pub struct SimStats {
    pub generation_count: usize,
    pub is_playing: bool,
}

pub struct StateManager {
    shared_state: SharedState,
    command_tx: mpsc::Sender<SimCommand>,
    stats_rx: mpsc::Receiver<SimStats>,
}
```

### Tâches

- [ ] Créer `src/shared_state.rs`
- [ ] Créer `src/state_manager.rs`
  - Méthode `new() -> Self` (channels + état initial)
  - Méthodes API : `play()`, `pause()`, `step_forward()`, `step_back()`, `set_speed()`, `toggle_cell()`, `read_grid()`, `get_stats()`
- [ ] Créer stub de simulation thread qui :
  - Reçoit les commandes via `command_rx`
  - Envoie des stats dummy via `stats_tx`
- [ ] Adapter `App` pour utiliser `StateManager` au lieu de `game` direct
- [ ] Retirer logique de simulation de `App::update()`

### Validation

- ✅ Communication fonctionne (commandes → stub → stats)
- ✅ UI répond aux commandes
- ✅ Aucune logique de simulation dans UI
- ✅ Prêt pour vrai thread de simulation

---

## Phase 3 - Simulation Thread (~4-5h)

**Objectif** : Déplacer la simulation dans un thread séparé, indépendant du rendu.

### Structure

#### `src/simulation_thread.rs`

```rust
pub fn spawn_simulation_thread(
    shared_state: SharedState,
    command_rx: mpsc::Receiver<SimCommand>,
    stats_tx: mpsc::Sender<SimStats>,
) -> JoinHandle<()>
```

### Tâches

- [ ] Créer `src/simulation_thread.rs`
  - Boucle principale avec timing variable (1-30 gen/s)
  - Écoute des commandes via `command_rx`
  - Calcul de `next_generation()` (déplacer depuis `game.rs`)
  - Gestion de l'historique (100 derniers états)
  - Envoi des stats via `stats_tx`
- [ ] Déplacer `next_generation()` et historique de `ui.rs` vers thread
- [ ] Spawner le thread dans `App::new()`
- [ ] Implémenter `Drop` pour `App` : envoyer `SimCommand::Shutdown` et `join()`
- [ ] Retirer toute logique de simulation de `App::update()`

### Validation

- ✅ Simulation tourne en arrière-plan de manière fluide
- ✅ UI reste à 60 FPS même avec simulation rapide
- ✅ Play/pause/step fonctionnent
- ✅ Pas de race conditions (vérifier avec `cargo test --release`)
- ✅ Thread se termine proprement à la fermeture

---

## Phase 4 - Infinite Grid (~5-6h)

**Objectif** : Remplacer `Vec<Vec<bool>>` par une grille infinie sparse avec `DashMap`.

### Dépendances

```toml
dashmap = "5.5"
```

### Structure

#### `src/infinite_grid.rs`

```rust
pub struct InfiniteGrid {
    cells: DashMap<(i32, i32), bool>,
}
```

### Tâches

- [ ] Ajouter `dashmap = "5.5"` dans `Cargo.toml`
- [ ] Créer `src/infinite_grid.rs`
  - `get(x, y) -> bool`
  - `set(x, y, alive)`
  - `toggle(x, y)`
  - `get_alive_cells() -> Vec<(i32, i32)>` (pour rendu)
  - `count_neighbors(x, y) -> usize`
- [ ] Adapter `GameOfLife` pour utiliser `InfiniteGrid`
- [ ] Adapter `SharedState` pour `Arc<InfiniteGrid>` (pas besoin de RwLock avec DashMap)
- [ ] Mettre à jour `GridView` pour coordonnées `i32` (monde infini)
- [ ] Adapter historique : `HashMap<(i32, i32), bool>` snapshots ou compression delta
- [ ] Ajouter feature flag pour migration progressive (optionnel)

### Validation

- ✅ Pan/zoom infini fonctionne
- ✅ Performance similaire ou meilleure
- ✅ Utilisation mémoire raisonnable (seules cellules vivantes stockées)
- ✅ Pas de race conditions avec accès concurrents

---

## Phase 5 - Optimisations & Polish (~3-4h)

**Objectif** : Peaufiner performance et UX.

### Tâches possibles

- [ ] **Profiling**
  - Installer `cargo install flamegraph`
  - Identifier bottlenecks
  - Optimiser si nécessaire
- [ ] **Optimisations avancées**
  - QuadTree pour culling spatial
  - SIMD pour `count_neighbors()`
  - GPU rendering (wgpu) si nécessaire
- [ ] **UX**
  - Grille de lignes (optionnelle)
  - Mini-map pour navigation
  - Graphe population en temps réel
  - Patterns préchargés (Glider, etc.)
- [ ] **Tests**
  - Tests unitaires pour `InfiniteGrid`
  - Tests d'intégration simulation/UI
- [ ] **Documentation**
  - Mettre à jour README avec nouvelle architecture
  - Documenter API `StateManager`

### Validation

- ✅ Performance optimale
- ✅ UX polie et intuitive
- ✅ Code bien testé et documenté

---

## 🎯 Priorités

**Ordre recommandé** : 1 → 2 → 3 → 4 → 5

Chaque phase peut être un commit/PR séparé. Le code reste fonctionnel entre chaque phase.

**Alternative** : Si grille infinie est priorité absolue, faire 1 → 4 → 2 → 3 → 5

---

## 📝 Notes

- **Historique** : Rester dans thread de simulation (pas de coupling avec UI)
- **Camera** : Rester dans UI (pure préoccupation visuelle)
- **DashMap** : Lock-free, parfait pour grille infinie avec accès concurrent
- **RwLock+Channels** : Bon pour grille fixe actuelle
- **Flexibilité** : Projet d'apprentissage, rien n'est définitif !

---

## 🚀 Prochaine étape

**Phase 1 - UI Separation** (~2-3h)

Prêt à commencer ? 🎮
