import "./style.css";
import io from "socket.io-client";

export default function Control({ consoleList, setConsoleList }) {
  return (
    <footer id="control">
      <button
        name="start"
        onClick={() => {
          console.log("start");
          setConsoleList(consoleList.concat(1));
        }}
        className="button"
        id="start"
      >
        Start
      </button>
      <button
        name="stop"
        onClick={() => setConsoleList(consoleList.concat(0))}
        className="button"
        id="stop"
      >
        Stop
      </button>
      <button
        name="load"
        onClick={() => setConsoleList(consoleList.concat(2))}
        className="button"
        id="load"
      >
        Load
      </button>
    </footer>
  );
}
