import { LitElement, html, css } from 'lit';
import { customElement, property } from 'lit/decorators.js';
import type { TemplateItem } from './template-list';
import './download-button';

@customElement('template-card')
export class TemplateCard extends LitElement {
  static styles = css`
    :host {
      display: block;
    }

    .card {
      background: var(--color-card-bg);
      border: 1px solid var(--color-border);
      border-radius: var(--radius-lg);
      padding: var(--spacing-md);
      transition: box-shadow 0.2s ease, transform 0.2s ease;
      cursor: pointer;
    }

    .card:hover {
      box-shadow: var(--shadow-lg);
      transform: translateY(-2px);
    }

    .header {
      display: flex;
      align-items: flex-start;
      gap: var(--spacing-md);
      margin-bottom: var(--spacing-sm);
    }

    .icon {
      width: 48px;
      height: 48px;
      border-radius: var(--radius-md);
      background: var(--color-tag-bg);
      display: flex;
      align-items: center;
      justify-content: center;
      flex-shrink: 0;
      overflow: hidden;
    }

    .icon img {
      width: 100%;
      height: 100%;
      object-fit: cover;
    }

    .icon-placeholder {
      color: var(--color-primary);
    }

    .info {
      flex: 1;
      min-width: 0;
    }

    .name {
      font-size: 1rem;
      font-weight: 600;
      color: var(--color-text);
      margin-bottom: 2px;
      white-space: nowrap;
      overflow: hidden;
      text-overflow: ellipsis;
    }

    .meta {
      font-size: 0.8125rem;
      color: var(--color-text-secondary);
    }

    .description {
      font-size: 0.875rem;
      color: var(--color-text-secondary);
      line-height: 1.4;
      margin-bottom: var(--spacing-sm);
      display: -webkit-box;
      -webkit-line-clamp: 2;
      -webkit-box-orient: vertical;
      overflow: hidden;
    }

    .tags {
      display: flex;
      flex-wrap: wrap;
      gap: var(--spacing-xs);
      margin-bottom: var(--spacing-md);
    }

    .tag {
      padding: 2px 8px;
      background: var(--color-tag-bg);
      color: var(--color-tag-text);
      border-radius: var(--radius-sm);
      font-size: 0.75rem;
      font-weight: 500;
    }

    .footer {
      display: flex;
      align-items: center;
      justify-content: space-between;
      padding-top: var(--spacing-sm);
      border-top: 1px solid var(--color-border);
    }

    .author {
      font-size: 0.8125rem;
      color: var(--color-text-secondary);
    }

    .actions {
      display: flex;
      gap: var(--spacing-sm);
    }
  `;

  @property({ type: Object }) template!: TemplateItem;

  render() {
    const t = this.template;

    return html`
      <article class="card">
        <div class="header">
          <div class="icon">
            ${t.icon ? html`<img src=${t.icon} alt=${t.name} />` : html`
              <svg class="icon-placeholder" width="24" height="24" viewBox="0 0 20 20" fill="currentColor">
                <path d="M4 4a2 2 0 012-2h8a2 2 0 012 2v12l-5-3-5 3V4z"/>
              </svg>
            `}
          </div>
          <div class="info">
            <div class="name">${t.name}</div>
            <div class="meta">${t.version} · ${t.protocol}</div>
          </div>
        </div>

        <p class="description">${t.description}</p>

        <div class="tags">
          ${t.tags.slice(0, 3).map(tag => html`<span class="tag">${tag}</span>`)}
          ${t.manufacturer ? html`<span class="tag">${t.manufacturer}</span>` : ''}
        </div>

        <div class="footer">
          <span class="author">${t.author_name}</span>
          <div class="actions">
            <download-button .itemId=${t.id} .itemType="template"></download-button>
          </div>
        </div>
      </article>
    `;
  }
}
