import { LitElement, html, css } from 'lit';
import { customElement, property, state } from 'lit/decorators.js';

@customElement('search-box')
export class SearchBox extends LitElement {
  static styles = css`
    :host {
      display: block;
    }

    .search-wrapper {
      position: relative;
      max-width: 500px;
      margin: 0 auto;
    }

    input {
      width: 100%;
      padding: var(--spacing-sm) var(--spacing-md);
      padding-left: 40px;
      border: 1px solid var(--color-border);
      border-radius: var(--radius-lg);
      font-size: 1rem;
      outline: none;
      transition: border-color 0.2s ease, box-shadow 0.2s ease;
    }

    input:focus {
      border-color: var(--color-primary);
      box-shadow: 0 0 0 3px rgba(37, 99, 235, 0.1);
    }

    .search-icon {
      position: absolute;
      left: 12px;
      top: 50%;
      transform: translateY(-50%);
      color: var(--color-text-secondary);
      pointer-events: none;
    }
  `;

  @property({ type: String }) value = '';
  @property({ type: Number }) debounceMs = 300;

  @state() private _debounceTimer: number | null = null;

  private _handleInput(e: Event) {
    const input = e.target as HTMLInputElement;
    this.value = input.value;

    if (this._debounceTimer !== null) {
      clearTimeout(this._debounceTimer);
    }

    this._debounceTimer = window.setTimeout(() => {
      this.dispatchEvent(new CustomEvent('search', {
        detail: { query: this.value },
        bubbles: true,
        composed: true,
      }));
    }, this.debounceMs);
  }

  render() {
    return html`
      <div class="search-wrapper">
        <svg class="search-icon" width="20" height="20" viewBox="0 0 20 20" fill="currentColor">
          <path fill-rule="evenodd" d="M8 4a4 4 0 100 8 4 4 0 000-8zM2 8a6 6 0 1110.89 3.476l4.817 4.817a1 1 0 01-1.414 1.414l-4.816-4.816A6 6 0 012 8z" clip-rule="evenodd"/>
        </svg>
        <input
          type="text"
          placeholder="搜索模板..."
          .value=${this.value}
          @input=${this._handleInput}
        />
      </div>
    `;
  }
}
