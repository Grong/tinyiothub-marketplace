import { LitElement, html, css } from 'lit';
import { customElement, property } from 'lit/decorators.js';

@customElement('download-button')
export class DownloadButton extends LitElement {
  static styles = css`
    :host {
      display: inline-block;
    }

    button {
      padding: 6px 12px;
      background: var(--color-primary);
      color: white;
      border: none;
      border-radius: var(--radius-md);
      font-size: 0.8125rem;
      font-weight: 500;
      cursor: pointer;
      transition: background-color 0.2s ease;
      display: flex;
      align-items: center;
      gap: 4px;
    }

    button:hover {
      background: var(--color-primary-hover);
    }

    button:active {
      transform: scale(0.98);
    }

    svg {
      width: 14px;
      height: 14px;
    }
  `;

  @property({ type: String }) itemId = '';
  @property({ type: String }) itemType: 'template' | 'driver' = 'template';

  private _handleClick(e: Event) {
    e.stopPropagation();
    // Navigate to download endpoint
    window.location.href = `/v1/${this.itemType}s/${this.itemId}/download`;
  }

  render() {
    return html`
      <button @click=${this._handleClick}>
        <svg viewBox="0 0 20 20" fill="currentColor">
          <path fill-rule="evenodd" d="M3 17a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm3.293-7.707a1 1 0 011.414 0L9 10.586V3a1 1 0 112 0v7.586l1.293-1.293a1 1 0 111.414 1.414l-3 3a1 1 0 01-1.414 0l-3-3a1 1 0 010-1.414z" clip-rule="evenodd"/>
        </svg>
        下载
      </button>
    `;
  }
}
