import "./style.css";
import { useState } from "react";
import { useSharedState } from "../../../../Data/Info/SensorData";
function makeOption(prop) {
  return <option value={prop.value}>{prop.name}</option>;
}
export default function Card() {
  const [cardvalue, setcardvalue] = useState(0);
  const { powersInfo } = useSharedState();
  function selectchange(event) {
    const value = event.target.value;
    setcardvalue(value);
  }
  return (
    <div id="card" className="extra">
      <select id="selector" onChange={selectchange}>
        {powersInfo.map((prop) => makeOption(prop))}
      </select>
      <h1 id="cardinfo">{cardvalue}</h1>
    </div>
  );
}
