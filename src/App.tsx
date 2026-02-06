import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import './App.css';

interface ConversionResult {
  success: boolean;
  output_path?: string;
  error?: string;
}

function App() {
  const [mermaidCode, setMermaidCode] = useState(`graph TD
    A[Client] -->|tcp_123| B
    B(Load Balancer)
    B -->|tcp_456| C[Server1]
    B -->|tcp_456| D[Server2]`);
  
  const [format, setFormat] = useState<'svg' | 'png'>('svg');
  const [status, setStatus] = useState<string>('');
  const [isConverting, setIsConverting] = useState(false);
  const [previewUrl, setPreviewUrl] = useState<string | null>(null);

  const handleConvert = async () => {
    setIsConverting(true);
    setStatus('Conversion en cours...');
    setPreviewUrl(null);

    try {
      const result = await invoke<ConversionResult>('convert_mermaid', {
        mermaidCode,
        format,
      });

      if (result.success && result.output_path) {
        setStatus(`✅ Fichier généré : ${result.output_path}`);
        const preview = await invoke<string>('read_file_as_base64', {
          path: result.output_path,
        });
        setPreviewUrl(`data:image/${format};base64,${preview}`);
      } else {
        setStatus(`❌ Erreur : ${result.error || 'Conversion échouée'}`);
      }
    } catch (error) {
      setStatus(`❌ Erreur : ${error}`);
    } finally {
      setIsConverting(false);
    }
  };

  const handleClear = () => {
    setMermaidCode('');
    setStatus('');
    setPreviewUrl(null);
  };

  const handleExample = () => {
    setMermaidCode(`sequenceDiagram
    participant A as Alice
    participant B as Bob
    A->>B: Bonjour Bob !
    B->>A: Bonjour Alice !
    Note over A,B: Communication sécurisée`);
  };

  return (
    <div className="app">
      <header className="app-header">
        <h1>🎨 MermaidForge</h1>
        <p>Convertisseur Mermaid professionnel</p>
      </header>

      <div className="container">
        <div className="editor-panel">
          <div className="toolbar">
            <select 
              value={format} 
              onChange={(e) => setFormat(e.target.value as 'svg' | 'png')}
              disabled={isConverting}
            >
              <option value="svg">SVG (vectoriel)</option>
              <option value="png">PNG (image)</option>
            </select>
            <button onClick={handleExample} disabled={isConverting}>
              📄 Exemple
            </button>
            <button onClick={handleClear} disabled={isConverting}>
              🗑️ Effacer
            </button>
          </div>

          <textarea
            className="mermaid-input"
            value={mermaidCode}
            onChange={(e) => setMermaidCode(e.target.value)}
            placeholder="Collez votre code Mermaid ici..."
            disabled={isConverting}
          />

          <button 
            className="convert-button" 
            onClick={handleConvert}
            disabled={isConverting || !mermaidCode.trim()}
          >
            {isConverting ? '⏳ Conversion...' : '🚀 Convertir'}
          </button>

          {status && (
            <div className={`status ${status.startsWith('✅') ? 'success' : 'error'}`}>
              {status}
            </div>
          )}
        </div>

        <div className="preview-panel">
          <h3>Aperçu</h3>
          {previewUrl ? (
            <div className="preview-container">
              <img src={previewUrl} alt="Mermaid preview" />
            </div>
          ) : (
            <div className="preview-placeholder">
              <p>👁️ L'aperçu apparaîtra ici après conversion</p>
            </div>
          )}
        </div>
      </div>

      <footer className="app-footer">
        <p>
          Made with ❤️ by Driss NAAMANE - Orange Business | 
          <a href="https://github.com/drissman/MermaidForge" target="_blank" rel="noopener noreferrer">
            GitHub
          </a>
        </p>
      </footer>
    </div>
  );
}

export default App;
