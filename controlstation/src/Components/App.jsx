import "./style.css";
import Navbar from "./Navbar/Navbar";
import Homepage from "./Homepage/Homepage";
import Control from "./Control/Control";
import { useState } from "react";

export default function App() {
  const [consoleList, setConsoleList] = useState([]);
  return (
    <div id="app">
      <Navbar />
      <Homepage consoleList={consoleList} />
      <Control consoleList={consoleList} setConsoleList={setConsoleList} />
    </div>
  );
}
