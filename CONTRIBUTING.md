# Contributing to MermaidForge

Thank you for considering contributing to MermaidForge! 🎉

## 📋 Table of Contents

- [Code of Conduct](#code-of-conduct)
- [How Can I Contribute?](#how-can-i-contribute)
- [Development Setup](#development-setup)
- [Project Structure](#project-structure)
- [Coding Guidelines](#coding-guidelines)
- [Commit Messages](#commit-messages)
- [Pull Request Process](#pull-request-process)

---

## Code of Conduct

- Be respectful and inclusive
- Constructive feedback only
- Focus on the code, not the person
- Help others learn and grow

---

## How Can I Contribute?

### 🐛 Reporting Bugs

1. Check if the bug already exists in [Issues](https://github.com/VOTRE_USERNAME/MermaidForge/issues)
2. If not, create a new issue with:
   - Clear title
   - Steps to reproduce
   - Expected vs actual behavior
   - Screenshots if applicable
   - System info (Windows version, app version)

### 💡 Suggesting Features

1. Check [Discussions](https://github.com/VOTRE_USERNAME/MermaidForge/discussions)
2. Create a new discussion with:
   - Use case description
   - Expected behavior
   - Mockups or examples (optional)

### 🔧 Code Contributions

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test thoroughly
5. Submit a pull request

---

## Development Setup

### Prerequisites

- **Rust** 1.75+ (https://rustup.rs)
- **Node.js** 18+ (https://nodejs.org)
- **Git** (https://git-scm.com)
- **GitHub Desktop** (optional, for beginners)

### Installation

```bash
# Clone the repo
git clone https://github.com/VOTRE_USERNAME/MermaidForge.git
cd MermaidForge

# Install dependencies
npm install

# Start development server
npm run tauri dev
```

### Building

```bash
# Build portable version
npm run tauri:build:portable

# Build installer
npm run tauri:build:msi

# Build both
npm run build:all
```

---

## Project Structure

```
MermaidForge/
├── src/                    # Frontend React
│   ├── components/         # UI components
│   ├── styles/             # CSS files
│   └── utils/              # Helper functions
│
├── src-tauri/              # Backend Rust
│   ├── src/
│   │   ├── main.rs         # Entry point
│   │   ├── lib.rs          # Public API
│   │   ├── converter/      # Conversion logic
│   │   ├── config/         # Configuration
│   │   ├── watcher/        # File watcher
│   │   └── commands/       # Tauri commands
│   └── Cargo.toml
│
├── .github/                # GitHub configs
│   └── workflows/          # CI/CD
│
└── docs/                   # Documentation
```

---

## Coding Guidelines

### Rust

```rust
// Use descriptive names
fn convert_mermaid_to_svg(code: &str) -> Result<String> {
    // Implementation
}

// Add documentation
/// Converts Mermaid code to SVG format
/// 
/// # Arguments
/// * `code` - Mermaid diagram code
/// 
/// # Returns
/// SVG string or error
pub fn convert(code: &str) -> Result<String> {
    // ...
}

// Use Result for error handling
fn process() -> Result<(), Error> {
    let data = read_file()?;
    validate(data)?;
    Ok(())
}
```

### TypeScript/React

```typescript
// Use functional components
export function Component({ prop }: Props) {
  const [state, setState] = useState<Type>(initial);
  
  // Event handlers
  const handleClick = () => {
    // ...
  };
  
  return <div onClick={handleClick}>...</div>;
}

// Type everything
interface Props {
  name: string;
  onAction: (value: string) => void;
}

// Use hooks
const value = useMemo(() => compute(data), [data]);
useEffect(() => {
  // Side effect
  return () => cleanup();
}, [dependencies]);
```

### CSS

```css
/* Use CSS variables */
:root {
  --primary-color: #ff7900;
}

/* BEM naming */
.component {}
.component__element {}
.component--modifier {}

/* Mobile-first */
.container {
  /* Mobile styles */
}

@media (min-width: 768px) {
  .container {
    /* Tablet styles */
  }
}
```

---

## Commit Messages

### Format

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Types

- **feat**: New feature
- **fix**: Bug fix
- **docs**: Documentation
- **style**: Formatting, missing semi colons, etc
- **refactor**: Code restructuring
- **test**: Adding tests
- **chore**: Maintenance

### Examples

```
feat(converter): add PDF export support

Implemented PDF rendering using headless_chrome.
Supports A4 and Letter paper sizes.

Closes #42
```

```
fix(ui): resolve drag-drop crash on large files

Files >100MB were causing memory issues.
Added file size validation and streaming.

Fixes #38
```

---

## Pull Request Process

### Before Submitting

1. **Test your changes**
   ```bash
   npm run tauri dev    # Manual testing
   cargo test           # Unit tests
   npm run lint         # Code quality
   ```

2. **Update documentation** if needed
3. **Update CHANGELOG.md**
4. **Ensure builds work**
   ```bash
   npm run build:all
   ```

### Submitting

1. **Push your branch**
   ```bash
   git push origin feature/your-feature
   ```

2. **Create Pull Request** on GitHub
   - Clear title
   - Description of changes
   - Link related issues
   - Add screenshots if UI changes

3. **Wait for review**
   - Address feedback
   - Make requested changes
   - Push updates

4. **Merge** (after approval)

---

## Testing

### Manual Testing

1. Build and run the app
2. Test your feature thoroughly
3. Test on clean install (portable mode)
4. Test edge cases

### Automated Testing

```bash
# Rust tests
cd src-tauri
cargo test

# Frontend tests (when implemented)
npm test
```

---

## Questions?

- 💬 **Discussions**: https://github.com/VOTRE_USERNAME/MermaidForge/discussions
- 📧 **Email**: drissman@gmail.com
- 🐛 **Issues**: https://github.com/VOTRE_USERNAME/MermaidForge/issues

---

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

**Thank you for contributing to MermaidForge!** 🚀
