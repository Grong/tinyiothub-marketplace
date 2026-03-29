import { LitElement, html, css } from 'lit';
import { customElement, property } from 'lit/decorators.js';
import type { DriverItem } from './driver-list';
import './download-button';

@customElement('driver-card')
export class DriverCard extends LitElement {
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

    .tag.protocol {
      background: #f0fdf4;
      color: #166534;
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

  @property({ type: Object }) driver!: DriverItem;

  render() {
    const d = this.driver;

    return html`
      <article class="card">
        <div class="header">
          <div class="icon">
            ${d.icon ? html`<img src=${d.icon} alt=${d.name} />` : html`
              <svg class="icon-placeholder" width="24" height="24" viewBox="0 0 20 20" fill="currentColor">
                <path fill-rule="evenodd" d="M11.3 1.046A1 1 0 0112 2v5h4a1 1 0 01.82 1.573l-7 10A1 1 0 018 18v-5H4a1 1 0 01-.82-1.573l7-10a1 1 0 011.12-.38z" clip-rule="evenodd"/>
              </svg>
            `}
          </div>
          <div class="info">
            <div class="name">${d.name}</div>
            <div class="meta">${d.version} · ${d.protocol}</div>
          </div>
        </div>

        <p class="description">${d.description}</p>

        <div class="tags">
          ${d.tags.slice(0, 3).map(tag => html`<span class="tag">${tag}</span>`)}
          <span class="tag protocol">${d.protocol}</span>
        </div>

        <div class="footer">
          <span class="author">${d.author_name}</span>
          <div class="actions">
            <download-button .itemId=${d.id} .itemType="driver"></download-button>
          </div>
        </div>
      </article>
    `;
  }
}
