import './index.html';
import './style.global.css';
import ReactDOM from 'react-dom';
import App from './App';

export function LoadApp() {
    ReactDOM.unmountComponentAtNode(document.querySelector('#app'));
    ReactDOM.render(<App />, document.querySelector('#app'));
}

LoadApp();