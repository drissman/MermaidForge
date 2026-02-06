import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';
import './App.css';

interface ConversionResult {
  success: boolean;
  output_path?: string;
  error?: string;
}

interface BatchResult {
  total: number;
  converted: number;
  failed: number;
  files: string[];
}

function App() {
  const [mermaidCode, setMermaidCode] = useState(`graph TD
    A[Client] -->|tcp_123| B
    B(Load Balancer)
    B -->|tcp_456| C[Server1]
    B -->|tcp_456| D[Server2]`);
  
  const [format, setFormat] = useState<'svg' | 'png'>('svg');
  const [scale, setScale] = useState<number>(1);
  const [customWidth, setCustomWidth] = useState<string>('');
  const [customHeight, setCustomHeight] = useState<string>('');
  const [bgColor, setBgColor] = useState<string>('');
  const [status, setStatus] = useState<string>('');
  const [isConverting, setIsConverting] = useState(false);
  const [previewUrl, setPreviewUrl] = useState<string | null>(null);
  const [batchProgress, setBatchProgress] = useState<string>('');

  const handleConvert = async () => {
    setIsConverting(true);
    setStatus('Conversion en cours...');
    setPreviewUrl(null);

    try {
      const result = await invoke<ConversionResult>('convert_mermaid', {
        mermaidCode,
        format,
        scale: scale,
        width: customWidth ? parseInt(customWidth) : null,
        height: customHeight ? parseInt(customHeight) : null,
        bgColor: bgColor || null,
      });

      if (result.success && result.output_path) {
        setStatus(`✅ Fichier généré : ${result.output_path}`);
        const preview = await invoke<string>('read_file_as_base64', {
          path: result.output_path,
        });
        setPreviewUrl(`data:image/${format === 'svg' ? 'svg+xml' : 'png'};base64,${preview}`);
      } else {
        setStatus(`❌ Erreur : ${result.error || 'Conversion échouée'}`);
      }
    } catch (error) {
      setStatus(`❌ Erreur : ${error}`);
    } finally {
      setIsConverting(false);
    }
  };

  const handleBatchConvert = async () => {
    try {
      const folder = await open({
        directory: true,
        multiple: false,
        title: 'Sélectionner un dossier contenant des fichiers .mmd',
      });

      if (!folder) return;

      setIsConverting(true);
      setBatchProgress('🔍 Lecture du dossier...');
      setStatus('');
      setPreviewUrl(null);

      const result = await invoke<BatchResult>('batch_convert_folder', {
        folderPath: folder,
        format: format,
        scale: scale,
        width: customWidth ? parseInt(customWidth) : null,
        height: customHeight ? parseInt(customHeight) : null,
        bgColor: bgColor || null,
      });

      setBatchProgress(`✅ Conversion terminée : ${result.converted}/${result.total} fichiers`);

      if (result.failed > 0) {
        setStatus(`⚠️ ${result.failed} fichier(s) en échec`);
      } else {
        setStatus(`✅ Tous les fichiers convertis avec succès !`);
      }

      // Show preview of the last converted file
      if (result.files.length > 0) {
        const lastFile = result.files[result.files.length - 1];
        const preview = await invoke<string>('read_file_as_base64', {
          path: lastFile,
        });
        setPreviewUrl(`data:image/${format === 'svg' ? 'svg+xml' : 'png'};base64,${preview}`);
      }
      
    } catch (error) {
      setStatus(`❌ Erreur : ${error}`);
      setBatchProgress('');
    } finally {
      setIsConverting(false);
    }
  };

  const handleClear = () => {
    setMermaidCode('');
    setStatus('');
    setPreviewUrl(null);
    setBatchProgress('');
  };

  const handleOpenOutputs = async () => {
    try {
      await invoke('open_output_folder');
    } catch (error) {
      setStatus(`❌ Erreur : ${error}`);
    }
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
            <select
              value={scale}
              onChange={(e) => setScale(parseFloat(e.target.value))}
              disabled={isConverting}
              title="Échelle de résolution (PNG)"
            >
              <option value={1}>1x (défaut)</option>
              <option value={1.5}>1.5x</option>
              <option value={2}>2x (HD)</option>
              <option value={3}>3x (max)</option>
            </select>
            <button onClick={handleExample} disabled={isConverting}>
              📄 Exemple
            </button>
            <button onClick={handleBatchConvert} disabled={isConverting}>
              📁 Dossier
            </button>
            <button onClick={handleOpenOutputs} disabled={isConverting}>
              📂 Outputs
            </button>
            <button onClick={handleClear} disabled={isConverting}>
              🗑️ Effacer
            </button>
          </div>

          <div className="resolution-options">
            <input
              type="number"
              placeholder="Largeur (px)"
              value={customWidth}
              onChange={(e) => setCustomWidth(e.target.value)}
              disabled={isConverting}
              min="100"
              max="10000"
            />
            <input
              type="number"
              placeholder="Hauteur (px)"
              value={customHeight}
              onChange={(e) => setCustomHeight(e.target.value)}
              disabled={isConverting}
              min="100"
              max="10000"
            />
            <input
              type="color"
              value={bgColor || '#ffffff'}
              onChange={(e) => setBgColor(e.target.value)}
              disabled={isConverting}
              title="Couleur de fond"
            />
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

          {batchProgress && (
            <div className="status info">
              {batchProgress}
            </div>
          )}

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