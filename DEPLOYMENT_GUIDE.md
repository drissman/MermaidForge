# 🚀 MermaidForge - Guide de Mise à Jour et GitHub

## 📋 Vous avez déjà un dossier MermaidForge incomplet

**Ce guide vous aide à compléter votre projet existant et le pousser sur GitHub.**

---

## Étape 1 : Copier les nouveaux fichiers (5 min)

### A. Extraction de l'archive

Vous avez reçu **MermaidForge-Complete.tar.gz** contenant tous les fichiers manquants.

**Dans WSL2 Ubuntu** :

```bash
# Aller dans votre dossier MermaidForge existant
cd /mnt/c/Users/bxzn0117/MermaidForge

# Copier l'archive ici
cp ~/Downloads/MermaidForge-Complete.tar.gz .

# Extraire (va compléter ce qui manque)
tar -xzf MermaidForge-Complete.tar.gz --strip-components=1

# Vérifier que les dossiers src/ et src-tauri/ existent maintenant
ls -la
```

**Résultat attendu** :
```
✅ src/                  (nouveau)
✅ src-tauri/            (nouveau)
✅ public/               (nouveau)
✅ package.json          (déjà existant)
✅ README.md             (déjà existant)
```

---

## Étape 2 : Installer les dépendances (10 min)

**Dans WSL2 Ubuntu** :

```bash
cd /mnt/c/Users/bxzn0117/MermaidForge

# Installer dépendances Node.js (déjà fait normalement)
npm install

# Vérifier que Rust est installé
rustc --version

# Si Rust n'est pas installé :
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

---

## Étape 3 : Tester en mode dev (5 min)

**Dans WSL2 Ubuntu** :

```bash
cd /mnt/c/Users/bxzn0117/MermaidForge

# Lancer en mode développement
npm run tauri dev
```

**✅ Si une fenêtre s'ouvre avec l'interface MermaidForge : SUCCÈS !**

**❌ Si erreur** : Copiez l'erreur complète et contactez-moi.

Fermez la fenêtre (Ctrl+C dans le terminal pour arrêter).

---

## Étape 4 : Pousser sur GitHub (15 min)

### A. Vérifier Git

**Dans WSL2 Ubuntu** :

```bash
cd /mnt/c/Users/bxzn0117/MermaidForge

# Vérifier status Git
git status

# Si erreur "not a git repository"
git init
```

### B. Configurer Git (si pas déjà fait)

```bash
git config --global user.name "Driss NAAMANE"
git config --global user.email "drissman@gmail.com"
```

### C. Ajouter tous les fichiers

```bash
# Ajouter tous les nouveaux fichiers
git add .

# Vérifier ce qui sera commité
git status

# Commit
git commit -m "feat: Add complete source code (React frontend + Rust backend)"
```

### D. Créer le repo sur GitHub

**Option 1 : Via GitHub Desktop (RECOMMANDÉ pour débutant)**

1. **Ouvrir GitHub Desktop** (application Windows)
2. **File → Add Local Repository**
3. **Sélectionner** : `C:\Users\bxzn0117\MermaidForge`
4. **Cliquer** "Publish repository"
5. **Configurer** :
   - Name: `MermaidForge`
   - Description: "Convertisseur Mermaid professionnel standalone pour Windows"
   - ✅ Keep this code private (si vous voulez privé)
   - ⬜ Keep this code private (si vous voulez public)
6. **Cliquer** "Publish repository"

**✅ TERMINÉ ! Votre code est sur GitHub !**

Vérifier : https://github.com/drissman/MermaidForge

---

**Option 2 : Via ligne de commande** (si GitHub Desktop ne fonctionne pas)

```bash
cd /mnt/c/Users/bxzn0117/MermaidForge

# Créer le repo sur GitHub (remplacer TOKEN par votre Personal Access Token)
# Obtenir token : https://github.com/settings/tokens

# Ajouter remote
git remote add origin https://github.com/drissman/MermaidForge.git

# Pousser
git branch -M main
git push -u origin main
```

---

## Étape 5 : Builder la version finale (OPTIONNEL)

**Si vous voulez distribuer l'application aux collègues** :

```bash
cd /mnt/c/Users/bxzn0117/MermaidForge

# Build complet (Portable + Installeur)
npm run build:all

# Fichiers générés dans :
# src-tauri/target/release/MermaidForge.exe        (Portable ~150 MB)
# src-tauri/target/release/bundle/msi/*.msi        (Installeur ~15 MB)

# Copier sur le Bureau Windows
mkdir -p /mnt/c/Users/bxzn0117/Desktop/MermaidForge-Distribution
cp src-tauri/target/release/MermaidForge.exe /mnt/c/Users/bxzn0117/Desktop/MermaidForge-Distribution/
cp src-tauri/target/release/bundle/msi/*.msi /mnt/c/Users/bxzn0117/Desktop/MermaidForge-Distribution/
```

---

## 🆘 Troubleshooting

### Erreur "src/ : No such file or directory"

→ L'extraction de l'archive a échoué. Réessayez :

```bash
cd /mnt/c/Users/bxzn0117/MermaidForge
tar -xzf MermaidForge-Complete.tar.gz --strip-components=1 --overwrite
```

### Erreur "npm: command not found"

→ Node.js n'est pas installé dans WSL2 :

```bash
curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
sudo apt-get install -y nodejs
```

### Erreur "cargo: command not found"

→ Rust n'est pas installé :

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

### GitHub Desktop ne voit pas le dossier

→ Le dossier est dans Windows, GitHub Desktop devrait le voir.

Si problème :
1. File → Options → Git → Refresh
2. OU utiliser la ligne de commande (Option 2)

---

## ✅ Checklist Finale

- [ ] Archive extraite dans dossier existant
- [ ] Dossiers `src/` et `src-tauri/` présents
- [ ] `npm install` terminé sans erreur
- [ ] `npm run tauri dev` lance l'app
- [ ] Git commit créé
- [ ] Code poussé sur GitHub
- [ ] URL GitHub fonctionne : https://github.com/drissman/MermaidForge

---

**Besoin d'aide ? Contactez-moi avec le message d'erreur complet ! 🚀**
