import { LitElement, html, css } from 'lit';
import { customElement } from 'lit/decorators.js';
import { activeTab } from './marketplace-app';

@customElement('marketplace-header')
export class MarketplaceHeader extends LitElement {
  static styles = css`
    :host {
      display: block;
      border-bottom: 1px solid var(--color-border);
      background: var(--color-bg);
      position: sticky;
      top: 0;
      z-index: 100;
    }

    .header-content {
      max-width: 1200px;
      margin: 0 auto;
      padding: var(--spacing-md);
      display: flex;
      align-items: center;
      justify-content: space-between;
      gap: var(--spacing-md);
    }

    .tabs {
      display: flex;
      gap: var(--spacing-sm);
    }

    .tab {
      padding: var(--spacing-sm) var(--spacing-md);
      border: none;
      background: transparent;
      color: var(--color-text-secondary);
      font-size: 0.9375rem;
      font-weight: 500;
      border-radius: var(--radius-md);
      cursor: pointer;
      transition: all 0.2s ease;
    }

    .tab:hover {
      background: var(--color-tag-bg);
      color: var(--color-primary);
    }

    .tab.active {
      background: var(--color-primary);
      color: white;
    }
  `;

  private _handleTabClick(tab: 'templates' | 'drivers') {
    activeTab.set(tab);
    this.dispatchEvent(new CustomEvent('tab-change', {
      detail: { tab },
      bubbles: true,
      composed: true,
    }));
  }

  render() {
    const currentTab = activeTab.get();

    return html`
      <header>
        <div class="header-content">
          <div class="tabs">
            <button
              class="tab ${currentTab === 'templates' ? 'active' : ''}"
              @click=${() => this._handleTabClick('templates')}
            >
              模板
            </button>
            <button
              class="tab ${currentTab === 'drivers' ? 'active' : ''}"
              @click=${() => this._handleTabClick('drivers')}
            >
              驱动
            </button>
          </div>
        </div>
      </header>
    `;
  }
}
