import { LitElement, html, css } from 'lit';
import { customElement, state } from 'lit/decorators.js';
import { signal } from '@lit-labs/signals';
import './driver-card';
import './search-box';
import './protocol-filter';
import './sort-dropdown';

export interface DriverItem {
  id: string;
  name: string;
  version: string;
  protocol: string;
  description: string;
  tags: string[];
  author_name: string;
  icon?: string;
  license: string;
  downloads: number;
  rating?: number;
  updated_at: string;
}

export const drivers = signal<DriverItem[]>([]);

@customElement('driver-list')
export class DriverList extends LitElement {
  static styles = css`
    :host {
      display: block;
    }

    .filters {
      display: flex;
      flex-wrap: wrap;
      gap: var(--spacing-md);
      margin-bottom: var(--spacing-lg);
      align-items: center;
      justify-content: space-between;
    }

    .filter-group {
      display: flex;
      flex-wrap: wrap;
      gap: var(--spacing-sm);
      align-items: center;
    }

    .grid {
      display: grid;
      grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
      gap: var(--spacing-lg);
    }

    .loading {
      text-align: center;
      padding: var(--spacing-2xl);
      color: var(--color-text-secondary);
    }

    .empty {
      text-align: center;
      padding: var(--spacing-2xl);
      color: var(--color-text-secondary);
    }
  `;

  @state() private _loading = false;
  @state() private _drivers: DriverItem[] = [];
  @state() private _searchQuery = '';
  @state() private _protocol = '';

  connectedCallback() {
    super.connectedCallback();
    this.loadDrivers();
  }

  private async loadDrivers() {
    this._loading = true;
    try {
      const params = new URLSearchParams({ page: '1', per_page: '100' });
      if (this._searchQuery) params.set('search', this._searchQuery);
      if (this._protocol) params.set('protocol', this._protocol);

      const res = await fetch(`/v1/drivers?${params}`);
      const data = await res.json();

      if (data.code === 0 && data.result) {
        this._drivers = data.result.items;
        drivers.set(this._drivers);
      }
    } catch (e) {
      console.error('Failed to load drivers:', e);
    } finally {
      this._loading = false;
    }
  }

  private _handleSearch(e: CustomEvent<{ query: string }>) {
    this._searchQuery = e.detail.query;
    this.loadDrivers();
  }

  private _handleProtocolChange(e: CustomEvent<{ value: string }>) {
    this._protocol = e.detail.value;
    this.loadDrivers();
  }

  render() {
    return html`
      <div class="filters">
        <div class="filter-group">
          <search-box
            @search=${this._handleSearch}
            placeholder="搜索驱动..."
          ></search-box>
          <protocol-filter @change=${this._handleProtocolChange}></protocol-filter>
        </div>
        <sort-dropdown></sort-dropdown>
      </div>

      ${this._loading ? html`
        <div class="loading">加载中...</div>
      ` : this._drivers.length === 0 ? html`
        <div class="empty">未找到驱动</div>
      ` : html`
        <div class="grid">
          ${this._drivers.map(d => html`
            <driver-card .driver=${d}></driver-card>
          `)}
        </div>
      `}
    `;
  }
}
