L'objectif ne sera pas de créer un clone de ext4, NTFS ou Btrfs avec quelques algorithmes fractals supplémentaires.

Nous allons identifier :

- Ce que les systèmes existants font exceptionnellement bien.
- Les problèmes qu'ils rencontrent.
- Les fonctionnalités qu'ils ne proposent pas suffisamment.
- Les techniques que nous pouvons améliorer.
- Les innovations que FaFs pourrait apporter.

Je propose de faire de cette analyse le fondement de l'architecture officielle de FaFs.

# 1\. Analyse stratégique des systèmes existants

## 1\. Analyse des forces et faiblesses

### A. ext4 — simplicité, stabilité et allocation efficace

📁 Hard Disks and File Systems

ext4

Notre référence pour l'allocation classique et la robustesse.

Ses forces :

- Allocation par groupes de blocs.
- Extents pour réduire les métadonnées d'allocation.
- Journalisation des métadonnées.
- Bonne gestion des fichiers ordinaires.
- Architecture éprouvée.

Ses limites :

- La journalisation ajoute des écritures et du travail de gestion.
- L'organisation en groupes ne garantit pas une absence de fragmentation.
- Les fonctionnalités natives de snapshots et de clonage sont moins avancées que celles de Btrfs et APFS.

Les groupes de blocs et les extents sont documentés dans l'architecture officielle du noyau Linux.

The Linux Kernel documentation

+1

Ce que FaFs doit reprendre :

| Technique | Décision pour FaFs |
| --- | --- |
| Block Groups | Oui, sous forme de régions fractales |
| Extents | Oui, absolument |
| Journalisation | Oui, avec transactions |
| Allocation locale | Oui, avec placement Hilbert |
| Inodes | Oui, avec identifiants stables |

Notre amélioration : rendre les régions d'allocation adaptatives, avec des statistiques sur leur occupation, leur fragmentation et leur activité.

### B. NTFS — métadonnées et indexation

How to extract data and timeline from Master File Table on NTFS filesystem | Andrea Fortuna

NTFS

Notre référence pour la gestion des fichiers et des métadonnées.

Ses forces :

- Master File Table (MFT).
- Indexation des répertoires.
- Attributs extensibles.
- Stockage résident de certains petits fichiers.
- Journalisation et mécanismes de récupération.
- Gestion avancée des permissions.

Ses limites :

- La MFT et les structures associées nécessitent une gestion et une maintenance importantes.
- Les mécanismes internes sont complexes.
- Certaines fonctionnalités sont spécifiques à l'écosystème Windows.

Microsoft décrit notamment la MFT, les transactions et les objectifs de fiabilité de NTFS.

Microsoft Learn

+1

Ce que FaFs doit reprendre :

| Technique | Décision pour FaFs |
| --- | --- |
| MFT | Table centrale d'inodes |
| B+ Tree | Index des répertoires |
| Attributs | Métadonnées extensibles |
| Resident Data | Stockage inline des petits fichiers |
| Journal | Transactions récupérables |

Notre amélioration : concevoir un moteur de recherche multi-index qui évite de parcourir toute la hiérarchie pour retrouver un fichier.

Un point particulièrement intéressant : les petits fichiers pourraient être stockés directement dans leur inode lorsqu'ils tiennent dans l'espace réservé, évitant une allocation de bloc supplémentaire.

Il faudra toutefois mesurer le seuil optimal pour ne pas agrandir excessivement les métadonnées.

### C. Btrfs — intégrité, snapshots et Copy-on-Write

Btrfs, snapshots and rollback - Speaker Deck

Btrfs

Notre référence pour la fiabilité et la gestion des versions.

Ses forces :

- Copy-on-Write.
- Snapshots.
- Checksums des données et métadonnées.
- Arbres B-tree.
- Partage de blocs entre snapshots.
- Détection de corruptions.

Ses limites :

- Le Copy-on-Write peut entraîner de la fragmentation.
- Les écritures peuvent nécessiter des mises à jour de plusieurs structures.
- La gestion des espaces libres et des snapshots devient complexe.
- Certaines charges de travail avec de nombreuses petites écritures peuvent être pénalisées.

La documentation Btrfs explique son fonctionnement par arbres, références d'extents et Copy-on-Write. La documentation décrit aussi les coûts possibles de fragmentation.

BTRFS documentation

+2

Ce que FaFs doit reprendre :

| Technique | Décision pour FaFs |
| --- | --- |
| Copy-on-Write | Oui, pour les métadonnées |
| Snapshots | Oui |
| Checksums | Oui |
| Référencement des blocs | Oui |
| Transaction par génération | Oui |

Notre amélioration : développer une politique d'allocation qui tient compte de la fragmentation introduite par le Copy-on-Write.

Je recommande de commencer par le Copy-on-Write des métadonnées, puis d'étudier son extension aux données. Cela évite de complexifier immédiatement tout le moteur de stockage.

### D. XFS — performances et parallélisme

Best Methods for Data Recovery from XFS File System: A Comprehensive Approach\!

XFS

Notre référence pour les gros volumes et les accès concurrents.

Ses forces :

- Allocation Groups.
- B+ Trees.
- Gestion de très gros fichiers.
- Allocation différée.
- Préallocation d'espace.
- Opérations parallèles.

Ses limites :

- Certaines charges de travail très riches en petites opérations de métadonnées peuvent être moins performantes.
- La gestion parallèle augmente la complexité.
- Il faut bien dimensionner et synchroniser les structures.

La documentation Red Hat souligne notamment les capacités de XFS en matière de parallélisme, tout en signalant ses performances plus faibles sur certaines charges mono-thread riches en métadonnées.

Red Hat Documentation

+1

Ce que FaFs doit reprendre :

- Des régions de stockage indépendantes.
- Des allocations parallèles.
- Des index B+ Tree.
- Des écritures différées.
- Une gestion concurrente des blocs.

Notre amélioration : utiliser les régions fractales comme unités de travail indépendantes, avec des verrous locaux plutôt qu'un verrou global sur tout le système.

### E. APFS — gestion intelligente de l'espace

Comment convertir le système de fichiers du disque de stockage du Mac de HFS à APFS - Tutoriel de réparation iFixit

APFS

Notre référence pour l'optimisation de l'espace et les clones.

Ses forces :

- Copy-on-Write.
- Clonage de fichiers.
- Snapshots.
- Partage dynamique de l'espace entre volumes.
- Optimisations pour les SSD.
- Chiffrement.

Ses limites ou contraintes :

- Conçu pour l'écosystème Apple.
- Architecture propriétaire et documentation interne plus difficile à exploiter.
- Certaines stratégies sont particulièrement orientées stockage flash.

Apple documente le partage d'espace, les clones, les snapshots et ses mécanismes Copy-on-Write.

Apple Support

+1

Ce que FaFs doit reprendre :

- Les clones de fichiers sans copie immédiate des données.
- Les snapshots.
- La gestion dynamique des espaces.
- Une allocation adaptée au type de périphérique.

Notre amélioration : créer un gestionnaire d'espace unifié, capable de distribuer dynamiquement les blocs disponibles entre plusieurs régions ou volumes.

# 2\. Le bilan : ce que FaFs doit corriger

Voici les problèmes que je veux placer au centre du développement.

| Problème observé | Réponse architecturale de FaFs |
| --- | --- |
| Fragmentation | Allocation fractale + extents |
| Recherche coûteuse | Index B+ + cache Hash |
| Petits fichiers gourmands en espace | Stockage inline |
| Copies coûteuses | Reflinks et Copy-on-Write |
| Corruption silencieuse | Checksums |
| Récupération après panne | Journal et transactions |
| Mauvais passage à l'échelle | Allocation régionale parallèle |
| Mauvaise gestion de l'espace | Régions adaptatives |
| Accès répétitifs | Cache intelligent |
| Différences HDD/SSD/NVMe | Politiques d'allocation adaptées |

Mais il faut être clair : ce sont des axes d'amélioration à tester, pas des défauts que tous les systèmes concurrents présentent systématiquement.

# 3\. La nouvelle architecture officielle de FaFs

Je propose maintenant de réunir les meilleures techniques dans une architecture unique.

Voici ce que je retiendrais.

# FaFs — Adaptive Fractal Storage Architecture

Architecture proposéeVFS — Virtual File System

File · Directory · Inode · Permissions · Metadata

Fractal Engine

Hilbert · Sierpinski · Spatial Mapping

Index Engine

B+ Tree · Hash · Metadata Index

Allocation Engine

Extents · Inline Data · Free Space

Cache Engine

Block · Inode · Directory · Read Ahead

Reliability Engine

Copy-on-Write · Journal · Checksums · Snapshots

Storage Abstraction Layer

Block Device · Disk Image · HDD · SSD · NVMe

Windows

Linux

macOS

## 4\. Nouvelle arborescence Rust

Je modifierais également ton arborescence pour que chaque responsabilité soit isolée.

text

```text
fafs/
│
├── .github/
│   └── workflows/
│       ├── ci.yml
│       ├── tests.yml
│       ├── security.yml
│       └── benchmarks.yml
│
├── docs/
│   ├── architecture/
│   ├── design-decisions/
│   ├── filesystem-format/
│   └── images/
│
├── benches/
│   ├── allocation.rs
│   ├── fragmentation.rs
│   ├── file_search.rs
│   ├── sequential_read.rs
│   ├── random_read.rs
│   ├── metadata_operations.rs
│   └── concurrent_io.rs
│
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── config.rs
│   ├── error.rs
│   │
│   ├── fractal/
│   │   ├── mod.rs
│   │   ├── hilbert.rs
│   │   ├── sierpinski.rs
│   │   ├── spatial_mapping.rs
│   │   └── region.rs
│   │
│   ├── allocator/
│   │   ├── mod.rs
│   │   ├── block_alloc.rs
│   │   ├── extent_alloc.rs
│   │   ├── inline_alloc.rs
│   │   ├── free_space.rs
│   │   └── fragmentation.rs
│   │
│   ├── index/
│   │   ├── mod.rs
│   │   ├── btree.rs
│   │   ├── name_index.rs
│   │   ├── inode_index.rs
│   │   ├── path_index.rs
│   │   └── metadata_index.rs
│   │
│   ├── cache/
│   │   ├── mod.rs
│   │   ├── block_cache.rs
│   │   ├── inode_cache.rs
│   │   ├── directory_cache.rs
│   │   └── read_ahead.rs
│   │
│   ├── vfs/
│   │   ├── mod.rs
│   │   ├── inode.rs
│   │   ├── directory.rs
│   │   ├── file_ops.rs
│   │   └── filesystem.rs
│   │
│   ├── metadata/
│   │   ├── mod.rs
│   │   ├── superblock.rs
│   │   ├── journal.rs
│   │   ├── transaction.rs
│   │   ├── checksum.rs
│   │   └── serialization.rs
│   │
│   ├── snapshot/
│   │   ├── mod.rs
│   │   ├── cow.rs
│   │   ├── reflink.rs
│   │   └── snapshot.rs
│   │
│   ├── storage/
│   │   ├── mod.rs
│   │   ├── block_device.rs
│   │   ├── disk_image.rs
│   │   └── memory_device.rs
│   │
│   └── backend/
│       ├── mod.rs
│       ├── windows/
│       ├── linux/
│       └── macos/
│
├── tests/
│   ├── allocation/
│   ├── indexing/
│   ├── filesystem/
│   ├── persistence/
│   ├── recovery/
│   ├── integrity/
│   └── cross_platform/
│
├── Cargo.toml
├── Cargo.lock
└── README.md
```

Une règle importante : le moteur VFS ne doit jamais dépendre directement de Windows, Linux ou macOS. Il doit communiquer avec une interface `BlockDevice`. Les backends se chargent de l'adaptation aux systèmes d'exploitation.

Pour commencer, nous utiliserons un périphérique mémoire et une image disque virtuelle. C'est beaucoup plus sûr que de développer directement sur un disque physique.

# 5\. Les innovations que je veux réellement développer

Il ne suffit pas de réunir des technologies existantes. FaFs doit avoir ses propres contributions.

Je retiens quatre innovations à explorer.

Hilbert Curve Infographic Generator — Mathematical Diagram | sora2hub

Innovation 1 — Fractal Spatial Allocator

Combiner les régions d'allocation, les extents et le placement Hilbert.

Objectif : limiter les déplacements inutiles et la fragmentation.

The Nature of Fractals » Quaint Symmetries

Innovation 2 — Adaptive Sierpinski Regions

Ajuster les régions selon leur occupation et leurs besoins.

Objectif : éviter les régions trop grandes, trop petites ou mal équilibrées.

SQL Server: Boas práticas com chaves - DevMedia

Innovation 3 — Predictive File Retrieval

Analyser les accès précédents et précharger les métadonnées ou données pertinentes.

Objectif : accélérer les accès répétitifs sans effectuer de prélecture inutile.

Week 4 Student Notes

Innovation 4 — Adaptive Storage Policy

Adapter les stratégies d'allocation et de cache selon les caractéristiques du support et les charges de travail.

Objectif : éviter d'utiliser les mêmes heuristiques sur HDD et NVMe.

Ces quatre innovations sont des hypothèses de conception. Nous devrons les comparer à des allocateurs classiques, des arbres B+ et des caches standards pour déterminer lesquelles méritent réellement d'être conservées.

# 6\. Plan de développement prioritaire

Je ne veux surtout pas que nous commencions par les snapshots, les caches intelligents et les algorithmes complexes en même temps.

Voici l'ordre que je recommande.

| Phase | Développement | Critère de validation |
| --- | --- | --- |
| 1 | Storage Engine | Lecture/écriture fiables |
| 2 | Inodes et répertoires | Cycle de vie des fichiers |
| 3 | Extents et allocation | Pas de double allocation |
| 4 | Fractal Engine | Mapping correct et mesurable |
| 5 | Index Engine | Recherche persistante |
| 6 | Cache Engine | Gains mesurés |
| 7 | Transactions et journal | Récupération après panne |
| 8 | Copy-on-Write | Cohérence des mises à jour |
| 9 | Snapshots et reflinks | Partage sûr des blocs |
| 10 | Backends multiplateformes | Tests sur chaque OS |

## 7\. Les règles de qualité que je veux imposer

Pour éviter de créer un système impressionnant sur le papier mais fragile dans la réalité :

- Aucun bloc ne peut être alloué deux fois.
- Aucun bloc référencé ne peut être libéré prématurément.
- Aucun index ne doit pointer vers un inode inexistant.
- Une interruption ne doit pas laisser un état incohérent après récupération.
- Un fichier doit retrouver exactement son contenu après démontage et remontage.
- Les checksums doivent détecter les corruptions testées.
- Les optimisations ne doivent pas compromettre la cohérence des données.

Nous ajouterons également des tests de panne simulée, des tests de concurrence, des fuzz tests sur le format disque et des benchmarks reproductibles.

# 8\. Comment FaFs devra se comparer aux concurrents

Je veux que nous ayons une vraie politique de validation :

Benchmark comparatif FaFs

Chaque résultat sera mesuré, avec configuration matérielle, version, cache froid/chaud et charge de travail documentés.

| Charge de travail | Références |
| --- | --- |
| Millions de petits fichiers | ext4, NTFS, XFS |
| Gros fichiers séquentiels | ext4, XFS, NTFS |
| Lectures aléatoires | ext4, Btrfs, XFS |
| Nombreuses modifications | ext4, Btrfs |
| Snapshots et clones | Btrfs, APFS |
| Recherche de fichiers | NTFS, ext4, XFS |
| Occupation de l'espace | Tous |

Mesures obligatoires

- Latence P50, P95 et P99.
- Débit MB/s.
- IOPS.
- Espace réellement utilisé.
- Fragmentation.
- Coût CPU et mémoire.
- Temps de récupération après panne.

Une précaution : les comparaisons doivent être faites sur des plateformes communes et avec des configurations équivalentes. Nous ne pourrons pas comparer directement toutes les fonctionnalités de NTFS, APFS et ext4 sur une seule machine.

# 9\. Les décisions techniques que je fixerais maintenant

Voici ma proposition de base pour FaFs :

| Élément | Décision |
| --- | --- |
| Langage | Rust |
| Bloc logique initial | 4 KiB |
| Allocation | Hybride inline + extents |
| Index persistant | B+ Tree |
| Index mémoire | HashMap |
| Organisation spatiale | Hilbert |
| Hiérarchie des régions | Sierpinski |
| Métadonnées | Inodes |
| Transactions | Journalisation |
| Protection | Checksums |
| Snapshots | Copy-on-Write |
| Cache | 2Q à évaluer contre LRU |
| Stockage de développement | Disk Image |
| Plateformes cibles | Windows, Linux, macOS |

Le format disque devra être versionné dès le début, avec des identifiants de fonctionnalités et des mécanismes de migration.

# 10\. Ma vision finale pour FaFs

Je veux que nous poursuivions trois objectifs dans cet ordre :

1. Être aussi fiable que les systèmes de fichiers établis.
2. Être compétitif en performances et en occupation de l'espace.
3. Apporter une véritable innovation avec l'organisation fractale adaptative.

Il serait prématuré de promettre que FaFs surpassera ext4, NTFS ou Btrfs. Mais nous pouvons concevoir un système sérieux, dont chaque décision est justifiée par des mesures et des tests.

Je ferais de cette stratégie le document de référence du projet : `docs/architecture/DESIGN.md`.

## Les documentations de référence

- [ext4 — Architecture du noyau Linux](<https://docs.kernel.org/6.15/filesystems/ext4/overview.html?utm_source=chatgpt.com>).
- [NTFS — Documentation Microsoft](<https://learn.microsoft.com/en-us/troubleshoot/windows-client/backup-and-storage/fat-hpfs-and-ntfs-file-systems?utm_source=chatgpt.com>).
- [Btrfs — Architecture interne](<https://btrfs.readthedocs.io/en/latest/dev/dev-btrfs-design.html?utm_source=chatgpt.com>).
- [XFS — Guide technique Red Hat](<https://docs.redhat.com/en/documentation/red_hat_enterprise_linux/8/html/system_design_guide/getting-started-with-xfs_system-design-guide?utm_source=chatgpt.com>).
- [APFS — Documentation Apple](<https://developer.apple.com/documentation/foundation/about-apple-file-system?changes=l_2&language=objc&utm_source=chatgpt.com>).

Ma recommandation : la prochaine étape doit être de formaliser le format disque de FaFs et son moteur d'allocation. C'est le socle sur lequel toutes les autres innovations vont reposer. Si ce socle est bien conçu, nous pourrons ajouter progressivement les index, les snapshots, le cache et les optimisations fractales sans devoir reconstruire tout le projet.
