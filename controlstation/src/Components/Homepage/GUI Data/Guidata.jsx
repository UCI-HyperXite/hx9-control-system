import Speedometer from "./Components/Speedometer";
import Odometer from "./Components/Odometer";
import Card from "./Components/Card";
import { SharedStateProvider } from "../../../Data/Info/SensorData";
import "./style.css";
export default function Guidata() {
  return (
    <div id="guidata">
      <Speedometer />
      <Odometer />
      <SharedStateProvider>
        <Card />
        <Card />
        <Card />
        <Card />
      </SharedStateProvider>
    </div>
  );
}
