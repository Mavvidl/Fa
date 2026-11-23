# FaFS – Système de fichiers basé sur la géométrie fractal en Rust

[![Licence: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.65%2B-blueviolet)](https://www.rust-lang.org)

**FracFS** est un système de fichiers expérimental, multiplateforme, écrit en Rust, dont l’organisation des données et des métadonnées repose sur des **algorithmes de géométrie fractale**.

## Objectifs

- **Localité spatiale** : grâce à une courbe de remplissage d’espace (Hilbert), les blocs logiquement proches sont physiquement contigus.
- **Scalabilité** : l’arbre fractal de métadonnées permet une indexation en O(log n) avec une auto‑similitude réduisant la fragmentation.
- **Résilience** : la structure fractale facilite la reconstruction rapide après une panne.
- **Multiplateforme** : développé en Rust (std::fs, raw syscalls uniquement en backend).

## Fonctionnalités clés (en développement)

- Courbe de Hilbert pour l’allocation des blocs.
- Arbre de Sierpinski pour les répertoires (navigation par quadrants).
- Algorithme de compactage fractal (auto‑défragmentation).
- Support des systèmes de blocs logiques de taille variable (fractal adaptive block sizes).

## Démarrage rapide

```bash
git clone ...
cd fafs
cargo build --release
# exemples d'utilisation à venir

