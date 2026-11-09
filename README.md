# emergent-agents

## Vision

Rémi veut construire un système d'agents dont le comportement émerge de cerveaux composés d'unités élémentaires.

L'objectif n'est pas d'imposer une architecture cognitive haut niveau prédéfinie. Le comportement doit émerger de la composition des unités, de leurs valeurs, de leur organisation, de leur parcours et de leur évolution.

Le projet commence simplement, notamment avec trois agents de démonstration, mais l'architecture ne doit pas introduire de limitation artificielle qui empêcherait des comportements plus riches plus tard.

## Modèle métier

### Agent

Un `Agent` représente un individu.

Il possède au minimum :

- un `AgentId` ;
- un nom ;
- un rôle ;
- un `Brain`.

Il peut également porter des informations utiles à l'organisation des expériences, notamment :

- des groupes ;
- un cycle d'entrée.

Le type exact utilisé pour les groupes et le cycle d'entrée n'est pas figé tant qu'un besoin concret ne l'impose pas.

### Brain

Un `Brain` représente le cerveau d'un agent.

Il possède :

- un `BrainId` ;
- un ou plusieurs `BrainGenome`.

Le `Brain` est responsable de l'orchestration de ses genomes : c'est à ce niveau qu'est déterminé quels genomes sont utilisés et comment ils sont combinés ou enchaînés.

La manière exacte de réaliser cette orchestration n'est pas figée dans l'architecture.

Le `Brain` ne doit pas devenir un conteneur d'état cognitif parallèle. Les éléments cognitifs, la mémoire et les autres états du cerveau ont vocation à être représentés dans les genomes, notamment par des `Unit`.

### BrainGenome

Un `BrainGenome` représente une structure cognitive appartenant à un cerveau.

Il possède :

- un `BrainGenomeId` ;
- un `Vec<Unit>`.

Le genome porte l'état réel correspondant à cette structure cognitive.

Il n'est pas limité à une configuration initiale immuable : ses valeurs et sa structure peuvent évoluer.

L'historique, les snapshots ou les anciennes versions relèvent de la persistance et ne doivent pas déformer le modèle métier courant.

### Identifiants

Chaque niveau important possède sa propre identité stable en UUIDv7 :

- `AgentId` ;
- `BrainId` ;
- `BrainGenomeId` ;
- `UnitId`.

Un UUID représente une identité.

Un `usize` représente uniquement une position dans une collection et ne doit jamais être confondu avec une identité.

## Unit

`Unit` est le point d'entrée commun de toutes les unités qui composent un genome.

Les catégories de Units ne sont pas figées au départ. Elles ne devront être introduites que si des groupes réellement stables et utiles apparaissent.

Une Unit utilise la représentation la plus adaptée à sa sémantique : `struct`, `enum` ou autre type dédié si nécessaire.

Un type utilisé à l'intérieur d'une Unit n'est pas automatiquement lui-même une Unit.

Chaque Unit porte sa propre sémantique et sa propre mécanique interne. Le moteur d'exécution ne doit pas reconstruire cette logique à l'extérieur.

Les Units déjà envisagées incluent notamment :

- `Start` ;
- `SplitWithSpace` ;
- `ConvertIntoBytes` ;
- des valeurs comme `F32` ;
- `Output` ;
- `Stop` ;
- plus tard, d'autres transformations, valeurs, fonctions, mémoires ou mécanismes de mutation.

## Start et Traversal

`Start` est une Unit particulière qui définit le point de départ d'un parcours et porte son `Traversal`.

Un genome possède normalement un seul `Start` initial.

Si plusieurs `Start` existent malgré tout, celui dont la position `usize` est la première dans le genome sert de point de départ initial.

Si un autre `Start` est rencontré pendant le parcours, son `Traversal` peut prendre le relais pour la suite.

### Traversal

`Traversal` remplace l'ancien concept trop étroit d'`OrderBy`.

Il définit la manière dont les Units sont parcourues à partir d'un `Start`.

Il peut notamment exprimer :

- une progression vers l'avant ou l'arrière ;
- des sauts de positions ;
- une sélection d'indices ;
- une sélection d'UUID ;
- des exclusions ;
- des filtres ;
- une sélection partielle d'un ensemble de candidats ;
- des règles de parcours plus riches.

Par exemple, un parcours équivalent à `x + 2` peut sauter une Unit entre deux exécutions.

Un Traversal peut aussi partir d'un ensemble de positions candidates et n'en retenir qu'une partie.

L'implémentation initiale peut rester simple, mais le modèle ne doit pas être limité à un simple `next_index() -> usize` ni imposer un parcours séquentiel de toutes les Units.

La forme exacte de `Traversal` n'est pas figée tant que les premiers besoins concrets ne l'imposent pas.

## Exécution

Le projet utilise une VM générique comme mécanisme d'exécution.

La VM n'est pas spécialisée par domaine : l'architecture ne doit pas conduire à multiplier des types comme `BrainVm`, `PhysicsVm`, `MutationVm`, etc.

La VM exécute ce qu'on lui fournit.

Elle orchestre l'exécution technique, applique les limites nécessaires et délègue la sémantique aux Units concernées.

Elle ne porte pas l'intelligence du cerveau et ne remplace pas `Brain`, `BrainGenome` ou `Unit`.

## Valeurs et mémoire

Les Units peuvent produire, transformer et consommer des valeurs typées.

La mémoire et les autres états cognitifs ont vocation à être modélisés dans les genomes à l'aide de Units plutôt qu'à être ajoutés comme un état global parallèle du `Brain`.

La forme exacte de ces Units n'est pas figée avant qu'un besoin réel ne l'exige.

## Mutation et évolution

Les mutations et évolutions du cerveau doivent être décrites par le système cognitif lui-même.

Si un genome ne contient aucun mécanisme de mutation, aucune mutation ne doit être inventée extérieurement.

Les mécanismes précis de mutation ne sont pas encore figés.

## Persistance

Deux représentations sont prévues :

- `.ron` comme format lisible ;
- `.bin` comme format runtime rapide.

Le format binaire utilise `bincode-next`.

Le runtime normal a vocation à utiliser le `.bin`.

La politique exacte de matérialisation, de régénération et de versionnement entre `.ron` et `.bin` sera définie lorsqu'elle deviendra nécessaire.

## Première expérience

La première expérience met en scène des agents avec les rôles :

- `Transmitter` ;
- `Receiver` ;
- `Interceptor`.

Alice, Bob et Eve sont des noms de démonstration.

Chaque participant est un véritable `Agent` possédant son propre `Brain`.

Le comportement observé doit provenir de l'exécution de leur système cognitif.

## Principe de conception

Le projet suit une règle simple :

> Ne rien figer ni inventer sans besoin concret.

L'architecture doit rester explicite et extensible.

Lorsqu'un concept n'est pas encore défini, il reste ouvert jusqu'à ce qu'un besoin réel permette de le définir correctement.
