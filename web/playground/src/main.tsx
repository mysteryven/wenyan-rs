import React from 'react'
import ReactDOM from 'react-dom/client'
// @ts-ignore
import * as wsam from 'wenyan-rs'
import App from './App'
import './index.css'

console.log(wsam.greet("123"));

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
)
