import Camera from "./Components/Camera";
import Console from "./Components/Console";
import "./style.css";
export default function ConsoleCamera({ consoleList }) {
  return (
    <div id="consolecamera">
      <Console consoleList={consoleList} />
      <Camera />
    </div>
  );
}
