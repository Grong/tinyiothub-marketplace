import { LitElement, html, css } from 'lit';
import { customElement, property } from 'lit/decorators.js';

@customElement('sort-dropdown')
export class SortDropdown extends LitElement {
  static styles = css`
    :host {
      display: inline-block;
    }

    select {
      padding: 6px 12px;
      border: 1px solid var(--color-border);
      border-radius: var(--radius-md);
      font-size: 0.875rem;
      background: white;
      cursor: pointer;
      outline: none;
    }

    select:focus {
      border-color: var(--color-primary);
    }
  `;

  @property({ type: String }) value = 'downloads';

  private _handleChange(e: Event) {
    const select = e.target as HTMLSelectElement;
    this.value = select.value;
    this.dispatchEvent(new CustomEvent('change', {
      detail: { value: this.value },
      bubbles: true,
      composed: true,
    }));
  }

  render() {
    return html`
      <select @change=${this._handleChange} .value=${this.value}>
        <option value="downloads">按下载量</option>
        <option value="rating">按评分</option>
        <option value="name">按名称</option>
        <option value="updated">按更新时间</option>
      </select>
    `;
  }
}
