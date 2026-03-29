import { LitElement, html, css } from 'lit';
import { customElement, state } from 'lit/decorators.js';
import { signal } from '@lit-labs/signals';

// State
export const activeTab = signal<'templates' | 'drivers'>('templates');
export const isLoading = signal(false);
export const isStale = signal(false);

@customElement('marketplace-app')
export class MarketplaceApp extends LitElement {
  static styles = css`
    :host {
      display: block;
      min-height: 100vh;
      background: var(--color-bg);
    }

    .hero {
      background: linear-gradient(135deg, var(--color-primary) 0%, #3b82f6 100%);
      color: white;
      padding: var(--spacing-2xl) var(--spacing-md);
      text-align: center;
    }

    .hero h1 {
      font-size: 2.5rem;
      font-weight: 700;
      margin-bottom: var(--spacing-sm);
    }

    .hero p {
      font-size: 1.125rem;
      opacity: 0.9;
      margin-bottom: var(--spacing-lg);
    }

    .stale-banner {
      background: #fef3c7;
      border-bottom: 1px solid #f59e0b;
      padding: var(--spacing-sm) var(--spacing-md);
      text-align: center;
      font-size: 0.875rem;
      color: #92400e;
    }

    .stale-banner svg {
      vertical-align: middle;
      margin-right: var(--spacing-xs);
    }

    main {
      max-width: 1200px;
      margin: 0 auto;
      padding: var(--spacing-xl) var(--spacing-md);
    }
  `;

  @state() private _stale = false;

  connectedCallback() {
    super.connectedCallback();
    // Check for stale header on initial load
    this.checkStale();
  }

  private async checkStale() {
    try {
      const res = await fetch('/v1/templates?per_page=1');
      this._stale = res.headers.get('X-Cache-Stale') === 'true';
      isStale.set(this._stale);
    } catch (e) {
      console.error('Failed to check cache status:', e);
    }
  }

  render() {
    return html`
      ${this._stale ? html`
        <div class="stale-banner">
          <svg width="16" height="16" viewBox="0 0 20 20" fill="currentColor">
            <path fill-rule="evenodd" d="M8.257 3.099c.765-1.36 2.722-1.36 3.486 0l5.58 9.92c.75 1.334-.213 2.98-1.742 2.98H4.42c-1.53 0-2.493-1.646-1.743-2.98l5.58-9.92zM11 13a1 1 0 11-2 0 1 1 0 012 0zm-1-8a1 1 0 00-1 1v3a1 1 0 002 0V6a1 1 0 00-1-1z" clip-rule="evenodd"/>
          </svg>
          正在同步最新数据...
        </div>
      ` : ''}

      <div class="hero">
        <h1>TinyIoTHub 市场</h1>
        <p>发现、安装物联网模板和驱动</p>
      </div>

      <main>
        <slot></slot>
      </main>
    `;
  }
}
