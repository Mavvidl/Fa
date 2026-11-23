
## Description des modules

### Module `fractal/` – Algorithmes fractaux

| Fichier | Description |
|---------|-------------|
| `mod.rs` | Traits et types communs pour les structures fractales |
| `hilbert.rs` | Implémentation de la courbe de Hilbert (encodage/décodage d'adresses) |
| `sierpinski.rs` | Arbre de répertoires basé sur le triangle/carré de Sierpinski |
| `block_alloc.rs` | Allocation spatiale fractale avec listes auto-similaires |

### Module `vfs/` – Virtual File System

| Fichier | Description |
|---------|-------------|
| `mod.rs` | Interface générique VFS |
| `inode.rs` | Structure d'inode fractal (métadonnées étendues) |
| `directory.rs` | Gestion des répertoires avec nœuds Sierpinski |
| `file_ops.rs` | Opérations de lecture/écriture viabe de Hilbert |

### Module `backend/win/` – Backend Windows

| Fichier | Description |
|---------|-------------|
| `mod.rs` | Adaptateur pour les appels système NT |
| `disk.rs` | Accès direct aux périphériques de stockage |
| `file_handle.rs` | Gestion des handles Windows (CreateFile, ReadFile, etc.) |
| `ioctl.rs` | Contrôle d'entrées-sorties pour volumes et partitions |

### Fichiers racier | Description |
|---------|-------------|
| `main.rs` | Point d'entrée pour le binaire de démonstration |
| `lib.rs` | Bibliothèque principale du projet |
| `config.rs` | Configuration globale (taille de fractale, etc.) |

## Tests

Les tests unitaires sont organisés dans le dossier `tests/` :
- `test_hilbert.rs` : Tests de la courbe de Hilbert
- `test_sierpinski.rs` : Tests de l'arbre de Spendances externes

Le projet nécessite les dépendances suivantes dans `Cargo.toml` :

```toml
[package]
name = "fracfs"
version = "0.1.0"
edition = "2021"
description = "Système de fichiers fractal multiplateforme"

[dependencies]
[target.'cfg(target_os = "windows")'.dependencies]
windows = { version = "0.52", features = [
    "Win32_Storage_FileSystem",
    "Win32_System_IO",
    "Win32_Foundation",
]}

[dev-dependencies]
rand = "0.8"



├── Cargo.toml
├── src/
│   ├── main.rs                  # Point d’entrée (binaire de démonstration)
│   ├── lib.rs                   # Bibliothèque principale
│   ├── fractal/
│   │   ├── mod.rs               # Module fractal : traits et types communs
│   │   ├── hilbert.rs           # Courbe de Hilbert (encodage/décodage)
│   │   ├── sierpinski.rs        # Arbre de répertoires de Sierpinski
│   │   └── block_alloc.rs       # Allocation fractale (listes auto-similaires)
│   ├── vfs/
│   │   ├── mod.rs               # VFS (Virtual File System) – interface générique
│   │   ├── inode.rs             # Inode fractal (métadonnées)
│   │   ├── directory.rs         # Répertoire fractal (nœud Sierpinski)
│   │   └── file_ops.rs          # Opérations de lecture/écriture
│   ├── backend/
│   │   ├── mod.rs
│   │   └── win/
│   │       ├── mod.rs           # Backend Windows : adaptateur pour syscalls NT
│   │       ├── disk.rs          # Gestion des périphériques (ouvrir, lire secteur)
│   │       ├── file_handle.rs   # Gestion des handles Windows (CreateFile, ReadFile, etc.)
│   │       └── ioctl.rs         # Contrôle d’entrées-sorties (volumes, partitions)
│   └── config.rs                # Configuration (taille de bloc, profondeur fractale…)
├── tests/
│   ├── test_hilbert.rs
│   └── test_sierpinski.rs
└── .gitignore                   # déjà fourni
