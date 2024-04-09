import { useEffect } from 'react'
import './App.css'
import init, { show_graph } from "wasm-graph-view";

function App() {
  useEffect(() => {
    init().then(() => {
      show_graph("#viz");
    }).catch(console.error);
  }, [])

  return (
    <>
      <canvas id="viz"></canvas>
    </>
  )
}

export default App
