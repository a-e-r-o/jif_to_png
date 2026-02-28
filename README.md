# jif_to_jpg / jif_to_png

> Le code et les assets de ce projet, ainsi que ce README ont été générés par Copilot (Claude Opus 4.6).

## Description

Deux utilitaires en Rust qui convertissent les fichiers `.jif` et `.webp` du répertoire courant.

- Les fichiers JIF sont des fichiers JPEG (format JFIF) avec une extension différente.
- Les fichiers WEBP sont décodés nativement.

Les programmes décodent ces fichiers et les ré-enregistrent dans le format choisi. Après une conversion réussie, le fichier source est supprimé.

- **jif_to_jpg** : convertit en JPEG qualité 95%.
- **jif_to_png** : convertit en PNG (lossless).

## Comportement

- Le programme scanne le répertoire depuis lequel il est lancé.
- Il cherche tous les fichiers `.jif` et `.webp` (insensible à la casse).
- Chaque fichier est converti en `.jpg` ou `.png` selon le programme utilisé.
- Si la conversion réussit, le fichier source est supprimé.
- Sur Windows, aucune fenêtre console ne s'ouvre : le programme s'exécute et se ferme silencieusement.

En pratique, il suffit de déposer l'exécutable dans un dossier contenant des fichiers `.jif` ou `.webp` et de le lancer.

## Prérequis

- Rust (via [rustup](https://rustup.rs/))

## Compilation pour Linux

Compilation standard :

```bash
cargo build --release
```

Les binaires se trouvent dans `target/release/` (`jif_to_jpg` et `jif_to_png`).

Note : le fichier `.cargo/config.toml` présent dans le projet force la target Windows (`x86_64-pc-windows-gnu`). Pour compiler en natif Linux, il faut soit le supprimer/renommer, soit spécifier la target explicitement :

```bash
cargo build --release --target x86_64-unknown-linux-gnu
```

## Compilation pour Windows

### Depuis Windows directement

Il faut avoir Rust installé sur Windows via rustup.

```powershell
cargo build --release
```

Le `.cargo/config.toml` n'a pas d'impact particulier si vous compilez depuis Windows avec la toolchain MSVC par défaut. Si vous utilisez la toolchain GNU, ça fonctionnera aussi.

Les binaires seront dans `target/release/` ou `target/x86_64-pc-windows-gnu/release/` selon la target.

### Depuis WSL (cross-compilation)

C'est la méthode utilisée dans ce projet. Elle permet de n'avoir qu'une seule installation Rust (sur WSL) et de produire un `.exe` Windows.

#### Setup (une seule fois)

1. Installer la target Windows :
   ```bash
   rustup target add x86_64-pc-windows-gnu
   ```

2. Installer le linker MinGW :
   ```bash
   sudo apt install gcc-mingw-w64-x86-64
   ```

Le fichier `.cargo/config.toml` est déjà configuré pour utiliser cette target par défaut et pointer vers le bon linker.

#### Compilation

```bash
cd /mnt/e/Dev/jif_to_png   # ou le chemin vers le projet
cargo build --release
```

Les binaires se trouvent dans `target/x86_64-pc-windows-gnu/release/` (`jif_to_jpg.exe` et `jif_to_png.exe`).

## Structure du projet

```
Cargo.toml              - Dépendances et métadonnées
build.rs                - Script de build (intègre les icônes sur Windows)
src/lib.rs              - Code partagé (scan, conversion)
src/bin/jif_to_jpg.rs   - Point d'entrée JPG
src/bin/jif_to_png.rs   - Point d'entrée PNG
assets/icon_jpg.ico     - Icône de jif_to_jpg.exe
assets/icon_png.ico     - Icône de jif_to_png.exe
.cargo/config.toml      - Configuration de cross-compilation WSL -> Windows
```

## Notes

- L'attribut `#![windows_subsystem = "windows"]` dans `lib.rs` empêche l'ouverture d'une console. Si vous avez besoin de debug, vous pouvez le retirer temporairement ou le remplacer par `"console"`.
- Les icônes sont intégrées aux exe via `windres` (MinGW). Le `build.rs` ne les applique que quand la target est Windows, donc la compilation Linux fonctionne sans problème.
- La crate `image` est importée avec uniquement les features `jpeg`, `png` et `webp` pour limiter la taille des binaires. Le profil release active LTO, strip des symboles, et `panic = "abort"` pour réduire encore la taille (~700 Ko par exe).
