import { LitElement, html, css } from 'lit';
import { customElement, state } from 'lit/decorators.js';
import { signal } from '@lit-labs/signals';
import './template-card';
import './search-box';
import './category-filter';
import './protocol-filter';
import './sort-dropdown';

export interface TemplateItem {
  id: string;
  name: string;
  version: string;
  category: string;
  protocol: string;
  manufacturer?: string;
  description: string;
  tags: string[];
  author_name: string;
  icon?: string;
  license: string;
  downloads: number;
  rating?: number;
  size?: number;
  updated_at: string;
}

export const templates = signal<TemplateItem[]>([]);

@customElement('template-list')
export class TemplateList extends LitElement {
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
  @state() private _templates: TemplateItem[] = [];
  @state() private _searchQuery = '';
  @state() private _category = '';
  @state() private _protocol = '';

  connectedCallback() {
    super.connectedCallback();
    this.loadTemplates();
  }

  private async loadTemplates() {
    this._loading = true;
    try {
      const params = new URLSearchParams({ page: '1', per_page: '100' });
      if (this._searchQuery) params.set('search', this._searchQuery);
      if (this._category) params.set('category', this._category);
      if (this._protocol) params.set('protocol', this._protocol);

      const res = await fetch(`/v1/templates?${params}`);
      const data = await res.json();

      if (data.code === 0 && data.result) {
        this._templates = data.result.items;
        templates.set(this._templates);
      }
    } catch (e) {
      console.error('Failed to load templates:', e);
    } finally {
      this._loading = false;
    }
  }

  private _handleSearch(e: CustomEvent<{ query: string }>) {
    this._searchQuery = e.detail.query;
    this.loadTemplates();
  }

  private _handleCategoryChange(e: CustomEvent<{ value: string }>) {
    this._category = e.detail.value;
    this.loadTemplates();
  }

  private _handleProtocolChange(e: CustomEvent<{ value: string }>) {
    this._protocol = e.detail.value;
    this.loadTemplates();
  }

  render() {
    return html`
      <div class="filters">
        <div class="filter-group">
          <search-box
            @search=${this._handleSearch}
            placeholder="搜索模板..."
          ></search-box>
          <category-filter @change=${this._handleCategoryChange}></category-filter>
          <protocol-filter @change=${this._handleProtocolChange}></protocol-filter>
        </div>
        <sort-dropdown></sort-dropdown>
      </div>

      ${this._loading ? html`
        <div class="loading">加载中...</div>
      ` : this._templates.length === 0 ? html`
        <div class="empty">未找到模板</div>
      ` : html`
        <div class="grid">
          ${this._templates.map(t => html`
            <template-card .template=${t}></template-card>
          `)}
        </div>
      `}
    `;
  }
}
