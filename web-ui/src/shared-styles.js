import { css } from 'lit';

// Import the CSS to ensure it's loaded, but we won't use it directly
import './css/main.css';

// Return an empty CSS template literal since Tailwind styles are global
export const sharedStyles = css`
  :host {
    display: block;
  }
`;

export default sharedStyles;