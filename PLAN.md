# Plan de projet — Modeleur Merise open source NeoLoop

> Successeur libre et moderne de Looping : MCD / MLD / MPD, interface Slint,
> serveur MCP intégré, optimisé pour tourner sur des machines modestes.

---

## 1. Vision et principes

- **Cible** : étudiants, enseignants et développeurs utilisant la méthode Merise.
- **Différenciateurs** : UI moderne (Slint), protocole MCP natif, format de fichier
  texte git-friendly, auto-layout intelligent, léger (vieux PC bas de gamme).
- **Licence** : GPLv3 (cohérente avec la licence gratuite de Slint).
- **Méthode** : agile, petites itérations livrables, trunk-based development.

## 2. Décisions techniques actées

| Sujet              | Décision                          | Justification |
|--------------------|-----------------------------------|---------------|
| Langage            | Rust (édition 2021+)              | Perf, sûreté mémoire, cross-platform |
| GUI                | Slint (licence GPL)               | Rendu moderne natif, conçu pour l'embarqué |
| Plateformes        | Linux, Windows, macOS             | CI GitHub Actions sur les 3 |
| Format de fichier  | Texte (JSON via serde, ou RON)    | Diffable, versionnable, manipulable par IA |
| MCP                | SDK officiel `rmcp`, transport stdio | Standard, réutilise le moteur core |
| Undo/redo          | Pattern Command dès le début      | Toute mutation passe par une commande |
| Auto-layout        | Force-directed maison (V1) + snap grille | ~150 lignes, gratifiant, adapté Merise |

## 3. Architecture (workspace Cargo)

```
merise-tool/
├── Cargo.toml            # [workspace]
├── LICENSE               # GPLv3
├── CONTRIBUTING.md       # DCO simple (Signed-off-by)
├── crates/
│   ├── core/             # Modèle : Entité, Association, Cardinalité, Héritage, CIF
│   │                     # + moteur de commandes (undo/redo) + validation
│   ├── transform/        # Règles de passage MCD → MLD → MPD
│   ├── sqlgen/           # Génération SQL par dialecte (trait Dialect)
│   ├── layout/           # Auto-layout (force-directed, puis raffinements)
│   ├── mcp/              # Serveur MCP (tools : create_entity, generate_mld, ...)
│   └── gui/              # Slint uniquement — aucune logique métier
└── .github/workflows/ci.yml
```

**Règle d'or** : `gui` et `mcp` dépendent de `core`/`transform`/`sqlgen`/`layout`,
jamais l'inverse. Le moteur doit compiler et se tester sans GUI.

## 4. Roadmap par itérations

Chaque itération : courte (quelques jours max), se termine par un merge sur
`main`, un tag, et quelque chose de démontrable ou de testé.

### Phase A — Fondations (pas de GUI)

- **It. 0 — Bootstrap** : repo GitHub, workspace Cargo, GPLv3, CONTRIBUTING.md,
  CI (fmt + clippy + tests sur Linux/Windows/macOS), README avec la vision.
- **It. 1 — Modèle MCD minimal** : structs `Entity`, `Attribute`, `Association`,
  `Cardinality` dans `core`, avec IDs stables (slotmap). Tests unitaires.
- **It. 2 — Persistance** : sérialisation serde du modèle, versionnage du format
  (`"version": 1`), tests aller-retour (save → load → égalité).
- **It. 3 — Moteur de commandes** : trait `Command { apply, revert }`, pile
  undo/redo, commandes de base (AddEntity, RenameEntity, AddAttribute...).
- **It. 4 — MCD → MLD** : crate `transform`, règles pour binaires
  (1,1)-(0/1,n) → clé étrangère ; (n)-(n) → table associative. Tests exhaustifs :
  **c'est le cœur métier, chaque règle Merise = au moins un test**.
- **It. 5 — SQL** : crate `sqlgen`, trait `Dialect`, premier dialecte
  (PostgreSQL ou SQLite). Snapshot tests (crate `insta`).

### Phase B — GUI Slint

- **It. 6 — Spike jetable (timeboxé 2 jours)** : 3 rectangles draggables reliés
  par des `Path`, pan (Flickable) + zoom (facteur d'échelle). Objectif : valider
  Slint pour le canvas avant d'investir. Le code sera jeté, pas les leçons.
- **It. 7 — Squelette applicatif** : fenêtre, barre d'outils, panneau propriétés,
  zone canvas, thème/direction visuelle (c'est le différenciateur : y passer du temps).
- **It. 8 — Entités à l'écran** : `ModelRc<EntityView>` synchronisé depuis `core`,
  création et drag d'entités (chaque drag = une commande → undo gratuit).
- **It. 9 — Associations** : tracé en `Path`, cardinalités en `Text`, recalcul des
  ancres au déplacement.
- **It. 10 — Cycle complet** : ouvrir/sauver, boutons "Générer MLD" et
  "Exporter SQL". **→ Première release publique 0.1, même imparfaite.**

### Phase C — Confort et intelligence

- **It. 11 — Undo/redo dans la GUI** (Ctrl+Z/Ctrl+Y, branché sur le moteur It. 3).
- **It. 12 — Auto-layout V1** : Fruchterman-Reingold adapté (répulsion entre
  rectangles, associations attirées vers le barycentre de leurs entités),
  snap grille, animation des positions dans Slint, undoable.
- **It. 13 — Serveur MCP** : binaire ou sous-commande `--mcp`, tools
  `create_entity`, `add_association`, `generate_mld`, `export_sql`, `auto_layout`.
- **It. 14 — Vue MLD/MPD** : affichage du modèle logique généré (lecture seule
  d'abord).

### Phase D — Parité Looping (backlog, une itération par item)

- Associations réflexives et n-aires
- Héritage/spécialisation + contraintes (XT, T, X, P)
- CIF (contraintes d'intégrité fonctionnelle)
- Dictionnaire de données
- Vérificateur de cohérence du modèle (rapport d'erreurs/avertissements)
- Dialectes SQL supplémentaires (MySQL/MariaDB, SQL Server, Oracle...)
- Export image (PNG/SVG)
- Zoom sur le layout : routage orthogonal des liens, héritage en arbre
- Impression, thèmes clair/sombre, i18n (FR/EN)

## 5. Remise à niveau Rust — l'essentiel pour CE projet

Ce dont tu auras besoin, dans l'ordre où tu le rencontreras :

1. **Ownership & borrowing** (It. 1) : le modèle en graphe est LE piège classique.
   Solution : pas de `Rc<RefCell<>>` — des arènes avec IDs (`slotmap`), les
   références entre objets sont des `EntityId`, pas des pointeurs.
2. **Enums + pattern matching** (It. 1) : `Cardinality::ZeroN`, etc. Le système
   de types encode les règles Merise → beaucoup d'erreurs impossibles.
3. **Traits** (It. 3-5) : `Command`, `Dialect`. Le polymorphisme à la Rust.
4. **serde** (It. 2) : `#[derive(Serialize, Deserialize)]` fait 95% du travail.
5. **Tests** (partout) : `#[test]` intégré, `cargo test`, + `insta` pour les
   snapshots SQL.
6. **Erreurs** (It. 2+) : `Result` + `thiserror` pour les erreurs métier.
7. **Slint** (Phase B) : callbacks, `ModelRc`, la frontière `.slint` ↔ Rust.

Ressources : *The Rust Book* (ch. 4, 6, 10 en priorité), `rustlings` pour
dérouiller la syntaxe, et la doc Slint (tutoriel "memory game").

Outils : `rustup`, `cargo fmt`, `cargo clippy -- -D warnings` (dans la CI),
`rust-analyzer` (VS Code), extension Slint pour le live-preview des `.slint`.

## 6. Conventions du projet

- **Git** : trunk-based, branches courtes (`feat/...`, `fix/...`), merge rapide,
  tags `v0.x.y` à chaque itération notable. Commits en anglais (projet public).
- **Qualité** : CI bloquante = fmt + clippy + tests sur les 3 OS.
- **Perf** : pas d'allocation dans les chemins chauds du rendu, culling du canvas
  (ne dessiner que le visible), Barnes-Hut pour le layout seulement si besoin
  (>500 entités — backlog).
- **Communauté** : README soigné avec GIF de démo dès la 0.1, issues labellisées
  `good first issue`, DCO plutôt que CLA.

## 7. Définition de "terminé" pour une itération

- Code mergé sur `main`, CI verte sur les 3 OS
- Tests couvrant le nouveau comportement
- CHANGELOG mis à jour (une ligne suffit)
- Démontrable : soit un test qui le prouve, soit une action visible dans l'app
