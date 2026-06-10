import { useState } from 'react'
import { useEffect } from 'react';

import './App.css'

function App() {
  const [message, setMessage] = useState('');

  useEffect(() => {
    // Hits http://localhost:5173/api/hello -> Proxied to http://127.0.0.1:3000/api/hello
    fetch('/api/hello')
      .then((res) => res.json())
      .then((data) => setMessage(data.text))
      .catch((err) => console.error(err));
  }, []);

  return (
    <div>
      <h1>Frontend</h1>
      <p>Backend says: {message || "Loading..."}</p>
    </div>
  );
}

export default App
