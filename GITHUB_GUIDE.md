# 🐙 Guide GitHub Desktop - Pour Débutants

**Guide complet pour utiliser MermaidForge avec GitHub et GitHub Desktop**

---

## 📥 Installation

### 1. Créer un compte GitHub (si pas déjà fait)
1. Aller sur https://github.com
2. Cliquer "Sign up"
3. Suivre les étapes
4. Vérifier votre email

### 2. Installer GitHub Desktop
1. Télécharger depuis https://desktop.github.com
2. Exécuter l'installeur
3. Se connecter avec votre compte GitHub
4. ✅ Installation terminée !

---

## 🚀 Premier Push : Créer le repo sur GitHub

### Étape 1 : Créer le repository sur GitHub.com

1. **Aller sur GitHub.com** et se connecter
2. **Cliquer sur "+" en haut à droite** → "New repository"
3. **Remplir le formulaire** :
   ```
   Repository name: MermaidForge
   Description: Professional Mermaid diagram converter
   ☐ Public (si vous voulez le partager)
   ☑ Private (si vous voulez le garder privé)
   ☐ Initialize with README (on a déjà un README)
   ☐ Add .gitignore (on en a déjà un)
   ☐ Choose a license (on en a déjà une)
   ```
4. **Cliquer "Create repository"**

### Étape 2 : Lier votre projet local au repo GitHub

1. **Ouvrir GitHub Desktop**
2. **File → Add Local Repository**
3. **Choisir le dossier** : `C:\Users\Driss\MermaidForge`
4. **Cliquer "Add Repository"**

### Étape 3 : Premier commit

Dans GitHub Desktop :

1. **Voir tous les fichiers** dans le panneau de gauche
2. **En bas à gauche, remplir** :
   ```
   Summary: Initial commit - MermaidForge v1.0.0
   Description: 
   - Complete Tauri + Rust + React architecture
   - Portable and installed modes
   - Batch conversion support
   - Orange Business branding
   ```
3. **Cliquer "Commit to main"**

### Étape 4 : Push vers GitHub

1. **En haut de GitHub Desktop** : cliquer "Publish repository"
2. **Décocher "Keep this code private"** si vous voulez le rendre public
3. **Cliquer "Publish repository"**
4. ✅ **Votre code est maintenant sur GitHub !**

### Étape 5 : Vérifier sur GitHub.com

1. Aller sur `https://github.com/VOTRE_USERNAME/MermaidForge`
2. Vous devriez voir tous vos fichiers !

---

## 🔄 Workflow Quotidien

### Faire des modifications

1. **Travailler sur le code** dans VSCode/votre éditeur
2. **Sauvegarder vos fichiers** (Ctrl+S)

### Commit les changements

1. **Ouvrir GitHub Desktop**
2. **Voir les fichiers modifiés** dans le panneau gauche
3. **Cocher les fichiers à inclure** (par défaut tous cochés)
4. **Écrire un message de commit** :
   ```
   Summary: Add live preview feature
   Description: Implemented Monaco editor with real-time Mermaid preview
   ```
5. **Cliquer "Commit to main"**

### Push vers GitHub

1. **En haut** : cliquer le bouton **"Push origin"** (avec flèche ↑)
2. ✅ **Vos changements sont sur GitHub !**

---

## 🌿 Travailler avec des Branches

### Pourquoi des branches ?

- **main** : Code stable, toujours fonctionnel
- **feature/nouvelle-fonctionnalite** : Développement en cours
- Vous pouvez tester sans casser `main`

### Créer une branche

1. **GitHub Desktop** → En haut : **Current Branch: main**
2. **Cliquer dessus** → **New Branch**
3. **Nom de la branche** : `feature/live-editor`
4. **Cliquer "Create Branch"**
5. **Publish branch** (push vers GitHub)

### Travailler sur la branche

1. Faire vos modifications
2. Commit comme d'habitude
3. Push comme d'habitude
4. ✅ Les modifications sont sur `feature/live-editor`, pas sur `main`

### Merger la branche dans main

Quand votre feature est terminée :

1. **GitHub Desktop** → **Branch** → **Create Pull Request**
2. Sur GitHub.com, la page de Pull Request s'ouvre
3. **Cliquer "Create Pull Request"**
4. **Ajouter une description** de ce que vous avez fait
5. **Cliquer "Merge Pull Request"**
6. **Cliquer "Confirm merge"**
7. ✅ Votre feature est maintenant dans `main` !

Dans GitHub Desktop :

1. **Revenir sur main** : Current Branch → `main`
2. **Fetch origin** (pour récupérer les changements)
3. **Pull origin** (pour mettre à jour votre code local)

---

## 📦 Créer une Release (Version)

### Quand créer une release ?

- Version stable prête à distribuer
- Toutes les features de la version fonctionnent
- Tests passés

### Comment créer une release

#### Méthode 1 : Via GitHub.com (Recommandé)

1. **Aller sur** `https://github.com/VOTRE_USERNAME/MermaidForge`
2. **Cliquer sur "Releases"** (colonne de droite)
3. **"Create a new release"**
4. **Remplir le formulaire** :
   ```
   Tag version: v1.0.0
   Release title: MermaidForge v1.0.0 - Initial Release
   
   Description:
   ## 🎉 First Release
   
   ### Features
   - ✅ Standalone Mermaid conversion (SVG, PNG)
   - ✅ Batch processing
   - ✅ Portable and installed modes
   - ✅ Orange Business branding
   
   ### Downloads
   - **Portable**: MermaidForge_Portable.zip (150 MB)
   - **Installer**: MermaidForge-setup.msi (15 MB)
   
   ### Installation
   See [README.md](README.md) for installation instructions.
   ```
5. **Attacher les fichiers** :
   - Cliquer "Attach binaries"
   - Uploader `MermaidForge.exe` (portable)
   - Uploader `MermaidForge-setup.msi` (installeur)
6. **Cliquer "Publish release"**
7. ✅ **Release publiée !**

#### Méthode 2 : Via Git Tag (Automatique avec GitHub Actions)

Si vous avez configuré GitHub Actions (déjà fait dans le projet) :

```bash
# Dans Git Bash ou terminal
git tag v1.0.0
git push origin v1.0.0
```

GitHub Actions va automatiquement :
1. Builder le projet
2. Créer la release
3. Uploader les fichiers

---

## 🔍 Commandes Git Utiles

### Dans GitHub Desktop (sans terminal)

**Tout se fait avec l'interface graphique !**

1. **Voir l'historique** : Cliquer sur "History"
2. **Voir un commit** : Cliquer dessus dans l'historique
3. **Comparer des versions** : Cliquer sur 2 commits avec Ctrl

### Dans Terminal (optionnel, pour les curieux)

```bash
# Voir l'état
git status

# Voir l'historique
git log --oneline --graph

# Voir les différences
git diff

# Annuler des modifications (ATTENTION !)
git checkout -- fichier.txt

# Voir les branches
git branch -a

# Changer de branche
git checkout main
```

---

## 🆘 Dépannage

### "Repository not found"

**Solution** : Vérifier que le repo existe sur GitHub.com

### "Authentication failed"

**Solution** : 
1. GitHub Desktop → File → Options → Accounts
2. Sign out puis Sign in again

### "Push rejected"

**Solution** : 
1. Fetch origin
2. Pull origin
3. Résoudre les conflits si nécessaire
4. Push again

### "Merge conflict"

**Solution** :
1. GitHub Desktop détecte automatiquement
2. Ouvrir les fichiers en conflit
3. Choisir quelle version garder
4. Sauvegarder
5. Commit le merge

### "Cannot push - need to pull first"

**Solution** :
1. Fetch origin
2. Pull origin
3. Push origin

---

## 🎓 Workflow Complet Exemple

### Scénario : Ajouter une nouvelle feature

```
1. GitHub Desktop → Create Branch → "feature/settings-panel"
2. Coder la feature dans VSCode
3. GitHub Desktop → Commit ("Add settings panel")
4. GitHub Desktop → Push origin
5. GitHub.com → Create Pull Request
6. Review + Merge
7. GitHub Desktop → Switch to main
8. GitHub Desktop → Pull origin
9. ✅ Feature intégrée !
```

---

## 📚 Ressources

- **GitHub Desktop Docs** : https://docs.github.com/en/desktop
- **Git Basics** : https://git-scm.com/book/en/v2
- **Markdown Guide** : https://guides.github.com/features/mastering-markdown/

---

## 💡 Bonnes Pratiques

### Messages de Commit

❌ **Mauvais** :
```
fix
update
wip
```

✅ **Bon** :
```
Add batch conversion progress bar
Fix crash when opening large files
Update README with installation steps
```

### Fréquence de Commit

- **Commit souvent** : Chaque feature/bugfix = 1 commit
- **Push régulièrement** : Au moins 1x par jour
- **Ne pas commit** : `node_modules/`, `target/`, fichiers temporaires

### Branches

- `main` : Toujours stable
- `feature/xxx` : Nouvelles features
- `fix/xxx` : Bugfixes
- `docs/xxx` : Documentation

---

## 🎯 Checklist Premier Push

- [ ] Compte GitHub créé
- [ ] GitHub Desktop installé
- [ ] Repository créé sur GitHub.com
- [ ] Projet local lié au repo
- [ ] Premier commit fait
- [ ] Push vers GitHub réussi
- [ ] Code visible sur GitHub.com
- [ ] README.md affiché correctement

**Si tout est ✅ : BRAVO ! Vous maîtrisez GitHub !** 🎉

---

**Besoin d'aide ?** Ouvrir une issue sur GitHub ou contacter drissman@gmail.com
