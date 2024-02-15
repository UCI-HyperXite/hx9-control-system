import  { useContext } from "react";
import { consoleContext } from "../../App";
import "./controlpanel.css";

export default function ControlPanel() {
  const context = useContext(consoleContext);

  if (!context) {
    // Handle the case when context is undefined
    return <div>Error: Context not available</div>;
  }

  const { setconsole } = context;

  const handleStart = () => {
    setconsole((prev: number[]) => [...prev, 1]);
  };

  const handleStop = () => {
    setconsole((prev: number[]) => [...prev, 0]);
  };

  const handleForceStop = () => {
    setconsole((prev: number[]) => [...prev, 2]);
  };

  const handleLoad = () => {
    setconsole((prev: number[]) => [...prev, 3]);
  };

  return (
    <div id="controlpanel">
      <button id="start" className="button" onClick={handleStart}>
        Start
      </button>
      <button id="stop" className="button" onClick={handleStop}>
        Stop
      </button>
      <button id="force" className="button" onClick={handleForceStop}>
        Force Stop
      </button>
      <button id="load" className="button" onClick={handleLoad}>
        Load
      </button>
    </div>
  );
}
