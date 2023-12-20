import { BrowserRouter as Router, Routes, Route } from "react-router-dom";
import Guidata from "./GUI Data/Guidata";
import ConsoleCamera from "./ConsoleCamera/ConsoleCamera";
import Powers from "../IndivPages/Powers";
import { SharedStateProvider } from "../../Data/Info/SensorData";
export default function Homepage({ consoleList }) {
  return (
    <Router>
      <div id="homepage">
        <Routes>
          <Route
            path="/"
            element={
              <>
                <Guidata />
                <ConsoleCamera consoleList={consoleList} />
              </>
            }
          />
          <Route
            path="/powers"
            element={
              <>
                <SharedStateProvider>
                  <Powers />
                </SharedStateProvider>
              </>
            }
          />
        </Routes>
      </div>
    </Router>
  );
}
