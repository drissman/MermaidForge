# 🎨 MermaidForge

**Convertisseur Mermaid professionnel standalone pour Windows**  
*By Driss NAAMANE - Orange Business*

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.75+-orange.svg)](https://www.rust-lang.org)
[![Tauri](https://img.shields.io/badge/Tauri-2.0-blue.svg)](https://tauri.app)

---

## 🚀 Fonctionnalités

- ✅ **100% Standalone** : Aucune dépendance externe (Chromium embedded)
- ✅ **Batch Processing** : Convertir des dizaines de fichiers simultanément
- ✅ **Watch Mode** : Conversion automatique à la sauvegarde
- ✅ **Live Editor** : Prévisualisation temps réel avec Monaco Editor
- ✅ **Multi-formats** : SVG (transparent/light/dark) + PNG (HD/4K)
- ✅ **Portable** : Version clé USB + Version installeur
- ✅ **Branding Orange Business** : Thème corporate intégré

---

## 📦 Installation

### Mode Portable (Recommandé)
```bash
1. Télécharger MermaidForge_Portable.zip depuis Releases
2. Extraire sur votre disque ou clé USB
3. Double-clic sur MermaidForge.exe
4. ✅ Opérationnel !
```

### Mode Installé
```bash
1. Télécharger MermaidForge-setup.msi depuis Releases
2. Exécuter l'installeur (droits admin requis)
3. Lancer depuis le menu Démarrer
```

---

## 🛠️ Développement

### Prérequis
- **Rust** 1.75+ : https://rustup.rs
- **Node.js** 18+ : https://nodejs.org
- **Windows 11** avec WSL2 (optionnel mais recommandé)

### Installation des dépendances

```bash
# Cloner le repo
git clone https://github.com/VOTRE_USERNAME/MermaidForge.git
cd MermaidForge

# Installer dépendances frontend
npm install

# Installer dépendances Rust (automatique avec cargo)
cd src-tauri
cargo build
```

### Lancer en mode développement

```bash
# Depuis la racine du projet
npm run tauri dev
```

Une fenêtre s'ouvre avec hot-reload activé.

### Build de production

```bash
# Version portable
npm run tauri build -- --target portable

# Version installeur
npm run tauri build -- --target msi

# Les deux versions
npm run build:all
```

**Outputs** :
- `src-tauri/target/release/MermaidForge.exe` (Portable)
- `src-tauri/target/release/bundle/msi/MermaidForge_1.0.0_x64.msi` (Installeur)

---

## 📚 Architecture

```
MermaidForge/
├── src/                      # Frontend React + TypeScript
│   ├── App.tsx               # Layout principal
│   ├── components/           # Composants UI
│   │   ├── DropZone.tsx
│   │   ├── LiveEditor.tsx
│   │   ├── BatchPanel.tsx
│   │   └── Settings.tsx
│   └── styles/
│       └── orange-theme.css
│
├── src-tauri/                # Backend Rust
│   ├── src/
│   │   ├── main.rs           # Entry point Tauri
│   │   ├── lib.rs            # API publique
│   │   ├── converter/        # Moteur de conversion
│   │   │   ├── mod.rs
│   │   │   ├── renderer.rs   # headless_chrome
│   │   │   └── batch.rs      # Batch processing
│   │   ├── watcher/          # File watcher
│   │   │   └── mod.rs
│   │   └── config/           # Configuration
│   │       ├── mod.rs
│   │       ├── settings.rs
│   │       └── paths.rs
│   └── Cargo.toml
│
├── .github/
│   └── workflows/
│       └── build.yml         # CI/CD automatique
│
├── docs/                     # Documentation
│   ├── DEVELOPMENT.md
│   └── DEPLOYMENT.md
│
└── README.md                 # Ce fichier
```

---

## 🐙 Guide GitHub Desktop (Débutants)

### 1️⃣ Première fois : Cloner le repo

1. **Ouvrir GitHub Desktop**
2. **File → Clone Repository**
3. **Onglet "URL"**
4. Coller : `https://github.com/VOTRE_USERNAME/MermaidForge.git`
5. Choisir dossier local : `C:\Users\Driss\Projects\MermaidForge`
6. Cliquer **Clone**

✅ Le projet est maintenant sur votre PC !

### 2️⃣ Workflow quotidien

#### Faire des modifications
```
1. Ouvrir VSCode dans le dossier MermaidForge
2. Modifier des fichiers (ex: src/App.tsx)
3. Sauvegarder (Ctrl+S)
```

#### Commit les changements (GitHub Desktop)
```
1. Ouvrir GitHub Desktop
2. Voir la liste des fichiers modifiés (panneau gauche)
3. Cocher les fichiers à inclure
4. En bas à gauche :
   - Summary : "Ajout LiveEditor component"
   - Description : "Implémentation de l'éditeur Monaco avec preview"
5. Cliquer "Commit to main"
```

#### Push vers GitHub
```
1. En haut : "Push origin" (avec une flèche ↑)
2. Cliquer dessus
3. ✅ Vos changements sont sur GitHub !
```

### 3️⃣ Récupérer les derniers changements

Si vous travaillez depuis un autre PC :
```
1. Ouvrir GitHub Desktop
2. En haut : "Fetch origin"
3. Si changements détectés : "Pull origin"
4. ✅ Votre code local est à jour !
```

### 4️⃣ Créer une Release

```
1. Dans GitHub Desktop : "Repository → View on GitHub"
2. Sur GitHub.com : Onglet "Releases"
3. "Create a new release"
4. Tag : v1.0.0
5. Title : MermaidForge v1.0.0 - Initial Release
6. Upload assets :
   - MermaidForge_Portable.zip
   - MermaidForge-setup.msi
7. "Publish release"
```

---

## 🔧 Commandes Git utiles (Terminal)

```bash
# Voir l'état des fichiers
git status

# Ajouter tous les fichiers modifiés
git add .

# Commit avec message
git commit -m "Description du changement"

# Push vers GitHub
git push origin main

# Pull depuis GitHub
git pull origin main

# Voir l'historique
git log --oneline

# Créer une branche
git checkout -b feature/nouvelle-fonctionnalite

# Revenir à main
git checkout main
```

---

## 🎯 Roadmap

### Version 1.0.0 (Actuelle)
- [x] Conversion standalone .mmd → SVG/PNG
- [x] Batch processing
- [x] Watch mode
- [x] Live editor
- [x] Settings persistées
- [x] Branding Orange Business

### Version 1.1.0 (Prochaine)
- [ ] Export PDF
- [ ] Validation syntaxe Mermaid live
- [ ] Templates pré-configurés
- [ ] Multi-langue (FR/EN)

### Version 2.0.0 (Future)
- [ ] Cloud sync settings
- [ ] Collaboration temps réel
- [ ] Plugin Obsidian intégré
- [ ] API REST locale

---

## 🤝 Contribution

Les contributions sont bienvenues !

```bash
1. Fork le projet
2. Créer une branche : git checkout -b feature/amazing-feature
3. Commit : git commit -m 'Add amazing feature'
4. Push : git push origin feature/amazing-feature
5. Ouvrir une Pull Request
```

---

## 📄 License

MIT License - Voir [LICENSE](LICENSE)

---

## 👨‍💻 Auteur

**Driss NAAMANE**  
Senior Cloud Architect @ Orange Business

- 📧 Email : drissman@gmail.com
- 💼 LinkedIn : [Votre profil]
- 🐙 GitHub : [@VOTRE_USERNAME]

---

## 🙏 Remerciements

- [Tauri](https://tauri.app) - Framework pour apps desktop
- [Mermaid.js](https://mermaid.js.org) - Diagramming
- [headless_chrome](https://github.com/rust-headless-chrome/rust-headless-chrome) - Rendu Chromium
- Orange Business - Pour le support

---

## 📞 Support

- 🐛 **Issues** : https://github.com/VOTRE_USERNAME/MermaidForge/issues
- 📖 **Documentation** : https://github.com/VOTRE_USERNAME/MermaidForge/wiki
- 💬 **Discussions** : https://github.com/VOTRE_USERNAME/MermaidForge/discussions

---

**Made with ❤️ in Bretagne, France**
