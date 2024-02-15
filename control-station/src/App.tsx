import React, { createContext, useState } from "react";
import Navbar from "@Navbar/Navbar";
import SensorBoxContainer from "@SensorBoxes/SensorBoxContainer";
import ControlPanel from "@ControlPanel/ControlPanel";
interface ConsoleContextProps {
  console: number[];
  setconsole: React.Dispatch<React.SetStateAction<number[]>>;
}

export const consoleContext = createContext<ConsoleContextProps | undefined>(undefined);

const App: React.FC = () => {
  const [console, setconsole] = useState<number[]>([]);
  return (
    <consoleContext.Provider value={{ console, setconsole }}>
      <main>
        <Navbar />
        <SensorBoxContainer />
        <ControlPanel />
      </main>
    </consoleContext.Provider>
  );
};

export default App;


