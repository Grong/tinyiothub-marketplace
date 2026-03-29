import { LitElement, html, css } from 'lit';
import { customElement, property } from 'lit/decorators.js';

@customElement('protocol-filter')
export class ProtocolFilter extends LitElement {
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

  @property({ type: String }) value = '';

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
        <option value="">全部协议</option>
        <option value="modbus">Modbus</option>
        <option value="mqtt">MQTT</option>
        <option value="http">HTTP</option>
        <option value="coap">CoAP</option>
        <option value="opcua">OPC UA</option>
      </select>
    `;
  }
}
