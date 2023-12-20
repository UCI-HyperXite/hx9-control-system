import "./style.css";

function makeli(prop) {
  switch (prop) {
    case 0:
      return <li className="console-list-item">Stop Sent</li>;
    case 1:
      return <li className="console-list-item">Start Sent</li>;
    case 2:
      return <li className="console-list-item">Load Sent</li>;
  }
}

export default function Console({ consoleList }) {
  return (
    <div id="console">
      <h1>Console</h1>
      <ul className="console-list">
        {consoleList.map((prop) => makeli(prop))}
      </ul>
    </div>
  );
}
