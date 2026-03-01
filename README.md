# QuickImgConvert

> Le code et les assets de ce projet, ainsi que ce README ont été générés par Copilot (Claude Opus 4.6).

## Description

Utilitaire graphique en Rust qui convertit les fichiers `.jif`, `.webp` et `.avif` du répertoire courant en PNG ou JPG.

- Les fichiers JIF sont des fichiers JPEG (format JFIF) avec une extension différente.
- Les fichiers WEBP sont décodés nativement.
- Les fichiers AVIF (AV1 Image File Format) sont décodés nativement.

Le programme propose une interface graphique minimale (via [egui](https://github.com/emilk/egui)/[eframe](https://github.com/emilk/egui/tree/master/crates/eframe)) permettant de choisir le format de sortie :

- **PNG** : conversion lossless
- **JPG** : compression qualité 95%

Après une conversion réussie, le fichier source est supprimé.

## Interface graphique

L'interface utilise egui avec le backend OpenGL (`glow`), ce qui la rend native et cross-platform (Windows et Linux). Au lancement, une fenêtre s'ouvre avec :

- Un **menu déroulant** pour choisir le format de sortie (PNG ou JPG)
- Un **bouton « Convertir »** qui lance la conversion de tous les fichiers compatibles du répertoire courant
- Un **message de statut** indiquant le résultat

Sur Windows, aucune fenêtre console ne s'ouvre. L'icône personnalisée est affichée dans la barre de titre, la barre des tâches, et dans l'explorateur de fichiers.

## Comportement

- Au lancement, une fenêtre s'ouvre avec un menu déroulant pour choisir le format de sortie.
- Cliquer sur « Convertir » scanne le répertoire courant.
- Tous les fichiers `.jif`, `.webp` et `.avif` (insensible à la casse) sont convertis.
- Si la conversion réussit, le fichier source est supprimé.
- Le résultat de la conversion est affiché dans la fenêtre.

En pratique, il suffit de déposer l'exécutable dans un dossier contenant des fichiers `.jif`, `.webp` ou `.avif` et de le lancer.

## Formats supportés

| Format d'entrée | Extension | Description |
|---|---|---|
| JPEG/JFIF | `.jif` | Fichiers JPEG avec extension JFIF |
| WebP | `.webp` | Format Google WebP |
| AVIF | `.avif` | Format AV1 Image File |

## Prérequis

- Rust (via [rustup](https://rustup.rs/))

### Linux uniquement

Pour compiler sur Linux, les bibliothèques de développement suivantes sont nécessaires (pour le rendu graphique egui/eframe) :

```bash
# Ubuntu/Debian
sudo apt install libxkbcommon-dev libwayland-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libgl-dev
```

## Compilation pour Linux

```bash
cargo build --release --target x86_64-unknown-linux-gnu
```

Le binaire se trouve dans `target/x86_64-unknown-linux-gnu/release/quick_img_convert`.

Note : le fichier `.cargo/config.toml` force la target Windows (`x86_64-pc-windows-gnu`). Pour compiler en natif Linux, spécifier la target explicitement comme ci-dessus.

## Compilation pour Windows

### Depuis Windows directement

Il faut avoir Rust installé sur Windows via rustup.

```powershell
cargo build --release
```

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

Le binaire se trouve dans `target/x86_64-pc-windows-gnu/release/quick_img_convert.exe`.

### Compression avec UPX (optionnel)

Le binaire release pèse ~3.7 Mo (dû à l'interface graphique egui et aux décodeurs d'images). On peut le compresser avec [UPX](https://upx.github.io/) pour réduire sa taille d'environ 50% :

```bash
# Installation (Ubuntu/Debian)
sudo apt install upx-ucl

# Compression
upx --best --lzma target/x86_64-pc-windows-gnu/release/quick_img_convert.exe
```

Résultat typique : **3.7 Mo → ~1.7 Mo**.

Impact : léger délai supplémentaire au premier lancement (~30-50ms pour la décompression en RAM), négligeable en pratique. Attention : certains antivirus peuvent signaler les exécutables compressés par UPX comme suspects (faux positif).

## Structure du projet

```
Cargo.toml                      - Dépendances et métadonnées
build.rs                        - Script de build (intègre l'icône sur Windows)
src/lib.rs                      - Code partagé (scan, conversion)
src/bin/quick_img_convert.rs    - Interface graphique (egui/eframe)
assets/Untitled.png             - Icône source (PNG)
assets/icon.ico                 - Icône Windows (générée depuis le PNG)
assets/convert_icon.py          - Script de conversion PNG → ICO
.cargo/config.toml              - Configuration de cross-compilation WSL -> Windows
```

## Notes

- L'attribut `#![windows_subsystem = "windows"]` dans le binaire empêche l'ouverture d'une console sur Windows. Pour debug, le retirer temporairement ou le remplacer par `"console"`.
- L'icône est intégrée à l'exe via `windres` (MinGW). Le `build.rs` ne l'applique que quand la target est Windows. L'icône est aussi chargée au runtime via `include_bytes!` pour s'afficher dans la barre de titre et la barre des tâches.
- L'interface graphique utilise [egui](https://github.com/emilk/egui) via [eframe](https://github.com/emilk/egui/tree/master/crates/eframe), une bibliothèque GUI immediate-mode en Rust. Le backend de rendu utilisé est `glow` (OpenGL).
- La crate `image` est importée avec les features `jpeg`, `png`, `webp` et `avif`. Le profil release active LTO, strip des symboles, et `panic = "abort"` pour réduire la taille.
